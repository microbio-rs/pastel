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

use std::collections::HashMap;

use async_trait::async_trait;
use derive_new::new;
use mockall::predicate::*;
use mockall::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not found user")]
    AuthUserNotFound,
}
pub type Result<T> = std::result::Result<T, Error>;

///////////////////////////////////////////////////////////////////////////////
// Auth
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, new, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username(pub String);

impl From<String> for Username {
    fn from(value: String) -> Self {
        Username(value)
    }
}

impl From<&str> for Username {
    fn from(value: &str) -> Self {
        Username(value.to_string())
    }
}

#[derive(Debug, new, Clone)]
pub struct AuthUser {
    pub username: Username,
    pub password: String,
    pub secret_name: String,
}

#[derive(Debug, new)]
pub struct AuthUsers(pub HashMap<Username, AuthUser>);

impl AuthUsers {
    pub fn get_by_username(&self, username: &Username) -> Option<&AuthUser> {
        self.0.get(username)
    }

    pub fn insert(&mut self, auth_user: AuthUser) {
        self.0.insert(auth_user.username.clone(), auth_user);
    }
}

#[derive(Debug, new)]
pub struct BaseAuthCommand {
    pub username: Username,
    pub password: String,
}

#[automock]
#[async_trait]
pub trait AuthKubeSecretPort {
    async fn list(&self) -> Result<AuthUsers>;
}

#[automock]
pub trait AuthHashPasswordPort {
    fn verify(&self, password: &str, password_hash: &str) -> Result<()>;
}

#[automock]
#[async_trait]
pub trait AuthUseCase {
    async fn basic_auth(&self, command: &BaseAuthCommand) -> Result<AuthUser>;
}

#[derive(new)]
pub struct AuthService {
    kube_secret_port: Box<dyn AuthKubeSecretPort + Send + Sync>,
    hash_password_port: Box<dyn AuthHashPasswordPort + Send + Sync>,
}

#[async_trait]
impl AuthUseCase for AuthService {
    async fn basic_auth(&self, command: &BaseAuthCommand) -> Result<AuthUser> {
        let secrets = self.kube_secret_port.list().await?;
        let auth_user = secrets.get_by_username(&command.username);
        if auth_user.is_none() {
            return Err(Error::AuthUserNotFound);
        }
        let auth_user = auth_user.unwrap();
        self.hash_password_port
            .verify(&command.password, &auth_user.password)?;
        Ok(auth_user.clone())
    }
}

///////////////////////////////////////////////////////////////////////////////
// Auth End
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn auth_service() {
        let secret_port = MockAuthKubeSecretPort::new()
            .expect_list()
            // .times(1)
            .returning(|| {
                let mut secrets = HashMap::new();
                secrets.insert(
                    "admin".into(),
                    AuthUser::new(
                        "admin".into(),
                        "password".to_string(),
                        "secret_name".to_string(),
                    ),
                );
                let auth_users: AuthUsers = AuthUsers::new(secrets);
                Ok(auth_users)
            });
        let hash_password_port = MockAuthHashPasswordPort::new()
            .expect_verify()
            .with(eq("password"), eq("password"))
            // .times(1)
            .returning(|_, _| Ok(()));

        let secret_port = Box::new(secret_port);
        let hash_password_port = Box::new(hash_password_port);

        let auth_service = AuthService::new(secret_port, hash_password_port);
        let auth_user_command =
            BaseAuthCommand::new("admin".into(), "password".to_string());
        let auth_user = auth_service.basic_auth(&auth_user_command).await;
    }
}
