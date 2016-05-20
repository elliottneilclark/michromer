use std::result;
use data::Level;
use error::Error;
use http::AuthHttpClient;

pub struct Client {
    http_client: AuthHttpClient,
}

impl Client {
    pub fn start_level(&mut self, level: &str) -> result::Result<Level, Error> {
        // Start a level
        let url = "https://www.stockfighter.io/gm/levels/".to_string() + level;
        let mut response = try!(self.http_client.post(&url).send());
        let level = try!(Level::new(&mut response));
        Ok(level)
    }
    pub fn new(api_key: &str) -> Client {
        Client {
            http_client: AuthHttpClient::new(api_key),
        }
    }
}
