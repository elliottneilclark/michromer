
use hyper::client::RequestBuilder;
use hyper::header::Headers;
use hyper::Client;
use std::result::Result;
use std::sync::Arc;
use error::Error;
use rustc_serialize::Decodable;
use data::Decoder;

/// Header class used to send the api key on each http request.
header! { (XStarfighterAuthorization, "X-Starfighter-Authorization") => [String] }

#[derive(Debug)]
pub struct AuthHttpClient {
    api_key: String,
    http_client: Arc<Client>,
    decoder: Decoder,
}

pub trait HttpClient {
    fn get<T: Decodable>(&self, url: &str) -> Result<T, Error>;
    fn post<T: Decodable>(&self, url: &str, body: Option<&str>) -> Result<T, Error>;
    fn delete<T: Decodable>(&self, url: &str) -> Result<T, Error>;
}

impl AuthHttpClient {
    pub fn new(key: &str) -> AuthHttpClient {
        AuthHttpClient {
            api_key: key.to_string(),
            http_client: Arc::new(Client::new()),
            decoder: Decoder {},
        }
    }
    fn build_post<'a>(&'a self, url: &str, body: Option<&'a str>) -> RequestBuilder {
        let mut build = self.http_client.post(url);
        if let Some(b) = body {
            build = build.body(b);
        }
        build = self.add_header(build);
        build
    }
    fn build_get(&self, url: &str) -> RequestBuilder {
        let mut build = self.http_client.get(url);
        build = self.add_header(build);
        build
    }
    fn build_delete(&self, url: &str) -> RequestBuilder {
        let mut build = self.http_client.delete(url);
        build = self.add_header(build);
        build
    }
    fn add_header<'a>(&'a self, b: RequestBuilder<'a>) -> RequestBuilder {
        let mut headers = Headers::new();
        headers.set(XStarfighterAuthorization(self.api_key.clone()));
        b.headers(headers)
    }
}
impl HttpClient for AuthHttpClient {
    fn get<T: Decodable>(&self, url: &str) -> Result<T, Error> {
        let mut response = try!(self.build_get(url).send());
        let r: T = try!(self.decoder.parse_response(&mut response));
        Ok(r)
    }
    fn post<T: Decodable>(&self, url: &str, body: Option<&str>) -> Result<T, Error> {
        let mut response = try!(self.build_post(url, body).send());
        let r: T = try!(self.decoder.parse_response(&mut response));
        Ok(r)
    }
    fn delete<T: Decodable>(&self, url: &str) -> Result<T, Error> {
        let mut response = try!(self.build_delete(url).send());
        let r: T = try!(self.decoder.parse_response(&mut response));
        Ok(r)
    }
}
impl Clone for AuthHttpClient {
    fn clone(&self) -> AuthHttpClient {
        AuthHttpClient {
            api_key: self.api_key.clone(),
            http_client: self.http_client.clone(),
            decoder: Decoder {},
        }
    }
}
