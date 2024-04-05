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

use std::{fmt::Display, str::FromStr};

use derive_new::new;

use crate::Error;

/// Minimium username length
const MIN_USERNAME_LENGTH: usize = 3;

/// Maximum username length
const MAX_USERNAME_LENGTH: usize = 30;

/// Minimium username length
const MIN_PASSWORD_LENGTH: usize = 6;

/// Maximum username length
const MAX_PASSWORD_LENGTH: usize = 20;

/// Username of user
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Username(String);

impl Username {
    fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl TryFrom<Vec<u8>> for Username {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> crate::Result<Self> {
        match std::str::from_utf8(&value) {
            Ok(v) => v.parse(),
            Err(_) => Err(Error::DomainError(
                "failed try username from vec<u8>".to_string(),
            )),
        }
    }
}

impl TryFrom<&[u8]> for Username {
    type Error = Error;

    fn try_from(value: &[u8]) -> crate::Result<Self> {
        match std::str::from_utf8(&value) {
            Ok(v) => v.parse(),
            Err(_) => Err(Error::DomainError(
                "failed try username from vec<u8>".to_string(),
            )),
        }
    }
}

impl TryFrom<String> for Username {
    type Error = Error;

    fn try_from(value: String) -> crate::Result<Self> {
        value.as_str().parse()
    }
}

impl FromStr for Username {
    type Err = Error;

