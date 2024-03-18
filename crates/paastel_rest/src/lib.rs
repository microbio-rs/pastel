// Copyright (c) 2024 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

pub mod error;
pub(crate) mod prometheus;
pub(crate) mod utils;

use std::time::Duration;

use axum::middleware;
use axum::response::IntoResponse;
use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

async fn start_main_server() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/healthz", get(healthz))
        .route_layer(middleware::from_fn(prometheus::track_metrics))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!(
        "listening rest server on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(utils::shutdown_signal())
        .await
        .unwrap();
}

pub async fn serve() -> Result<(), error::Error> {
    let (_main_server, _metrics_server) =
        tokio::join!(start_main_server(), prometheus::start_metrics_server());
    Ok(())
}

async fn handler() -> impl IntoResponse {
    Html("<h1>Hello, PaaStel!</h1>")
}

async fn healthz() -> impl IntoResponse {
    "ok"
}
