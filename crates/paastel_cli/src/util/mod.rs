use std::path::Path;

use clap::{Arg, ArgAction};
use paastel_settings::{Location, Settings};

pub mod compress;
pub mod style;

pub fn flag(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .help(help)
        .action(ArgAction::SetTrue)
}

pub fn opt(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name).long(name).help(help).action(ArgAction::Set)
}

pub(crate) fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .init();
}

pub(crate) fn parse_settings_var(env: &str) -> Result<Settings, String> {
    let path = Path::new(env).to_path_buf();
    let settings_location: Location = path.into();
    let settings =
        Settings::try_from(&settings_location).map_err(|e| e.to_string())?;
    Ok(settings)
}
