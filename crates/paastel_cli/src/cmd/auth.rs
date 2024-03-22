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

use std::time::Duration;

use clap::{Arg, ArgMatches, Command};
use reqwest::ClientBuilder;

use crate::error::Error;

// Name your user agent after your app?
static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub fn command() -> Command {
    Command::new("login")
        .about("PaaStel login to the server")
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

pub async fn login(matches: &ArgMatches) -> Result<(), Error> {
    let username = match matches.get_one::<String>("username") {
        Some(u) => {
            let u = u.as_str();
            if u.trim().is_empty() {
                panic!("invalid username");
            } else {
                u
            }
        }
        None => "ask username",
    };

    let password = match matches.get_one::<String>("password") {
        Some(u) => {
            let u = u.as_str();
            if u.trim().is_empty() {
                panic!("invalid username");
            } else {
                u
            }
        }
        None => "ask username",
    };

    // NOTE: validate url and required value
    let url = match matches.get_one::<String>("url") {
        Some(u) => {
            let u = u.as_str();
            if u.trim().is_empty() {
                panic!("invalid username");
            } else {
                u
            }
        }
        None => "ask username",
    };

    // TODO: update settings with username, password, url

    // verify credentials
    let client = ClientBuilder::new()
        .user_agent(APP_USER_AGENT)
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let res = client
        .get(&format!("{url}{}", "/me"))
        .basic_auth(&username, Some(password))
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        println!("sucesso");
    } else {
        println!("falha");
    }

    // TODO: save settings (create file)

    Ok(())
}