    fn from_str(value: &str) -> crate::Result<Self> {
        if value.trim().is_empty() {
            return Err(Error::DomainError(
                "`username` not be empty".to_string(),
            ));
        }

        if value.trim().len() < MIN_USERNAME_LENGTH
            || value.trim().len() > MAX_USERNAME_LENGTH
        {
            return Err(Error::DomainError(format!(
                "`username` must be greater than {MIN_USERNAME_LENGTH} and less than {MAX_USERNAME_LENGTH}"
            )));
        }
        Ok(Self::new(value))
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait RetrievePassword<T: AsRef<str>> {
    fn password(&self) -> &T;
}

/// PasswordHash of user
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PasswordHash(String);

impl PasswordHash {
    fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl TryFrom<Vec<u8>> for PasswordHash {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> crate::Result<Self> {
        match std::str::from_utf8(&value) {
            Ok(v) => v.parse(),
            Err(_) => Err(Error::DomainError(
                "failed try password hash from vec<u8>".to_string(),
            )),
        }
    }
}

impl TryFrom<&[u8]> for PasswordHash {
    type Error = Error;

    fn try_from(value: &[u8]) -> crate::Result<Self> {
        match std::str::from_utf8(&value) {
            Ok(v) => v.parse(),
            Err(_) => Err(Error::DomainError(
                "failed try password hash from vec<u8>".to_string(),
            )),
        }
    }
}

impl TryFrom<String> for PasswordHash {
    type Error = Error;

    fn try_from(value: String) -> crate::Result<Self> {
        value.as_str().parse()
    }
}

impl FromStr for PasswordHash {
    type Err = Error;

    fn from_str(value: &str) -> crate::Result<Self> {
        if value.trim().is_empty() {
            return Err(Error::DomainError(
                "`password` not be empty".to_string(),
            ));
        }
        Ok(Self::new(value))
    }
}

impl AsRef<str> for PasswordHash {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Password of user
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Password(String);

impl Password {
    fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl TryFrom<String> for Password {
    type Error = Error;

    fn try_from(value: String) -> crate::Result<Self> {
        value.as_str().parse()
    }
}

impl FromStr for Password {
    type Err = Error;

    fn from_str(value: &str) -> crate::Result<Self> {
        if value.trim().is_empty() {
            return Err(Error::DomainError(
                "`password` not be empty".to_string(),
            ));
        }

        if value.trim().len() < MIN_PASSWORD_LENGTH
            || value.trim().len() > MAX_PASSWORD_LENGTH
        {
            return Err(Error::DomainError(format!(
                "`password` must be greater than {MIN_PASSWORD_LENGTH} and less than {MAX_PASSWORD_LENGTH}"
            )));
        }
        Ok(Self::new(value))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**********")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Credential {
    /// Username of credential
    username: Username,

    /// Password text format of credential
    password: Password,
}

impl Credential {
    pub fn new<U: AsRef<str>, P: AsRef<str>>(
        username: U,
        password: P,
    ) -> crate::Result<Self> {
        Ok(Self {
            username: username.as_ref().parse()?,
            password: password.as_ref().parse()?,
        })
    }

    /// Return reference username
    pub fn username(&self) -> &Username {
        &self.username
    }

    // /// Return reference password
    // pub fn password(&self) -> &Password {
    // }
}

impl RetrievePassword<Password> for Credential {
    fn password(&self) -> &Password {
        &self.password
    }
}

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserSecret {
    username: Username,
    password: PasswordHash,
}

impl UserSecret {
    // pub fn new<U: TryFrom<Username, Error = Error>, P: AsRef<str>>(
    //     username: U,
    //     password: P,
    // ) -> crate::Result<Self> {
    //     Ok(Self {
    //         username: Usernameinto(),
    //         password: password.as_ref().parse()?,
    //     })
    // }
    pub fn username(&self) -> &Username {
        &self.username
    }

    // pub fn password(&self) -> &Password {
    //     &self.password
    // }
}

impl RetrievePassword<PasswordHash> for UserSecret {
    fn password(&self) -> &PasswordHash {
        &self.password
    }
}

#[derive(new)]
pub struct UserSecrets(Vec<UserSecret>);

impl UserSecrets {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> UserSecretsIterator {
        UserSecretsIterator {
            data: self,
            index: 0,
        }
    }
}

impl IntoIterator for UserSecrets {
    type Item = UserSecret;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct UserSecretsIterator<'a> {
    data: &'a UserSecrets,
    index: usize,
}

impl<'a> Iterator for UserSecretsIterator<'a> {
    type Item = &'a UserSecret;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let result = &self.data.0[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelKey(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelValue(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecretLabel {
    key: LabelKey,
    value: LabelValue,
}

impl SecretLabel {
    pub fn new<U: AsRef<str>, P: AsRef<str>>(
        key: U,
        value: P,
    ) -> crate::Result<Self> {
        Ok(Self {
            key: key.as_ref().parse()?,
            value: value.as_ref().parse()?,
        })
    }

    pub fn key(&self) -> &LabelKey {
        &self.key
    }

    pub fn value(&self) -> &LabelValue {
        &self.value
    }
}

impl Default for SecretLabel {
    fn default() -> Self {
        SecretLabel {
            key: "paastel.io/api-user-credentials".parse().unwrap(),
            value: "true".parse().unwrap(),
        }
    }
}

impl Display for SecretLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key(), self.value())
    }
}

impl LabelValue {
    fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl TryFrom<String> for LabelValue {
    type Error = Error;

    fn try_from(value: String) -> crate::Result<Self> {
        value.as_str().parse()
    }
}

impl FromStr for LabelValue {
    type Err = Error;

    fn from_str(value: &str) -> crate::Result<Self> {
        if value.trim().is_empty() {
            return Err(Error::DomainError(
                "`label key` not be empty".to_string(),
            ));
        }

        Ok(Self::new(value))
    }
}

impl AsRef<str> for LabelValue {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for LabelValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl LabelKey {
    fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl TryFrom<String> for LabelKey {
    type Error = Error;

    fn try_from(value: String) -> crate::Result<Self> {
        value.as_str().parse()
    }
}

impl FromStr for LabelKey {
    type Err = Error;

    fn from_str(value: &str) -> crate::Result<Self> {
        if value.trim().is_empty() {
            return Err(Error::DomainError(
                "`label key` not be empty".to_string(),
            ));
        }

        Ok(Self::new(value))
    }
}

impl AsRef<str> for LabelKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for LabelKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_success() {
        let username = Username::from_str("validUser").unwrap();
        assert_eq!(username.as_ref(), "validUser");
    }

    #[test]
    fn test_username_empty() {
        let result = Username::from_str("");
        assert!(result.is_err());
    }

    #[test]
    fn test_username_too_short() {
        let result = Username::from_str("ab");
        assert!(result.is_err());
    }

    #[test]
    fn test_username_too_long() {
        let result =
            Username::from_str("a".repeat(MAX_USERNAME_LENGTH + 1).as_str());
        assert!(result.is_err());
    }

    #[test]
    fn test_password_success() {
        let password = Password::from_str("validPass123").unwrap();
        assert_eq!(password.as_ref(), "validPass123");
    }

    #[test]
    fn test_password_empty() {
        let result = Password::from_str("");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_too_short() {
        let result = Password::from_str("12345");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_too_long() {
        let result =
            Password::from_str("a".repeat(MAX_PASSWORD_LENGTH + 1).as_str());
        assert!(result.is_err());
    }

    #[test]
    fn test_credential_success() {
        let credential = Credential::new("validUser", "validPass123").unwrap();
        assert_eq!(credential.username().as_ref(), "validUser");
        assert_eq!(credential.password().as_ref(), "validPass123");
    }

    #[test]
    fn test_credential_invalid_username() {
        let result = Credential::new("ab", "validPass123");
        assert!(result.is_err());
    }

    #[test]
    fn test_credential_invalid_password() {
        let result = Credential::new("validUser", "12345");
        assert!(result.is_err());
    }
}
