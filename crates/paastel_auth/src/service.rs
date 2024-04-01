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

use std::sync::Arc;

use async_trait::async_trait;
use derive_new::new;
use tracing::info;
use tracing::warn;

use crate::Error;
use crate::KubeSecretPort;
use crate::Password;
use crate::PasswordHashPort;
use crate::UserSecret;
use crate::{CheckCredentialUseCase, Credential};

/// # AuthService
///
/// This service implement use cases from authentication
#[derive(new)]
pub struct AuthService<P> {
    kube_port: Box<dyn KubeSecretPort + Send + Sync>,
    password_port: Box<dyn PasswordHashPort<P> + Send + Sync>,
}

pub type ArcLoginUseCase = Arc<dyn CheckCredentialUseCase + Send + Sync>;

#[async_trait]
impl CheckCredentialUseCase for AuthService<Password> {
    async fn check_credential(
        &self,
        credential: &Credential,
    ) -> crate::Result<UserSecret> {
        let username = credential.username();

        info!("Login to your PaaStel cluster with [{}]", username);

        // 1. call port kubernetes secrets for get secret credential
        // NOTE: User has multiples secrets
        let secrets = self.kube_port.find_secrets_by_username(username).await?;
        let user_secret_found =
            secrets.iter().find(|us| us.username() == username);

        if let Some(usf) = user_secret_found {
            // 2. call port hash password to verify password
            let p_text = credential.password_text();
            let p_hash = usf.password_hashed();
            self.password_port.check_password(p_text, p_hash).await?;

            info!("Login succesfull");
            Ok(usf.clone())
        } else {
            warn!("Username [{}] secret not found", username);
            Err(Error::SecretNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::{
        MockKubeSecretPort, MockPasswordHashPort, Password, UserSecret,
        UserSecrets, Username,
    };

    use super::*;

    fn new_kube_port(
        username: &'static str,
        password_hashed: &'static str,
    ) -> crate::Result<impl KubeSecretPort> {
        let mut kube_port = MockKubeSecretPort::new();
        kube_port
            .expect_find_secrets_by_username()
            .with(eq(username.parse::<Username>()?))
            .times(1)
            .returning(move |_| {
                Ok(UserSecrets::new(vec![UserSecret::new(
                    username.parse()?,
                    password_hashed.parse()?,
                )]))
            });
        Ok(kube_port)
    }

    fn new_password_port(
        password_text: &'static str,
        password_hashed: &'static str,
    ) -> crate::Result<impl PasswordHashPort<Password>> {
        let mut password_port = MockPasswordHashPort::new();
        password_port
            .expect_check_password()
            .with(
                eq(password_text.parse::<Password>()?),
                eq(password_hashed.parse::<Password>()?),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        Ok(password_port)
    }

    #[tokio::test]
    async fn auth_service_ok() -> crate::Result<()> {
        let kube_port = new_kube_port("username", "password_hashed")?;
        let password_port =
            new_password_port("password_text", "password_hashed")?;

        let credential = Credential::new("username", "password_text")?;
        let auth_service =
            AuthService::new(Box::new(kube_port), Box::new(password_port));
        let result = auth_service.check_credential(&credential).await;
        assert!(result.is_ok());

        Ok(())
    }
}
