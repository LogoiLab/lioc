use super::ioc::{Ioc, IocType};

use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

pub fn parse_file(path: &str) -> Vec<Ioc> {
    let file = match File::open(path) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to read log file: {}", e);
            panic!();
        },
    };

    let reader = BufReader::new(file);

    let mut iocs: Vec<Ioc> = vec!();

    for line in reader.lines() {
        match line {
            Ok(o) => {
                match parse_line(o) {
                    Some(s) => iocs.push(s),
                    None => (),
                }
            },
            Err(_) => (),
        }
    }
    return iocs;
}

fn parse_line(line: String) -> Option<Ioc> {
    let url_regex = Regex::new(r"(https?|ftps?|file|data)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]").unwrap();

    if url_regex.is_match(line.as_str()) {
        let url_match = url_regex.find(line.as_str()).unwrap();
        return Some(Ioc::new(IocType::Url, String::from(line.get(url_match.start()..url_match.end()).unwrap())));
    } else {
        return None;
    }
}
