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

use async_trait::async_trait;
use derive_new::new;
use tracing::info;

use crate::KubeSecretPort;
use crate::Password;
use crate::PasswordHashPort;
use crate::{Credential, LoginUseCase};

/// # AuthService
///
/// This service implement use cases from authentication
#[derive(new)]
pub struct AuthService<P> {
    kube_port: Box<dyn KubeSecretPort + Send + Sync>,
    password_port: Box<dyn PasswordHashPort<P> + Send + Sync>,
}

#[async_trait]
impl LoginUseCase for AuthService<Password> {
    async fn login(&self, credential: &Credential) -> crate::Result<()> {
        info!(
            "Login to your PaaStel cluster with [{}]",
            credential.username()
        );

        // 1. TODO: validate credential
        let username = credential.username();

        // 2. call port kubernetes secrets for get secret credential
        let user_secret = self.kube_port.get_secret(username).await?;

        // 3. call port hash password to verify password
        let cred_password = credential.password(); // password text
        let user_password = user_secret.password_hashed(); // password hashed
        self.password_port
            .check_password(cred_password, user_password)
            .await?;

        info!("Login succesfull");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::{
        MockKubeSecretPort, MockPasswordHashPort, Password, UserSecret,
        Username,
    };

    use super::*;

    #[tokio::test]
    async fn auth_service_ok() {
        let mut kube_port = MockKubeSecretPort::new();
        kube_port
            .expect_get_secret()
            .with(eq(Username::new("username")))
            .times(1)
            .returning(move |_| {
                Ok(UserSecret::new(
                    Username::new("username"),
                    Password::new("password_hashed"),
                ))
            });

        let mut password_port = MockPasswordHashPort::new();
        password_port
            .expect_check_password()
            .with(
                eq(Password::new("password_text")),
                eq(Password::new("password_hashed")),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let credential = Credential::new("username", "password_text");
        let auth_service =
            AuthService::new(Box::new(kube_port), Box::new(password_port));
        let result = auth_service.login(&credential).await;
        assert!(result.is_ok());
    }
}
