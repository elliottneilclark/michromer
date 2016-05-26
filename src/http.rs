
use hyper::client::RequestBuilder;
use hyper::header::Headers;
use hyper::Client;
use std::result;
use std::sync::Arc;
use error::Error;
use rustc_serialize::Decodable;
use data::parse_response;

/// Header class used to send the api key on each http request.
header! { (XStarfighterAuthorization, "X-Starfighter-Authorization") => [String] }

#[derive(Debug)]
pub struct AuthHttpClient {
    api_key: String,
    http_client: Arc<Client>,
}

impl AuthHttpClient {
    pub fn get<T: Decodable>(&self, url: &str) -> result::Result<T, Error> {
        let mut response = try!(self.build_get(url).send());
        let r: T = try!(parse_response(&mut response));
        Ok(r)
    }
    pub fn post<T: Decodable>(&self, url: &str, body: Option<&str>) -> result::Result<T, Error> {
        let mut response = try!(self.build_post(url, body).send());
        let r: T = try!(parse_response(&mut response));
        Ok(r)
    }
    pub fn delete<T: Decodable>(&self, url: &str) -> result::Result<T, Error> {
        let mut response = try!(self.build_delete(url).send());
        let r: T = try!(parse_response(&mut response));
        Ok(r)
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
    pub fn new(key: &str) -> AuthHttpClient {
        AuthHttpClient {
            api_key: key.to_string(),
            http_client: Arc::new(Client::new()),
        }
    }
}

impl Clone for AuthHttpClient {
    fn clone(&self) -> AuthHttpClient {
        AuthHttpClient {
            api_key: self.api_key.clone(),
            http_client: self.http_client.clone(),
        }
    }
}
