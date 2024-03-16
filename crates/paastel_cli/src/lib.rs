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

pub mod cmd;
pub mod error;

use clap::{
    builder::{styling::AnsiColor, Styles},
    Arg, Command,
};

const HELP_TEMPLATE: &str = "
{about}

Usage:
  {usage}

Available Commands:
{subcommands}

Flags:
{options}
";

const ABOUT_TEMPLATE: &str = " 
           ██████╗  █████╗  █████╗ ███████╗████████╗███████╗██╗     
           ██╔══██╗██╔══██╗██╔══██╗██╔════╝╚══██╔══╝██╔════╝██║     
           ██████╔╝███████║███████║███████╗   ██║   █████╗  ██║     
           ██╔═══╝ ██╔══██║██╔══██║╚════██║   ██║   ██╔══╝  ██║     
           ██║     ██║  ██║██║  ██║███████║   ██║   ███████╗███████╗
           ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚══════╝╚══════╝
        paastel cli is the official command line interface for PaaStel";

pub fn execute() -> Result<(), error::Error> {
    init_tracing();

    let matches = Command::new("paastel")
        .about(ABOUT_TEMPLATE)
        .term_width(80)
        .help_template(HELP_TEMPLATE)
        .styles(get_styles())
        .subcommand(cmd::settings::command())
        .subcommand(cmd::auth::command())
        .arg(Arg::new("settings-file"))
        .get_matches();

    match matches.subcommand() {
        Some(("login", sub_m)) => cmd::auth::login(sub_m)?,
        Some(("settings", sub_m)) => cmd::settings::matches(sub_m)?,
        _ => {}
    }

    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .init();
}

pub fn get_styles() -> Styles {
    let styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default());
    styles
}
