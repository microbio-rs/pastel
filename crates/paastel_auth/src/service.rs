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
use mockall::automock;
use mockall::predicate::*;
use tracing::info;

use crate::{Credential, LoginUseCase};

/// # AuthService
///
/// This service implement use cases from authentication
pub struct AuthService {}

#[automock]
#[async_trait]
impl LoginUseCase for AuthService {
    async fn login(&self, credential: Credential) -> crate::Result<()> {
        info!("Login to your PaaStel cluster [{}]", credential.url());
        // 1. validate credential
        // 2. call port kubernetes secrets for get secret credential
        // 3. verify password
        info!("Login succesfull");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use url::Url;

    use crate::{Password, ServerUrl, Username};

    use super::*;

    #[tokio::test]
    async fn auth_service() {
        let mut mock = MockAuthService::new();
        let credential = Credential::new(
            Username::new("username"),
            Password::new("password"),
            ServerUrl::new(Url::parse("http://127.0.0.1:3000").unwrap()),
        );
        mock.expect_login()
            .with(predicate::eq(credential.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let result = mock.login(credential).await;
        assert!(result.is_ok());
    }
}
