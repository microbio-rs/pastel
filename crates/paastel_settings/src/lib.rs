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
    path::{Path, PathBuf},
};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// Default path of settings file
const DEFAULT_SETTINGS_FILE_PATH: &str = "paastel/settings.toml";

/// Represent PaaStel settings
#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    /// Currently namespace
    namespace: String,

    /// Origin of data, file which was loaded
    location: Option<PathBuf>,
}

impl Settings {
    /// Return namespace
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return location
    pub fn location(&self) -> Option<&PathBuf> {
        self.location.as_ref()
    }
}

impl TryFrom<&Path> for Settings {
    type Error = ConfigError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::from_path(value)
    }
}

impl Settings {
    fn from_path<P: AsRef<Path>>(p: P) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(p.as_ref().to_str().unwrap()))
            .add_source(Environment::with_prefix("paastel"))
            .build()?;

        s.try_deserialize()
    }

    pub fn new() -> Result<Self, ConfigError> {
        let run_mode =
            env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name(
                "examples/hierarchical-env/config/default",
            ))
            .add_source(
                File::with_name(&format!(
                    "examples/hierarchical-env/config/{}",
                    run_mode
                ))
                .required(false),
            )
            .add_source(Environment::with_prefix("paastel"))
            .build()?;

        s.try_deserialize()
    }
}

fn default_settings_file_path() -> PathBuf {
    dirs::config_dir()
        .expect("failed to deteminate config dir")
        .join(DEFAULT_SETTINGS_FILE_PATH)
}
