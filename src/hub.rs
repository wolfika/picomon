//! Shared state between the collector thread and HTTP handlers.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::sync::broadcast;

use crate::collect::{Collector, Point, Sample};

pub struct Hub {
    state: Mutex<State>,
    capacity: usize,
    tx: broadcast::Sender<Arc<str>>,
}

/// History and latest sample share one lock, so a snapshot always has a
/// history that ends with the latest sample.
struct State {
    history: VecDeque<Point>,
    /// Latest sample, serialized once and shared by every client.
    latest: Option<Arc<str>>,
}

impl Hub {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(16);
        Self {
            state: Mutex::new(State {
                history: VecDeque::with_capacity(capacity),
                latest: None,
            }),
            capacity,
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
        {
            let mut st = self.state.lock().unwrap();
            if self.capacity > 0 {
                if st.history.len() == self.capacity {
                    st.history.pop_front();
                }
                st.history.push_back(sample.point());
            }
            st.latest = Some(json.clone());
        }
        // An error only means there are no subscribers.
        let _ = self.tx.send(json);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<str>> {
        self.tx.subscribe()
    }

    pub fn latest(&self) -> Option<Arc<str>> {
        self.state.lock().unwrap().latest.clone()
    }

    /// The history as a JSON array and the latest sample, taken together.
    pub fn snapshot(&self) -> (String, Option<Arc<str>>) {
        let st = self.state.lock().unwrap();
        let points: Vec<Point> = st.history.iter().copied().collect();
        let latest = st.latest.clone();
        drop(st);
        (
            serde_json::to_string(&points).unwrap_or_else(|_| "[]".into()),
            latest,
        )
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
