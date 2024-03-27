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

// use std::time::Duration;

use std::fmt::Display;

use clap::{Arg, ArgAction, ArgMatches, Command};
use derive_new::new;
use requestty::Question;
// use reqwest::ClientBuilder;

use crate::{error::Error, util::flag};

// Name your user agent after your app?
// static APP_USER_AGENT: &str =
// concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(new)]
pub struct Username(String);

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

/// Password encoded
#[derive(new)]
pub struct PasswordEncoded(String);

impl Display for PasswordEncoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl AsRef<str> for PasswordEncoded {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<[u8]> for PasswordEncoded {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(new)]
pub struct PasswordDecoded(String);

impl PasswordDecoded {
    pub fn show_sensitive(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for PasswordDecoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "*******")
    }
}

impl AsRef<str> for PasswordDecoded {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<[u8]> for PasswordDecoded {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

// impl From<PasswordDecoded> for PasswordEncoded {
//     fn from(_value: PasswordDecoded) -> Self {
//         todo!()
//     }
// }

#[derive(new)]
pub struct Credential {
    username: Username,
    password: PasswordEncoded,
}

impl Credential {
    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn password(&self) -> &PasswordEncoded {
        &self.password
    }
}

pub fn command() -> Command {
    let usage = color_print::cstr!(
        "<cyan,bold>paastel login</> <cyan>[OPTIONS] <<URL>></>"
    );
    Command::new("login")
        .about("PaaStel login to the server at <url>")
        .override_usage(usage)
        .arg(
            Arg::new("url")
                .value_name("URL")
                .action(ArgAction::Set)
                .required(true),
        )
        .arg(
            flag("username", "Username that will be used to login")
                .env("PAASTEL_USERNAME"),
        )
        .arg(
            flag("password", "Password that will be used to login")
                .env("PAASTEL_PASSWORD"),
        )
}

fn is_valid(password: &str, _: &requestty::Answers) -> bool {
    password.contains(|c: char| c.is_ascii_digit())
        && password.contains(char::is_alphabetic)
}

fn letter_and_numbers(
    password: &str,
    ans: &requestty::Answers,
) -> Result<(), String> {
    if is_valid(password, ans) {
        Ok(())
    } else {
        Err("Password needs to have at least 1 letter and 1 number.".to_owned())
    }
}

pub async fn login(_matches: &ArgMatches) -> Result<(), Error> {
    let questions = vec![
        Question::password("password1")
            .message("Enter a password")
            .validate(letter_and_numbers)
            .build(),
        Question::password("password2")
            .message("Enter a masked password")
            .mask('*')
            .validate_on_key(is_valid)
            .validate(letter_and_numbers)
            .build(),
    ];

    println!("{:#?}", requestty::prompt(questions));
    // let username = match matches.get_one::<String>("username") {
    //     Some(u) => {
    //         let u = u.as_str();
    //         if u.trim().is_empty() {
    //             panic!("invalid username");
    //         } else {
    //             u
    //         }
    //     }
    //     None => "ask username",
    // };

    // let password = match matches.get_one::<String>("password") {
    //     Some(u) => {
    //         let u = u.as_str();
    //         if u.trim().is_empty() {
    //             panic!("invalid username");
    //         } else {
    //             u
    //         }
    //     }
    //     None => "ask username",
    // };

    // // NOTE: validate url and required value
    // let url = match matches.get_one::<String>("url") {
    //     Some(u) => {
    //         let u = u.as_str();
    //         if u.trim().is_empty() {
    //             panic!("invalid username");
    //         } else {
    //             u
    //         }
    //     }
    //     None => "ask username",
    // };

    // TODO: update settings with username, password, url

    // // verify credentials
    // let client = ClientBuilder::new()
    //     .user_agent(APP_USER_AGENT)
    //     .timeout(Duration::from_secs(5))
    //     .build()
    //     .unwrap();
    // let res = client
    //     .get(&format!("{url}{}", "/me"))
    //     .basic_auth(username, Some(password))
    //     .send()
    //     .await
    //     .unwrap();

    // if res.status().is_success() {
    //     println!("sucesso");
    // } else {
    //     println!("falha");
    // }

    // TODO: save settings (create file)

    Ok(())
}
