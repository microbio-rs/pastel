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
const MAX_USERNAME_LENGTH: usize = 15;

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
        write!(f, "{}", self.0)
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

    /// Return reference password
    pub fn password(&self) -> &Password {
        &self.password
    }
}

#[derive(Debug, new, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserSecret {
    username: Username,
    password: Password,
}

impl UserSecret {
    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn password_hashed(&self) -> &Password {
        &self.password
    }
}
