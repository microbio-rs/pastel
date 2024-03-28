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

/// Default namespace kubernetes
const DEFAULT_NAMESPACE: &str = "paastel-space";

#[derive(
    new, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Namespace(String);

impl AsRef<str> for Namespace {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsMut<str> for Namespace {
    fn as_mut(&mut self) -> &mut str {
        self.0.as_mut_str()
    }
}

impl Default for Namespace {
    fn default() -> Self {
        Self(DEFAULT_NAMESPACE.to_string())
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl From<String> for Namespace {
    fn from(value: String) -> Self {
        Self(value.clone())
    }
}

impl From<&str> for Namespace {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_default() {
        let namespace = Namespace::default();
        assert_eq!(namespace.as_ref(), "workspace");
    }

    #[test]
    fn namespace_display() {
        let namespace = Namespace("example".to_string());
        let display_format = format!("{}", namespace);
        assert_eq!(display_format.trim(), "example");
    }

    #[test]
    fn namespace_from_string() {
        let input = "custom_namespace".to_string();
        let namespace: Namespace = input.clone().into();
        assert_eq!(namespace.as_ref(), "custom_namespace");
    }

    #[test]
    fn namespace_as_ref() {
        let namespace = Namespace("direct_reference".to_string());
        assert_eq!(namespace.as_ref(), "direct_reference");
    }
}
