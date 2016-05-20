use hyper::client::Response;
use rustc_serialize::json;
use std::io::Read;
use error::Error;
use std::collections::HashMap;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Level {
    pub ok: bool,
    pub instanceId: i64,
    pub account: String,
    pub instructions: HashMap<String, String>,
    pub tickers: Vec<String>,
    pub venues: Vec<String>,
}
impl Level {
    pub fn new(response: &mut Response) -> Result<Level, Error> {
        let mut buf = String::new();
        try!(response.read_to_string(&mut buf));
        let l: Level = try!(json::decode(&buf));
        return Ok(l);
    }
}
