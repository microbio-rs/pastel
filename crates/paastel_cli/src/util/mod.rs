use std::path::Path;

use clap::{Arg, ArgAction};
use paastel_settings::{Location, Settings};

// pub mod compress;
// pub mod style;

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
    use tracing_subscriber::prelude::*;

    let env = tracing_subscriber::EnvFilter::from_env("PAASTEL_LOG");
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::Uptime::default())
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_writer(std::io::stderr)
        .with_filter(env);

    let registry = tracing_subscriber::registry().with(fmt_layer);
    registry.init();

    tracing::trace!(
        start =
            humantime::format_rfc3339(std::time::SystemTime::now()).to_string()
    );
}

pub(crate) fn parse_settings_var(env: &str) -> Result<Settings, String> {
    let path = Path::new(env).to_path_buf();
    let settings_location: Location = path.into();
    let settings =
        Settings::try_from(&settings_location).map_err(|e| e.to_string())?;
    Ok(settings)
}
