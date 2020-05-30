use clap::{crate_authors, crate_version, App, Arg};

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
             .about("The log file to parse")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("output")
             .short('o')
             .long("output")
             .value_name("FILE")
             .about("The file to store found IoCs.")
             .takes_value(true))
        .get_matches();
}
