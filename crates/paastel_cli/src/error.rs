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

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UrlParse(String),
    Settings(String),
    Io(String),
    Toml(String),
    Base64(String),
    Server,
    Unknown,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UrlParse(e) => write!(f, "parser url {e}"),
            Error::Settings(e) => write!(f, "settings {e}"),
            Error::Io(e) => write!(f, "io {e}"),
            Error::Toml(e) => write!(f, "toml parser {e}"),
            Error::Base64(e) => write!(f, "base64 {e}"),
            Error::Server => write!(f, "server error"),
            Error::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::error::Error for Error {}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParse(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::Toml(value.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Self::Toml(value.to_string())
    }
}

impl From<base64::DecodeError> for Error {
    fn from(value: base64::DecodeError) -> Self {
        Self::Base64(value.to_string())
    }
}

impl From<paastel_rest::error::Error> for Error {
    fn from(_: paastel_rest::error::Error) -> Self {
        Self::Server
    }
}
