use crypto_hash::{Algorithm, hex_digest};

use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn follow_url(url: String) -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.url(url.as_str()).unwrap();
    easy.follow_location(true).unwrap();
    easy.perform().unwrap();

    return hex_digest(Algorithm::SHA256, &easy.get_ref().0);
}
