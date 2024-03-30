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
#[cfg(test)]
use mockall::automock;

use crate::{Credential, UserSecret, Username};

///////////////////////////////////////////////////////////////////////////////
// Ports Incoming
///////////////////////////////////////////////////////////////////////////////

/// # Login use case
///
/// Incoming port
#[async_trait]
pub trait LoginUseCase {
    async fn login(&self, credential: &Credential) -> crate::Result<()>;
}

///////////////////////////////////////////////////////////////////////////////
// Ports Outgoing
///////////////////////////////////////////////////////////////////////////////

/// Outogoing port to iteract with kubernetes api
#[cfg_attr(test, automock)]
#[async_trait]
pub trait KubeSecretPort {
    async fn get_secret(
        &self,
        username: &Username,
    ) -> crate::Result<UserSecret>;
}

/// Outgoing port to check password
#[cfg_attr(test, automock)]
#[async_trait]
pub trait PasswordHashPort<P: AsRef<str> + Send + Sync> {
    async fn check_password(
        &self,
        password_text: &P,
        password_hash: &P,
    ) -> crate::Result<()>;
}
