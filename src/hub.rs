//! Shared state between the collector thread and HTTP handlers.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::sync::broadcast;

use crate::collect::{Collector, Point, Sample};

pub struct Hub {
    history: Mutex<VecDeque<Point>>,
    capacity: usize,
    /// Latest sample, serialized once and shared by every client.
    latest: Mutex<Option<Arc<str>>>,
    tx: broadcast::Sender<Arc<str>>,
}

impl Hub {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(16);
        Self {
            history: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
            latest: Mutex::new(None),
            tx,
        }
    }

    fn publish(&self, sample: &Sample) {
        let json: Arc<str> = match serde_json::to_string(sample) {
            Ok(s) => s.into(),
            Err(e) => {
                eprintln!("picomon: serializing sample: {e}");
                return;
            }
        };
        if self.capacity > 0 {
            let mut h = self.history.lock().unwrap();
            if h.len() == self.capacity {
                h.pop_front();
            }
            h.push_back(sample.point());
        }
        *self.latest.lock().unwrap() = Some(json.clone());
        // An error only means there are no subscribers.
        let _ = self.tx.send(json);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<str>> {
        self.tx.subscribe()
    }

    pub fn latest(&self) -> Option<Arc<str>> {
        self.latest.lock().unwrap().clone()
    }

    pub fn history_json(&self) -> String {
        let h = self.history.lock().unwrap();
        let (a, b) = h.as_slices();
        let mut v = Vec::with_capacity(h.len());
        v.extend_from_slice(a);
        v.extend_from_slice(b);
        drop(h);
        serde_json::to_string(&v).unwrap_or_else(|_| "[]".into())
    }
}

/// Runs the collector on a dedicated OS thread so slow filesystem calls
/// (e.g. statvfs on a stalled mount) never block the HTTP server.
pub fn spawn_collector(hub: Arc<Hub>, mut collector: Collector, interval: Duration) {
    std::thread::Builder::new()
        .name("collector".into())
        .spawn(move || {
            let mut next = Instant::now() + interval;
            loop {
                std::thread::sleep(next.saturating_duration_since(Instant::now()));
                hub.publish(&collector.sample());
                next += interval;
                // Skip missed ticks after a stall instead of bursting to catch up.
                let now = Instant::now();
                if next < now {
                    next = now + interval;
                }
            }
        })
        .expect("spawning collector thread");
}
