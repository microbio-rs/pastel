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
use url::Url;

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
        Self(value.clone())
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

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
        Self(value.clone())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, new, Serialize, Deserialize, Clone,
)]
pub struct ServerUrl(Url);

impl AsRef<str> for ServerUrl {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for ServerUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(
    Debug, new, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Credential {
    username: Username,
    password: Password,
    url: ServerUrl,
}

impl Credential {
    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn password(&self) -> &Password {
        &self.password
    }

    pub fn url(&self) -> &ServerUrl {
        &self.url
    }
}
