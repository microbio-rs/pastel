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
    fmt::{Debug, Display},
    path::Path,
};

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use tracing::info;

pub mod namespace;
pub use namespace::*;

pub mod location;
pub use location::*;

/// Represent PaaStel settings
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Currently namespace default is `workspace`
    namespace: Namespace,

    /// Origin of data, now from memory or file
    // #[serde(skip_serializing)]
    location: Location,
}

impl Settings {
    /// Return namespace
    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn namespace_mut(&mut self) -> &mut Namespace {
        &mut self.namespace
    }

    /// Return location
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Set location
    pub fn location_mut(&mut self) -> &mut Location {
        &mut self.location
    }

    fn from_location(loc: &Location) -> Result<Self, ConfigError> {
        match loc.exists() {
            true => Self::from_path(loc),
            false => match loc.is_default_path() {
                true => Ok(Self::default()),
                false => Ok(Self {
                    location: loc.clone(),
                    ..Default::default()
                }),
            },
        }
    }
    /// Loads PaaStel settings from the specific location
    fn from_path<P: AsRef<Path> + Debug>(p: P) -> Result<Self, ConfigError> {
        let path = p.as_ref();
        let location: Location = path.into();

        tracing::debug!(?path, "load config from file");

        let s = Config::builder()
            // Required file path
            .add_source(
                File::with_name(
                    path.to_str()
                        .expect("failed convert path of settings to str"),
                )
                .required(true),
            )
            // Try loading fields from PAASTEL_ env prefix
            .add_source(Environment::with_prefix("paastel"))
            .set_override("location.file", location)?
            .build()?;

        info!("Loaded from {p:?}");

        s.try_deserialize()
    }

    // /// Loads PaaStel settings from default file path
    // pub fn from_default_path() -> Result<Self, ConfigError> {
    //     Self::try_from(&default_settings_file_path())
    // }
}

impl TryFrom<&Location> for Settings {
    type Error = ConfigError;

    fn try_from(location: &Location) -> Result<Self, Self::Error> {
        Self::from_location(location)
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "load from `{}` location", self.location())
    }
}
