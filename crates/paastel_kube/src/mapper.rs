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

use std::collections::BTreeMap;

use derive_new::new;
use k8s_openapi::{api::core::v1::Secret, ByteString};
use kube::{api::ListParams, core::ObjectList};

use paastel_auth::{SecretLabel, UserSecret, UserSecrets};

/// Secret field username
const SECRET_FIELD_USERNAME: &str = "username";

/// Secret field password
const SECRET_FIELD_PASSWORD: &str = "password";

#[derive(Default, Clone, new)]
pub struct KubernetesMapper {}

impl KubernetesMapper {
    pub fn list_secrets_to_domain(
        &self,
        secrets_list: &ObjectList<Secret>,
    ) -> UserSecrets {
        let content: Vec<UserSecret> = secrets_list
            .iter()
            .filter_map(check_secret_data)
            .filter_map(check_secret_content)
            .collect();
        UserSecrets::new(content)
    }

    pub fn from_label_to_lp(&self, _label: &SecretLabel) -> ListParams {
        todo!()
    }
}

fn check_secret_content(
    (_metadata, data): (&String, &BTreeMap<String, ByteString>),
) -> Option<UserSecret> {
    if data.get(SECRET_FIELD_USERNAME).is_none()
        || data.get(SECRET_FIELD_PASSWORD).is_none()
    {
        tracing::info!("not found username or password on secret ...");
        None
    } else {
        tracing::debug!("found username and password on secret ...");
        Some(
            UserSecret::new(
                "dkjksjkj",
                "dkjksjkj", // data.get("username").unwrap(),
                           // data.get("password").unwrap(),
            )
            .unwrap(),
        )
    }
}

fn check_secret_data(
    secret: &Secret,
) -> Option<(&String, &BTreeMap<String, ByteString>)> {
    if secret.data.is_some() && secret.metadata.name.is_some() {
        Some((
            secret.metadata.name.as_ref().unwrap(),
            secret.data.as_ref().unwrap(),
        ))
    } else {
        tracing::info!("not found data on secret ...");
        None
    }
}
