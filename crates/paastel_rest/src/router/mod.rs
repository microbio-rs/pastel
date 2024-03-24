// Copyright (c) 2024 Murilo Ijanc' <mbsd@m0x.ru>

// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::time::Duration;

use axum::{http::HeaderName, response::IntoResponse, routing::get, Router};
use tower_http::{
    limit::RequestBodyLimitLayer,
    propagate_header::PropagateHeaderLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{prometheus, state::AppState};

pub(crate) mod v1;

const MAX_PUBLISH_CONTENT_LENGTH: usize = 128 * 1024 * 1024; // 128 MB

pub(crate) fn make_app(state: AppState) -> Router<()> {
    let v1_route = v1::make_route(state.clone());
    let ver_route: Router<AppState> = Router::new()
        .nest("/v1", v1_route)
        .with_state(state.clone());

    Router::new()
        .route("/healthz", get(healthz))
        .nest("/api", ver_route)
        .route_layer(axum::middleware::from_fn(prometheus::track_metrics))
        .layer(RequestBodyLimitLayer::new(MAX_PUBLISH_CONTENT_LENGTH))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(20)),
        ))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .with_state(state)
}

async fn healthz() -> impl IntoResponse {
    "ok"
}
