
use hyper::header::Headers;
use hyper::client;
use std::sync::Arc;
use error::Result;
use std::io::Read;

/// Header class used to send the api key on each http request.
header! { (XStarfighterAuthorization, "X-Starfighter-Authorization") => [String] }

#[derive(Debug, Clone)]
pub struct AuthHttpClient {
    api_key: String,
    http_client: Arc<client::Client>,
}

pub trait HttpClient {
    fn get(&self, url: &str) -> Result<String>;
    fn post(&self, url: &str, body: Option<&str>) -> Result<String>;
    fn delete(&self, url: &str) -> Result<String>;
}

impl AuthHttpClient {
    pub fn new(key: &str) -> AuthHttpClient {
        AuthHttpClient {
            api_key: key.to_string(),
            http_client: Arc::new(client::Client::new()),
        }
    }
    fn build_post<'a>(&'a self, url: &str, body: Option<&'a str>) -> client::RequestBuilder {
        let mut build = self.http_client.post(url);
        if let Some(b) = body {
            build = build.body(b);
        }
        build = self.add_header(build);
        build
    }
    fn build_get(&self, url: &str) -> client::RequestBuilder {
        let mut build = self.http_client.get(url);
        build = self.add_header(build);
        build
    }
    fn build_delete(&self, url: &str) -> client::RequestBuilder {
        let mut build = self.http_client.delete(url);
        build = self.add_header(build);
        build
    }
    fn add_header<'a>(&'a self, b: client::RequestBuilder<'a>) -> client::RequestBuilder {
        let mut headers = Headers::new();
        headers.set(XStarfighterAuthorization(self.api_key.clone()));
        b.headers(headers)
    }
}
impl HttpClient for AuthHttpClient {
    fn get(&self, url: &str) -> Result<String> {
        let mut r = try!(self.build_get(url).send());
        let mut buf = String::new();
        try!(r.read_to_string(&mut buf));
        trace!("get buf = {:?}", buf);
        Ok(buf)
    }
    fn post(&self, url: &str, body: Option<&str>) -> Result<String> {
        let mut r = try!(self.build_post(url, body).send());
        let mut buf = String::new();
        try!(r.read_to_string(&mut buf));
        trace!("post buf = {:?}", buf);
        Ok(buf)
    }
    fn delete(&self, url: &str) -> Result<String> {
        let mut r = try!(self.build_delete(url).send());
        let mut buf = String::new();
        try!(r.read_to_string(&mut buf));
        trace!("del buf = {:?}", buf);
        Ok(buf)
    }
}
