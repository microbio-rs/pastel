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

use axum::extract::{Multipart, Path, State};
use axum::http::HeaderName;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{response::Html, routing::get, Router};
use axum::{Extension, Json};
use paastel::{AppService, AuthService, CreateAppCommand, Name};
use paastel_hash::Argon2Adapter;
use paastel_kube::KubernetesAdapter;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::state::AppState;
use crate::utils;
use crate::{middleware, prometheus};

pub(crate) async fn start_main_server() {
    let hash_port = Argon2Adapter::default();
    let kube_port = KubernetesAdapter::default().await;
    let auth_usecase =
        AuthService::new(Box::new(kube_port.clone()), Box::new(hash_port));
    let create_app_usecase = AppService::new(Box::new(kube_port));
    let app_state =
        AppState::new(Arc::new(create_app_usecase), Arc::new(auth_usecase));
    let app = Router::new()
        .route("/", get(handler))
        .route("/healthz", get(healthz))
        .route("/me", get(me))
        .route("/namespaces/:namespace/applications", post(crete_app))
        .route(
            "/namespaces/:namespace/applications/:app/store",
            post(upload_app),
        )
        .route_layer(axum::middleware::from_fn(prometheus::track_metrics))
        .route_layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth,
        ))
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(20)),
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
    info!("requesting me");
    let username = current_user.username;
    Html(format!("<h1>Hello, {username}</h1>"))
}

#[derive(Serialize, Deserialize)]
pub struct CreateAppRequest {
    name: String,
}

async fn crete_app(
    State(AppState {
        create_app_usecase, ..
    }): State<AppState>,
    Extension(current_user): Extension<middleware::CurrentUser>,
    Path(namespace): Path<String>,
    Json(CreateAppRequest { name }): Json<CreateAppRequest>,
) -> impl IntoResponse {
    info!("requesting creating app");

    let command =
        CreateAppCommand::new(Name::new(name.clone()), namespace.clone());
    create_app_usecase.create(&command).await.unwrap();

    Html(format!(
        "<h1>Hello, {current_user:?} create {name} on {namespace}</h1>"
    ))
}

async fn upload_app(
    State(AppState {
        create_app_usecase: _,
        ..
    }): State<AppState>,
    Extension(current_user): Extension<middleware::CurrentUser>,
    Path((namespace, app)): Path<(String, String)>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    info!("requesting uploading app");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );
    }

    Html(format!(
        "<h1>Hello, {current_user:?} upload {app} on {namespace}</h1>"
    ))
}
