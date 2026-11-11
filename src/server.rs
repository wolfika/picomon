//! HTTP server: embedded frontend, SSE stream and basic auth.

use std::convert::Infallible;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{HeaderValue, StatusCode, Uri, header};
use axum::middleware::{self, Next};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use base64::Engine;
use rust_embed::RustEmbed;
use serde::Serialize;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt};

use crate::collect::HostInfo;
use crate::config::Config;
use crate::hub::Hub;

#[derive(RustEmbed)]
#[folder = "web/dist"]
#[allow_missing = true]
struct Assets;

const CSP: &str = "default-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; \
                   object-src 'none'; base-uri 'none'; frame-ancestors 'none'";

#[derive(Clone)]
struct App {
    hub: Arc<Hub>,
    /// Static part of the SSE `init` event.
    meta: Arc<str>,
    index: Option<Arc<str>>,
}

#[derive(Serialize)]
struct Meta<'a> {
    title: &'a str,
    host: &'a HostInfo,
    /// Seconds between samples.
    interval: f64,
    /// Number of history points kept.
    history_len: usize,
}

pub fn router(cfg: &Config, hub: Arc<Hub>, host: &HostInfo) -> Router {
    let title = cfg.server.title.as_deref().unwrap_or(&host.hostname);
    let meta = serde_json::to_string(&Meta {
        title,
        host,
        interval: cfg.collector.interval,
        history_len: cfg.collector.history_len(),
    })
    .expect("serializing meta");

    let app = App { hub, meta: meta.into(), index: render_index(cfg, title).map(Into::into) };

    let mut router = Router::new()
        .route("/api/stream", get(stream))
        .route("/api/sample", get(sample))
        .fallback(get(static_file))
        .with_state(app);

    if let (Some(user), Some(pass)) = (&cfg.auth.username, &cfg.auth.password) {
        let expected = format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(format!("{user}:{pass}"))
        );
        router = router.layer(middleware::from_fn_with_state(Arc::<str>::from(expected), basic_auth));
    }
    router
}

/// Injects the configured theme and title into the built index.html.
fn render_index(cfg: &Config, title: &str) -> Option<String> {
    let file = Assets::get("index.html")?;
    let html = String::from_utf8_lossy(&file.data);
    let mut style = String::new();
    if !cfg.theme.colors.is_empty() {
        style.push_str("<style>:root:root{");
        for (k, v) in &cfg.theme.colors {
            style.push_str(&format!("--{k}:{v};"));
        }
        style.push_str("}</style>");
    }
    Some(
        html.replacen("data-theme=\"glass\"", &format!("data-theme=\"{}\"", cfg.theme.preset), 1)
            .replacen("<title>Picomon</title>", &format!("<title>{}</title>{style}", escape_html(title)), 1),
    )
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

async fn stream(State(app): State<App>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Subscribe before taking the snapshot so no sample falls in between.
    // A sample may then appear twice; the client drops samples it already has.
    let rx = app.hub.subscribe();
    let latest = app.hub.latest();
    let init = format!(
        r#"{{"meta":{},"history":{},"sample":{}}}"#,
        app.meta,
        app.hub.history_json(),
        latest.as_deref().unwrap_or("null"),
    );
    let first = tokio_stream::once(Ok(Event::default().event("init").data(init)));
    // Lagged receivers skip the missed samples.
    let rest = BroadcastStream::new(rx)
        .filter_map(|r| r.ok().map(|json| Ok(Event::default().event("sample").data(&*json))));
    Sse::new(first.chain(rest)).keep_alive(KeepAlive::default())
}

async fn sample(State(app): State<App>) -> Response {
    match app.hub.latest() {
        Some(json) => ([(header::CONTENT_TYPE, "application/json")], json.to_string()).into_response(),
        None => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

async fn static_file(State(app): State<App>, uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    if path.is_empty() || path == "index.html" {
        return match app.index {
            Some(html) => (
                [
                    (header::CONTENT_TYPE, "text/html; charset=utf-8"),
                    (header::CACHE_CONTROL, "no-cache"),
                    (header::CONTENT_SECURITY_POLICY, CSP),
                    (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
                ],
                html.to_string(),
            )
                .into_response(),
            None => (
                StatusCode::NOT_FOUND,
                "Frontend not built. Run `make web` (or `cd web && bun run build`) and rebuild.",
            )
                .into_response(),
        };
    }
    let Some(file) = Assets::get(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    // Vite puts content-hashed files under assets/.
    let cache = if path.starts_with("assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    };
    (
        [
            (header::CONTENT_TYPE, mime.as_ref()),
            (header::CACHE_CONTROL, cache),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        ],
        Body::from(file.data),
    )
        .into_response()
}

async fn basic_auth(State(expected): State<Arc<str>>, req: Request, next: Next) -> Response {
    let ok = req
        .headers()
        .get(header::AUTHORIZATION)
        .is_some_and(|v| constant_time_eq(v.as_bytes(), expected.as_bytes()));
    if ok {
        return next.run(req).await;
    }
    let mut res = StatusCode::UNAUTHORIZED.into_response();
    res.headers_mut()
        .insert(header::WWW_AUTHENTICATE, HeaderValue::from_static("Basic realm=\"picomon\", charset=\"UTF-8\""));
    res
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ct_eq() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"ab"));
    }

    #[test]
    fn escapes_title() {
        assert_eq!(escape_html("<a&\"b>"), "&lt;a&amp;&quot;b&gt;");
    }
}
