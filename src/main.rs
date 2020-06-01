pub mod parser;
pub mod ioc;

#[macro_use]
extern crate log;

use parser::*;

use clap::{crate_authors, crate_version, App, Arg};
use env_logger::Env;

fn main() {
    let matches = App::new("lioc")
        .about("A smart IoC scanner for log files")
    // use crate_authors! to pull the author(s) names from the Cargo.toml
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("log")
             .short('l')
             .long("log")
             .value_name("FILE")
             .about("The log file to parse.")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("output")
             .short('o')
             .long("output")
             .value_name("FILE")
             .about("The file to store found IoCs.")
             .takes_value(true))
        .arg(Arg::with_name("follow")
             .short('f')
             .long("follow")
             .about("Follow URLs, hash the pages/files at the URL location.")
             .takes_value(false))
        .arg(Arg::with_name("verbose")
             .short('v')
             .long("verbose")
             .about("Set the verbosity level")
             .multiple(true)
             .takes_value(false))
        .get_matches();

    let verbosity = match matches.occurrences_of("verbose") {
        0 => "info",
        1 => "debug",
        2 => "trace",
        _ => "trace",
    };
    env_logger::from_env(Env::default().default_filter_or(verbosity)).init();
    debug!("Debug mode");
    trace!("Trace mode");

    if let Some(log_path) = matches.value_of("log") {
        for ioc in parse_file(log_path) {
            println!("IoC type: {:?}, IoC: {}", ioc.ioc_type, ioc.data);
        }
    }
}
