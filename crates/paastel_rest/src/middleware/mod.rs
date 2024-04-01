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
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
use base64::Engine;
use paastel::BaseAuthCommand;

use crate::state::AppState;

#[derive(Debug, Clone)]
pub(crate) struct CurrentUser {
    pub(crate) username: String,
}

pub(crate) async fn auth(
    State(AppState { auth_usecase, .. }): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let split = auth_header.split_once(' ');
    match split {
        Some(("Basic", contents)) => {
            let decoded = decode(contents).unwrap();
            let command =
                BaseAuthCommand::new(decoded.0.into(), decoded.1.unwrap());
            let auth_user = auth_usecase.basic_auth(&command).await.unwrap();
            let current_user = CurrentUser {
                username: auth_user.username.0.to_string(),
            };
            req.extensions_mut().insert(current_user);
            Ok(next.run(req).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Decodes the two parts of basic auth using the colon
fn decode(input: &str) -> Result<(String, Option<String>), ()> {
    // Decode from base64 into a string
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(input)
        .unwrap();
    // .map_err(|_| err)?;
    let decoded = String::from_utf8(decoded).unwrap();

    // Return depending on if password is present
    Ok(if let Some((id, password)) = decoded.split_once(':') {
        (id.to_string(), Some(password.to_string()))
    } else {
        (decoded.to_string(), None)
    })
}
