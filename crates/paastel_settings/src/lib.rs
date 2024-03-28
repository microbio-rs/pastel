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
#![allow(dead_code, unused)]
use std::{
    env,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use tracing::info;

/// Default path of settings file
const DEFAULT_SETTINGS_FILE_PATH: &str = "paastel/settings.toml";

/// Default namespace
const DEFAULT_NAMESPACE: &str = "workspace";

/// Represent PaaStel settings
#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Currently namespace
    namespace: String,

    /// Origin of data, file which was loaded
    #[serde(skip_serializing)]
    location: Option<PathBuf>,
}

impl Settings {
    /// Return namespace
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn namespace_mut(&mut self) -> &mut str {
        self.namespace.as_mut_str()
    }

    /// Return location
    pub fn location(&self) -> Option<&PathBuf> {
        self.location.as_ref()
    }

    /// Set location
    pub fn location_mut(&mut self) -> Option<&mut PathBuf> {
        self.location.as_mut()
    }

    /// Loads PaaStel settings from the specific location
    fn from_path<P: AsRef<Path> + Debug>(p: P) -> Result<Self, ConfigError> {
        info!("Loading from {p:?}");

        let path_ref = p.as_ref();

        let mut s = Config::builder()
            // Required file path
            .add_source(
                File::with_name(
                    path_ref.to_str().expect("failed convert path to str"),
                )
                .required(false),
            )
            // Try loading fields from PAASTEL_ env prefix
            .add_source(Environment::with_prefix("paastel"))
            .build()?;

        info!("Loaded from {p:?}");

        let mut setting: Settings = s.try_deserialize()?;
        // location no serialize/deserialize
        let mut location = setting.location_mut();
        if location.is_none() {
            location = Some(&mut path_ref.to_path_buf());
        }

        Ok(setting)
    }

    /// Loads PaaStel settings from default file path
    pub fn from_default_path() -> Result<Self, ConfigError> {
        Self::try_from(&default_settings_file_path())
    }

    /// Loads PaaStel settings from memory or default values
    pub fn from_memory() -> Self {
        Self::default()
    }

    /// Show if PaaStel settings from a existing path
    pub fn exists(&self) -> bool {
        match self.location() {
            Some(loc) => loc.exists(),
            None => false,
        }
    }

    pub fn in_memory(&self) -> bool {
        self.location().is_none()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            namespace: DEFAULT_NAMESPACE.to_owned(),
            location: Some(default_settings_file_path()),
        }
    }
}

impl TryFrom<&Path> for Settings {
    type Error = ConfigError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::from_path(value)
    }
}

impl TryFrom<&PathBuf> for Settings {
    type Error = ConfigError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        Self::from_path(value.as_path())
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.location() {
            Some(loc) => writeln!(f, "Settings from `{loc:?}`"),
            None => writeln!(f, "Settings load from `memory`"),
        }
    }
}

fn default_settings_file_path() -> PathBuf {
    dirs::config_dir()
        .expect("failed to deteminate config dir")
        .join(DEFAULT_SETTINGS_FILE_PATH)
}
