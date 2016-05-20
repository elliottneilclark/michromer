
use hyper::client::RequestBuilder;
use hyper::header::Headers;
use hyper::Client;

header! { (XStarfighterAuthorization, "X-Starfighter-Authorization") => [String] }
pub struct AuthHttpClient {
    api_key: String,
    http_client: Client,
}

impl AuthHttpClient {
    pub fn post(&mut self, url: &str) -> RequestBuilder {
        let build = self.http_client.post(url);
        return self.add_header(build);
    }
    fn add_header<'a>(&'a self, b: RequestBuilder<'a>) -> RequestBuilder {
        let mut headers = Headers::new();
        headers.set(XStarfighterAuthorization(self.api_key.to_owned()));
        return b.headers(headers);
    }
    pub fn new(key:&str) -> AuthHttpClient {
        AuthHttpClient {
            api_key: key.to_string(),
            http_client: Client::new(),
        }
    }
}
