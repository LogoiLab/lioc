use crypto_hash::{Algorithm, hex_digest};

pub fn follow_url(url: String) -> String {
    let mut data = Vec::new();
    let mut curl = curl::easy::Easy::new();
    curl.url(url.as_str()).unwrap();
    curl.follow_location(true).unwrap();
    {
        let mut transfer = curl.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    return hex_digest(Algorithm::SHA256, data.as_slice());
}
