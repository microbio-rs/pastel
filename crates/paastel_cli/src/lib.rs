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
pub mod util;

// use util::flag;

use std::path::PathBuf;

use clap::{Arg, Command};

pub async fn execute() -> Result<(), error::Error> {
    // init_tracing();

    // let usage = color_print::cstr!(
    //     "<cyan,bold>paastel</> <cyan>[OPTIONS] [COMMAND]</>"
    // );

    let command = Command::new("paastel")
        //         .next_display_order(800)
        //         .disable_version_flag(true)
        //         .term_width(80)
        //         .help_template(color_print::cstr!(
        //             "\
        // CLI to interact with PaaStel
        // <green,bold>Usage:</> {usage}
        // <green,bold>Options:</>
        // {options}
        // <green,bold>Commands:</>
        // {subcommands}
        // See '<cyan,bold>paastel help</> <cyan><<command>></>' for more information on a specific command.\n",
        //         ))
        //         .styles(get_styles())
        //         .override_usage(usage)
        // .subcommand(cmd::settings::command())
        // .subcommand(cmd::auth::command())
        // .subcommand(
        //     Command::new("server")
        //     .about("Starts the PaaStel rest server.")
        //     .long_about("This command starts the PaaStel server. Ensures the server is running inside your cluster. Normally you don't need to run this command manually.")
        // )
        // .subcommand(cmd::push::command())
        //  .arg(
        //     Arg::new("verbose")
        //     .long("verbose")
        //     .help("Use verbose output (-vv very verbose)")
        //     .short('v')
        //     .action(ArgAction::Count)
        //     .global(true),
        // )
        // .arg(Arg::new("quiet").help("Do not print cargo log messages").short('q').global(true))
        // .arg(
        //     Arg::new("color")
        //         .help("Coloring: auto, always, never")
        //         .value_name("WHEN")
        //         .global(true),
        // )
        .arg(
            Arg::new("settings-file")
                .long("settings-file")
                .value_parser(clap::value_parser!(PathBuf))
                .default_value(
                    cmd::settings::default_location().into_os_string(),
                )
                .env("PAASTEL_SETTINGS")
                .help("Set path of settings file"),
        );
    // .arg(flag("version", "Print version info and exit").short('V'));
    let _matches = command.clone().get_matches();

    // if *matches.get_one::<bool>("version").unwrap() {
    //     println!("{}", cmd::version::get_version_string());
    //     return Ok(());
    // }

    // match matches.subcommand() {
    //     Some(("login", sub_m)) => cmd::auth::login(sub_m).await?,
    //     // Some(("push", sub_m)) => cmd::push::push(sub_m).await?,
    //     // Some(("settings", sub_m)) => cmd::settings::matches(sub_m)?,
    //     // Some(("server", _sub_m)) => paastel_rest::serve().await?,
    //     _ => command.print_help()?,
    // }

    Ok(())
}

// fn init_tracing() {
//     use tracing_subscriber::EnvFilter;

//     tracing_subscriber::fmt()
//         .with_env_filter(EnvFilter::from_default_env())
//         .with_target(true)
//         .init();
// }

// pub fn get_styles() -> Styles {
//     clap::builder::styling::Styles::styled()
//         .header(style::HEADER)
//         .usage(style::USAGE)
//         .literal(style::LITERAL)
//         .placeholder(style::PLACEHOLDER)
//         .error(style::ERROR)
//         .valid(style::VALID)
//         .invalid(style::INVALID)
// }
