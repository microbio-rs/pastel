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

use async_trait::async_trait;
use derive_new::new;
use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, core::ObjectList, Api, Error as KError};
use paastel_auth::{OutgoingKubernetesPort, SecretLabel};

/// Reads secrets managed by a rchestrator`].
#[derive(new)]
pub struct KubernetsSecretsAdapter {
    api: Api<Secret>,
}

impl KubernetsSecretsAdapter {
    async fn get_all<P: Into<String>>(
        &self,
        labels: P,
    ) -> StdResult<ObjectList<Secret>, KError> {
        let lp = ListParams::default().match_any().labels(&labels.into());
        self.api.list(&lp).await
    }
}

#[async_trait]
impl OutgoingKubernetesPort for KubernetsSecretsAdapter {
    async fn find_secrets_by_label(
        &self,
        _label: &SecretLabel,
    ) -> paastel_auth::Result<paastel_auth::UserSecrets> {
        // let secrets_list = self
        //     .get_all(label)
        //     .await
        //     .map_err(|e| paastel_auth::Error::SecretNotFound)?;
        todo!()
    }
}
