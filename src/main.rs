
extern crate clap;

use std::io;
use std::net::IpAddr;
use std::process;
use clap::{Arg, App};


fn main() {

    let matches = App::new("validate ip")
        .version("1.0")
        .author("Rene R. <richterrettich@gmail.com>")
        .about("Validates ip addresses")
        .arg(Arg::with_name("quiet").short("q").long("quiet").help(
            "don't show any output",
        ))
        .arg(Arg::with_name("only-v4").short("4").long("only-v4").help(
            "only allows valid ipv4 addresses",
        ))
        .arg(Arg::with_name("only-v6").short("6").long("only-v6").help(
            "only allows valid ipv6 addresses",
        ))
        .arg(Arg::with_name("INPUT").help("The ip to use").index(1))
        .get_matches();




    let quiet = matches.is_present("quiet");

    let parse_result = match matches.value_of("INPUT") {
        Some(ip) => ip.parse::<IpAddr>(),
        None => read_input().parse::<IpAddr>(),
    };

    if parse_result.is_err() {
        log(quiet, "invalid ip address.");
        process::exit(1);
    }

    let ip = parse_result.ok().unwrap();


    match ip {
        IpAddr::V4(_) => {
            if matches.value_of("only-v6").is_some() {
                log(quiet, "invalid ip v6 address.");
                process::exit(1);
            }
            log(quiet, "valid ipv4");
        }
        IpAddr::V6(_) => {
            if matches.value_of("only-v4").is_some() {
                log(quiet, "invalid ip v4 address.");
                process::exit(1);
            }
            log(quiet, "valid ipv6");
        }
    }


}

fn log(quiet: bool, message: &str) {
    if !quiet {
        println!("{}", message);
    }
}


fn read_input() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect(
        "could not read input",
    );
    line
}
