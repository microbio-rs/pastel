use std::{
    fmt::Display,
    io::Write,
    path::{Path, PathBuf},
};

use base64::prelude::*;
use clap::{ArgMatches, Command};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::error::Error;

/// Default PaasTel settings file name
pub const DEFAULT_SETTINGS_PATH: &str = "paastel/settings.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenSettings {
    access_token: String,
    expiry: i64,
    refresh_token: String,
    token_type: String,
}

/// Settings represents a PaaStel settings
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    api: String,
    namespace: String,
    password: String,
    token: Option<TokenSettings>,
    username: String,
    wss: String,
    location: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api: Default::default(),
            namespace: "workspace".to_string(),
            password: Default::default(),
            token: Default::default(),
            username: Default::default(),
            wss: Default::default(),
            location: Default::default(),
        }
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Settings {
    /// Loads the PaaStel settings from specific file
    pub fn from_path<P: AsRef<Path>>(p: P) -> Result<Self, Error> {
        let path = p.as_ref();
        let location = path.to_str().unwrap();

        info!("Loading from {location}");

        let settings = if path.exists() {
            let text = std::fs::read_to_string(path)?;
            let mut settings: Settings = toml::from_str(&text)?;
            // NOTE: improve deserialize automatic decode password
            settings.password = String::from_utf8(
                BASE64_STANDARD.decode(&settings.password.trim())?,
            )
            .unwrap();
            settings
        } else {
            Settings::default()
        };

        // NOTE: hide password
        info!("Loaded value {settings}");

        Ok(settings)
    }

    /// Saves the PaaStel settings
    pub fn save(&self) -> Result<(), Error> {
        let path = default_location();
        let location = path.as_path().to_str().unwrap();
        let dir = path.parent();

        info!("Saving to {location}");

        if dir.is_none() {
            return Err(Error::Settings(String::from(
                "{location} no has parent dir",
            )));
        }

        // create dir
        let dir = dir.unwrap();
        std::fs::create_dir_all(dir)?;

        let settings_str = toml::to_string(self)?;

        // saving settings
        let mut file = std::fs::File::create(path.as_path())?;
        file.write_all(settings_str.as_bytes())?;

        info!("Saved value {self}");

        Ok(())
    }
}

pub fn default_location() -> PathBuf {
    let config_path = dirs::config_dir().unwrap();
    let config_path = config_path.join(DEFAULT_SETTINGS_PATH);
    config_path
}

pub fn load() -> Result<Settings, Error> {
    Ok(Settings::from_path(default_location().as_path())?)
}

pub fn command() -> Command {
    Command::new("settings")
        .about("PaaStel settings management")
        .long_about("Manage the PaaStel cli settings")
        .subcommand(Command::new("show"))
}

pub fn matches(m: &ArgMatches) -> Result<(), Error> {
    match m.subcommand() {
        Some(("show", _sub_m)) => {
            println!("aqui");
            let l = load()?;
            dbg!(l);
        }
        _ => {}
    }
    Ok(())
}
