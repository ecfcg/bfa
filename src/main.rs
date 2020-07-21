use clap::{App, Arg};
use std::assert;
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
        )
        .arg(
            Arg::with_name("thread_num")
                .help("max raw string length")
                .required(true),
        )
        .arg(
            Arg::with_name("start_code")
                .help("clace starting code")
                .short("s")
                .takes_value(true),
        );

    let matches = app.get_matches();
    let hash = match matches.value_of("hash") {
        Some(s) => s,
        None => exit(1),
    };

    let len = match matches.value_of("len") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => exit(1),
    };

    let thread_num = match matches.value_of("thread_num") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => 4,
    };

    let start_code = match matches.value_of("start_code") {
        Some(s) => bfa::parse_start_code(s),
        None => Vec::new(),
    };

    assert!(len > start_code.len());

    let thread_num = if 8 < thread_num {
        8
    } else if thread_num == 0 {
        4
    } else {
        thread_num
    };

    let clk = bfa::Clacker::new(String::from(hash), len, thread_num, start_code);
    clk.clack();
}
