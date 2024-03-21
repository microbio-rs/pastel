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

use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, Api, Client};

const SECRET_LABEL_KEY: &str = "paastel.io/api-user-credentials";
const SECRET_LABEL_VALUE: &str = "true";

#[derive(Debug, thiserror::Error)]
pub enum Error {}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default)]
pub struct UserSecret {
    pub username: String,
    pub password: String,
    pub secret_name: String,
}

pub type UserSecrets = Vec<UserSecret>;

#[async_trait]
pub trait Secrets {
    async fn list_user_secrets(&self) -> Result<UserSecrets>;
}

#[derive(Debug)]
pub struct KubeSecrets {
    secrets: Api<Secret>,
}

impl KubeSecrets {
    pub fn new(client: Client) -> Self {
        Self {
            secrets: Api::default_namespaced(client),
        }
    }
}

#[async_trait]
impl Secrets for KubeSecrets {
    async fn list_user_secrets(&self) -> Result<UserSecrets> {
        let lp = ListParams::default()
            .match_any()
            .timeout(60)
            .labels(&format!("{}={}", SECRET_LABEL_KEY, SECRET_LABEL_VALUE));
        let secrets: UserSecrets = self
            .secrets
            .list(&lp)
            .await
            .unwrap()
            .iter()
            .map(UserSecret::from)
            .collect();

        Ok(secrets)
    }
}

impl From<&Secret> for UserSecret {
    fn from(value: &Secret) -> Self {
        let mut user_secret = UserSecret::default();
        let data = value.data.as_ref();
        user_secret.username =
            String::from_utf8(data.unwrap().get("username").unwrap().0.clone())
                .unwrap();
        user_secret.password =
            String::from_utf8(data.unwrap().get("password").unwrap().0.clone())
                .unwrap();
        user_secret.secret_name = value.metadata.name.clone().unwrap();
        user_secret
    }
}
