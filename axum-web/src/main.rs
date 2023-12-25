use axum::{
  body::Bytes,
  extract::MatchedPath,
  http::{HeaderMap, Request},
  response::{Html, Response},
  routing::get,
  Router,
};
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};

use tracing::log::debug;
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // axum logs rejections from built-in extractors with the `axum::rejection`
        // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
        "axum_web=debug,tower_http=debug,axum::rejection=trace".into()
      }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  // tracing_subscriber::fmt()
  //   .with_max_level(tracing::Level::DEBUG)
  //   // disable log color in production mode to facilitate log monitoring systems (e.g. ELK)
  //   .with_line_number(true)
  //   .with_test_writer()
  //   .init();

  // build our application with a route
  let app = Router::new()
    .route("/", get(handler))
    // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
    // It provides good defaults but is also very customizable.
    //
    // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
    //
    // If you want to customize the behavior using closures here is how.
    .layer(
      TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
          // Log the matched route's path (with placeholders not filled in).
          // Use request.uri() or OriginalUri if you want the real path.
          let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);
          // 获取请求头中的 "x-request-id" 字段作为 trace ID
          let mut trace_id = request
            .headers()
            .get("x-request-id")
            .map(|id| {
              debug!("id: {:?}", id);
              id.to_str().unwrap_or("111")
            })
            .map(|_| "222");
          trace_id = Some("333");
          debug!("request: {:?}", request);
          debug!("trace_id: {:?}", trace_id);

          info_span!(
                      "http_request",
                      method = ?request.method(),
                      matched_path,
                      some_other_field = tracing::field::Empty,
          trace_id = trace_id
                  )
        })
        .on_request(|_request: &Request<_>, _span: &Span| {
          _span.record("some_other_field", "111");
          // You can use `_span.record("some_other_field", value)` in one of these
          // closures to attach a value to the initially empty field in the info_span
          // created above.
        })
        .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
          _span.record("some_other_field", "112");
          // ...
        })
        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
          // ...
        })
        .on_eos(
          |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
            // ...
          },
        )
        .on_failure(|_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
          // ...
        }),
    );

  // run it
  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
  tracing::debug!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
  debug!("request received");
  Html("<h1>Hello, World!</h1>")
}
