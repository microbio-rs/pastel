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

use std::result::Result as StdResult;

use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, core::ObjectList, Api, Error as KError};

use crate::client::KubernetesClient;

/// Reads secrets managed.
#[derive(Clone)]
pub(crate) struct KubernetsSecretsAdapter {
    api: Api<Secret>,
}

impl KubernetsSecretsAdapter {
    pub fn new(client: &KubernetesClient) -> Self {
        Self {
            api: Api::default_namespaced(client.as_ref().clone()),
        }
    }
}

impl KubernetsSecretsAdapter {
    pub(crate) async fn get_all(
        &self,
        list_params: &ListParams,
    ) -> StdResult<ObjectList<Secret>, KError> {
        self.api.list(&list_params).await
    }
}
