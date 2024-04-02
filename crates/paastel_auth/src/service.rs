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

use crate::{
    Credential, Error, OutgoingKubernetesSecretPort, Password,
    PasswordHashPort, SecretLabel, UserSecret, ValidateCredentialUseCase,
};

/// # AuthService
///
/// This service implement use cases from authentication
#[derive(new)]
pub struct AuthService {
    kubernetes_port: Box<dyn OutgoingKubernetesSecretPort + Send + Sync>,
    password_port: Box<dyn PasswordHashPort<Password> + Send + Sync>,
}

pub type ArcValidateCredentialUseCase =
    Arc<dyn ValidateCredentialUseCase + Send + Sync>;

#[async_trait]
impl ValidateCredentialUseCase for AuthService {
    async fn validate_credential(
        &self,
        credential: &Credential,
    ) -> crate::Result<UserSecret> {
        let username = credential.username();

        tracing::info!("validate `{username}` credential on PaaStel cluster");

        // find secrets using default label paastel.io/api-user-credentials
        let label = SecretLabel::default();
        let secrets =
            self.kubernetes_port.find_secrets_by_label(&label).await?;

        // filter secrets by credential username
        let user_secret = secrets.iter().find(|us| us.username() == username);

        match user_secret {
            Some(us) => {
                self.password_port
                    .check(credential.password(), us.password())
                    .await?;
                // self.password_port.check_password(credential, user_secret).await?;

                Ok(us.clone())
            }
            None => {
                tracing::error!(
                    "username `{username}` not found on kubernetes secret"
                );
                Err(Error::SecretNotFound)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::{
        AuthService, Credential, MockOutgoingKubernetesSecretPort,
        MockPasswordHashPort, OutgoingKubernetesSecretPort, Password,
        PasswordHashPort, SecretLabel, UserSecret, UserSecrets,
        ValidateCredentialUseCase,
    };

    fn new_kube_port(
        label: SecretLabel,
        password_hashed: &'static str,
    ) -> crate::Result<impl OutgoingKubernetesSecretPort> {
        let mut kube_port = MockOutgoingKubernetesSecretPort::new();
        kube_port
            .expect_find_secrets_by_label()
            .with(eq(label))
            .times(1)
            .returning(move |_| {
                Ok(UserSecrets::new(vec![UserSecret::new(
                    "username".parse()?,
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
            .expect_check()
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
        let kube_port =
            new_kube_port(SecretLabel::default(), "password_hashed")?;
        let password_port =
            new_password_port("password_text", "password_hashed")?;

        let credential = Credential::new("username", "password_text")?;
        let auth_service =
            AuthService::new(Box::new(kube_port), Box::new(password_port));
        let result = auth_service.validate_credential(&credential).await;
        assert!(result.is_ok());

        Ok(())
    }
}
