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

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Extension, Json,
};
use paastel::{CreateAppCommand, Name};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{middleware, state::AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateAppRequest {
    name: String,
}

pub(crate) async fn crete_app(
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
