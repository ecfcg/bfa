use clap::{App, Arg};
use std::process::exit;

fn main() {
    let app = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(Arg::with_name("hash").help("hashed string").required(true))
        .arg(
            Arg::with_name("len")
                .help("max raw string length")
                .required(true),
        );

    let matches = app.get_matches();
    let hash = match matches.value_of("hash") {
        Some(s) => s,
        None => exit(1),
    };
    let len = match matches.value_of("len") {
        Some(s) => s,
        None => exit(1),
    };

    let hash = hash
        .chars()
        .into_iter()
        .map(|c| c.to_uppercase().to_string())
        .collect();

    let decrypter = bfa::Md5Decrypter::new();
    decrypter.decrypt(hash, len.parse::<usize>().unwrap());
}
