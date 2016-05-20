use hyper::client::RequestBuilder;
use hyper::header::Headers;
use hyper;
use std::result;
use data::Level;
use error::Error;

header! { (XStarfighterAuthorization, "X-Starfighter-Authorization") => [String] }

pub struct Client {
    api_key: String,
    http_client: hyper::Client,
}
impl Client {
    fn put(&mut self, url: &str) -> RequestBuilder {
        let build = self.http_client.put(url);
        return self.add_header(build);
    }
    fn post(&mut self, url: &str) -> RequestBuilder {
        let build = self.http_client.post(url);
        return self.add_header(build);
    }
    fn add_header<'a>(&'a self, b: RequestBuilder<'a>) -> RequestBuilder {
        let mut headers = Headers::new();
        headers.set(XStarfighterAuthorization(self.api_key.to_owned()));
        return b.headers(headers);
    }
    pub fn start_level(&mut self, level: &str) -> result::Result<Level, Error> {
        // Start a level
        let url = "https://www.stockfighter.io/gm/levels/".to_string() + level;
        let mut response = try!(self.post(&url).send());
        let level = try!(Level::new(&mut response));
        Ok(level)
    }
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_string(),
            http_client: hyper::Client::new(),
        }
    }
}
