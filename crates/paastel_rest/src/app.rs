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

use std::sync::Arc;
use std::time::Duration;

use axum::http::HeaderName;
use axum::response::IntoResponse;
use axum::Extension;
use axum::{response::Html, routing::get, Router};
use paastel::AuthService;
use paastel_hash::Argon2Adapter;
use paastel_kube::KubernetesAdapter;
use tokio::net::TcpListener;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::state::AppState;
use crate::utils;
use crate::{middleware, prometheus};

pub(crate) async fn start_main_server() {
    let hash_port = Argon2Adapter::default();
    let kube_port = KubernetesAdapter::default().await;
    let auth_usecase =
        AuthService::new(Box::new(kube_port), Box::new(hash_port));
    let app_state = AppState::new(Arc::new(auth_usecase));
    let app = Router::new()
        .route("/", get(handler))
        .route("/healthz", get(healthz))
        .route("/me", get(me))
        .route_layer(axum::middleware::from_fn(prometheus::track_metrics))
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth,
        ))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .with_state(app_state);

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

async fn handler() -> impl IntoResponse {
    Html("<h1>Hello, PaaStel!</h1>")
}

async fn healthz() -> impl IntoResponse {
    "ok"
}

async fn me(
    Extension(current_user): Extension<middleware::CurrentUser>,
) -> impl IntoResponse {
    let username = current_user.username;
    Html(format!("<h1>Hello, {username}</h1>"))
}
