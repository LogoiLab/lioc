use crypto_hash::{Algorithm, hex_digest};

pub fn follow_url(url: &str) -> String {
    let body = reqwest::blocking::get(url).unwrap()
        .text().unwrap();

    return body;
}
