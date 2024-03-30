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

use std::fmt::Display;

use derive_new::new;
use serde::{Deserialize, Serialize};

/// Username of user
#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Username(String);

impl Username {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl From<String> for Username {
    fn from(value: String) -> Self {
        Self::new(value.clone())
    }
}

impl From<&str> for Username {
    fn from(value: &str) -> Self {
        Self::new(value)
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
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Password(String);

impl Password {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

impl From<String> for Password {
    fn from(value: String) -> Self {
        Self::new(value.clone())
    }
}

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Credential {
    username: Username,
    password: Password,
}

impl Credential {
    pub fn new<U: Into<Username>, P: Into<Password>>(
        username: U,
        password: P,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}

#[derive(
    Debug, new, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
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
