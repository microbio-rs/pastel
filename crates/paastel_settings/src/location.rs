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

use std::{
    ffi::OsString,
    fmt::Display,
    path::{Path, PathBuf},
};

use config::{Value, ValueKind};
use serde::{Deserialize, Serialize};

const DEFAULT_SETTINGS_PATH: &str = "paastel/settings.toml";

/// Define where settings from
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Location {
    /// When save or another irect with settings this is location
    File(PathBuf),
}

impl Location {
    fn config_dir() -> PathBuf {
        dirs::config_dir().expect("failed to deteminate config dir")
    }

    fn default_path() -> PathBuf {
        Self::config_dir().join(DEFAULT_SETTINGS_PATH)
    }

    pub fn is_default_path(&self) -> bool {
        match self {
            Self::File(loc) => loc == &Self::default_path(),
        }
    }

    pub fn exists(&self) -> bool {
        match self {
            Self::File(loc) => loc.exists(),
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::File(Self::default_path())
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(loc) => write!(f, "{}", loc.as_path().display()),
        }
    }
}

impl AsRef<Path> for Location {
    fn as_ref(&self) -> &Path {
        match self {
            Self::File(loc) => loc.as_path(),
        }
    }
}

impl From<Location> for Value {
    fn from(value: Location) -> Self {
        Value::new(
            Some(&"location.file".to_string()),
            ValueKind::String(value.to_string()),
        )
    }
}

impl From<String> for Location {
    fn from(value: String) -> Self {
        Self::File(Path::new(&value).to_path_buf())
    }
}

impl From<&Path> for Location {
    fn from(value: &Path) -> Self {
        Self::File(value.to_path_buf())
    }
}

impl From<PathBuf> for Location {
    fn from(value: PathBuf) -> Self {
        Self::File(value)
    }
}

impl From<&PathBuf> for Location {
    fn from(value: &PathBuf) -> Self {
        Self::File(value.clone())
    }
}
