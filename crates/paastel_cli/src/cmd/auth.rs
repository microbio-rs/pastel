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

pub fn login(matches: &ArgMatches) -> Result<(), Error> {
    let _username = match matches.get_one::<String>("username") {
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

    let _password = match matches.get_one::<String>("password") {
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
    let _url = match matches.get_one::<String>("url") {
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
    // TODO: verify credentials
    // 1. settings have username, password, url api
    // 2. client call api to /me route
    // 3. /me route has middleware.Authentication
    // 3.1 check authorization header
    // 3.2 create auth service
    // 3.2.1 this auth service has kubernetes port to secret and port to configmap
    // 3.3 check if basic authentication (performs the basic authentication)
    // 3.3.1 get username and password basic_auth
    // 3.3.2 auth service above get user by username
    // 3.3.2.1 getuserbyname get all users
    // 3.3.2.2 getallusers get use secrets
    // 3.3.2.3 get use secrets call kubernetes and filter
    // 3.3.2.4 convert secrets into users
    // TODO: save settings (create file)

    println!("username: {_username}");
    println!("password: {_password}");
    println!("url: {_url}");

    Ok(())
}
