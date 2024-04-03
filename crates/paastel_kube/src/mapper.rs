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

use derive_new::new;
use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, core::ObjectList};

use paastel_auth::{SecretLabel, UserSecret, UserSecrets};

#[derive(Default, Clone, new)]
pub struct KubernetesMapper {}

impl KubernetesMapper {
    pub fn from_label_to_lp(&self, _label: &SecretLabel) -> ListParams {
        todo!()
    }

    pub fn from_list_secrets_to_domain(
        &self,
        secrets_list: &ObjectList<Secret>,
    ) -> UserSecrets {
        let content: Vec<UserSecret> = secrets_list
            .iter()
            .filter_map(|secret| {
                if secret.data.is_some() {
                    Some((&secret.metadata, secret.data.as_ref().unwrap()))
                } else {
                    tracing::info!("not found data on secret ...");
                    None
                }
            })
            .filter_map(|(_metadata, data)| {
                if data.get("username").is_none()
                    || data.get("password").is_none()
                {
                    tracing::info!(
                        "not found username or password on secret ..."
                    );
                    None
                } else {
                    tracing::debug!(
                        "found username and password on secret ..."
                    );
                    Some(UserSecret::new(
                        "dkjksjkj".parse().unwrap(),
                        "dkjksjkj".parse().unwrap(), // data.get("username").unwrap(),
                                                     // data.get("password").unwrap(),
                    ))
                }
            })
            .collect();
        UserSecrets::new(content)
    }
}
