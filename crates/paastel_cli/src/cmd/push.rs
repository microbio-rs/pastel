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

use std::{env::current_dir, path::Path, time::Duration};

use clap::{Arg, ArgMatches, Command};
use reqwest::ClientBuilder;

use crate::{error::Error, util::compress};

// Name your user agent after your app?
static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub fn command() -> Command {
    Command::new("push")
        .about("Push an application declared in the specified manifest")
        .long_about(
            "The login command allows you to authenticate against \
            an Paastel instance and updates the settings file with the \
            generated authentication token",
        )
        .arg(
            Arg::new("name")
                .long("name")
                .env("PAASTEL_NAME")
                .short('n')
                .help("that will be used to login"),
        )
}

// Push pushes an app
// TODO: validate
// TODO: stage
// TODO: (tail logs)
// TODO: wait for staging to be done (complete or fail)
// TODO: deploy
// TODO: wait for app
pub async fn push(_matches: &ArgMatches) -> Result<(), Error> {
    // NOTE: name is optional or generating one or get from folder name
    // TODO: create app
    // TODO: compress folder
    // TODO: upload  /namespaces/:namespace/applications/:app/store
    // TODO: upload  s3

    let out_dir = Path::new("/tmp/paastel_compress.zip");

    // compress::dir(
    //     current_dir().unwrap().to_str().unwrap(),
    //     "/tmp/paastel_compress.zip",
    // )
    // .unwrap();

    // verify credentials
    let client = ClientBuilder::new()
        .user_agent(APP_USER_AGENT)
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let content: Vec<u8> = tokio::fs::read(out_dir).await?;

    let part = reqwest::multipart::Part::bytes(content)
        .mime_str("application/zip")
        .unwrap()
        .file_name(out_dir.file_name().unwrap().to_str().unwrap());

    let file = reqwest::multipart::Form::new().part("file", part);

    let res = client
        .post(
            "http://127.0.0.1:3000/namespaces/default/applications/mysuperapp/store",
        )
        .basic_auth("admin@paastel.io", Some("password"))
        .multipart(file)
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        println!("sucesso");
    } else {
        println!("falha");
    }
    Ok(())
}
