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

use clap::{Arg, ArgMatches, Command};

use crate::error::Error;

pub fn command() -> Command {
    Command::new("push")
        .about("Push an application declared in the specified manifest")
        .long_about(
            "The login command allows you to authenticate against \
            an Paastel instance and updates the settings file with the \
            generated authentication token",
        )
        .arg(Arg::new("url"))
        .arg(
            Arg::new("username")
                .long("username")
                .env("PAASTEL_USERNAME")
                .short('u')
                .help("username that will be used to login"),
        )
        .arg(
            Arg::new("password")
                .long("password")
                .env("PAASTEL_PASSWORD")
                .short('p')
                .help("password that will be used to login"),
        )
    // .arg(
    //     Arg::new("trust-ca")
    //         .long("trust-ca")
    //         .help("automatically trust the unknown CA")
    //         .action(clap::ArgAction::SetTrue),
    // )
    // .arg(
    //     Arg::new("oidc")
    //         .long("oidc")
    //         .help("perform OIDC authentication (user and password will be ignored)")
    //         .action(clap::ArgAction::SetTrue),
    // )
}

// Push pushes an app
// TODO: validate
// TODO: stage
// TODO: (tail logs)
// TODO: wait for staging to be done (complete or fail)
// TODO: deploy
// TODO: wait for app
pub async fn push(_matches: &ArgMatches) -> Result<(), Error> {
    // TODO: create app
    // TODO: compress folder
    // TODO: upload  /namespaces/:namespace/applications/:app/store
    // TODO: upload  s3

    todo!()
}
