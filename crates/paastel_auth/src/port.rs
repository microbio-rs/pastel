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

use crate::{Credential, RetrievePassword, SecretLabel, UserSecret};

///////////////////////////////////////////////////////////////////////////////
// Ports Incoming
///////////////////////////////////////////////////////////////////////////////

/// # Login use case
///
/// Incoming port
#[async_trait]
pub trait ValidateCredentialUseCase {
    async fn validate_credential(
        &self,
        credential: &Credential,
    ) -> crate::Result<UserSecret>;
}

///////////////////////////////////////////////////////////////////////////////
// Ports Outgoing
///////////////////////////////////////////////////////////////////////////////

/// Outogoing port to iteract with kubernetes api
#[cfg_attr(test, automock)]
#[async_trait]
pub trait OutgoingKubernetesPort {
    async fn find_secrets_by_label(
        &self,
        label: &SecretLabel,
    ) -> crate::Result<crate::UserSecrets>;
}

pub type OutKubernetesPort = Box<dyn OutgoingKubernetesPort + Send + Sync>;

/// Outgoing port to check password
#[cfg_attr(test, automock)]
#[async_trait]
pub trait OutgoingArgon2HashPort<T, H>
where
    T: RetrievePassword + Send + Sync,
    H: RetrievePassword + Send + Sync,
{
    async fn check(
        &self,
        password_text: &T,
        password_hash: &H,
    ) -> crate::Result<()>;
}

pub type OutArgon2Port<T, H> =
    Box<dyn OutgoingArgon2HashPort<T, H> + Send + Sync>;
