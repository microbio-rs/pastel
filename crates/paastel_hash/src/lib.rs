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

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString,
    },
    Argon2, PasswordHasher,
};
use paastel::AuthHashPasswordPort;

pub enum Error {}

#[derive(Debug)]
pub struct Argon2Adapter<'a> {
    #[allow(dead_code)]
    salt: SaltString,
    inner: Argon2<'a>,
}

impl<'a> Argon2Adapter<'a> {
    pub fn hash_password(
        &self,
        password: &str,
    ) -> Result<String, argon2::password_hash::Error> {
        let password_hash = self
            .inner
            .hash_password(password.as_bytes(), &self.salt)?
            .to_string();
        Ok(password_hash)
    }
}

impl<'a> AuthHashPasswordPort for Argon2Adapter<'a> {
    fn verify(
        &self,
        password: &str,
        password_hash: &str,
    ) -> paastel::Result<()> {
        let parsed_hash = PasswordHash::new(password_hash).unwrap();
        self.inner
            .verify_password(password.as_bytes(), &parsed_hash)
            .unwrap();
        Ok(())
    }
}

impl<'a> Default for Argon2Adapter<'a> {
    fn default() -> Self {
        Self {
            salt: SaltString::generate(&mut OsRng),
            inner: Argon2::default(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_password_ok() {
        let password = "password";
        let argon2_adapter = Argon2Adapter::default();
        // $argon2id$v=19$m=19456,t=2,p=1$1SoziBLmGitKRfXC2+e7Ng$hfPRJDDkKyLH3FyHuqxm397sxPkmVkzydPI+LDQp+OU
        let hash_password = argon2_adapter.hash_password(password).unwrap();
        let result = argon2_adapter.verify(password, &hash_password);
        assert!(result.is_ok())
    }
}
