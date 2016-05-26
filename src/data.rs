#![allow(non_snake_case)]

use hyper::client::Response;
use rustc_serialize::Decodable;
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

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct HeartBeatResponse {
    pub ok: bool,
    pub error: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct VenueHeartBeatResponse {
    pub ok: bool,
    pub venue: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct StockListResponse {
    pub ok: bool,
    pub symbols: Vec<StockSymbol>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct StockSymbol {
    pub name: String,
    pub symbol: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct OrderbookResponse {
    pub ok: bool,
    pub venue: String,
    pub symbol: String,
    pub bids: Option<Vec<BidAsk>>,
    pub asks: Option<Vec<BidAsk>>,
    pub ts: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct BidAsk {
    pub price: u64,
    pub qty: u64,
    pub isBuy: bool,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct QuoteResponse {
    pub ok: bool,
    pub symbol: String,
    pub venue: String,
    pub bid: Option<u64>,
    pub ask: Option<u64>,
    pub bidSize: u64,
    pub askSize: u64,
    pub bidDepth: u64,
    pub askDepth: u64,
    pub last: Option<u64>,
    pub lastSize: Option<u64>,
    pub lastTrade: Option<String>,
    pub quoteTime: Option<String>,
}

#[derive(RustcEncodable, Debug)]
pub struct Order {
    pub account: String,
    pub venue: String,
    pub stock: String,
    pub price: u64,
    pub qty: u64,
    // [TODO]: Come back and make this type safe. - 2016-05-20 10:42P
    pub direction: String,
    pub orderType: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct OrderResponse {
    pub ok: bool,
    pub symbol: String,
    pub venue: String,
    pub originalQty: u64,
    pub qty: u64,
    pub price: u64,
    pub orderType: String,
    pub id: u64,
    pub account: String,
    pub ts: String,
    pub fills: Vec<Fill>,
    pub totalFilled: u64,
    pub open: bool,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Fill {
    pub price: u64,
    pub qty: u64,
    pub ts: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct OrderStatusResponse {
    pub ok: bool,
    pub symbol: String,
    pub venue: String,
    pub direction: String,
    pub originalQty: u64,
    pub qty: u64,
    pub price: u64,
    pub orderType: String,
    pub id: u64,
    pub account: String,
    pub fills: Vec<Fill>,
    pub ts: String,
    pub open: bool,
    pub totalFilled: u64,
}

pub fn parse_response<T: Decodable>(response: &mut Response) -> Result<T, Error> {
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    let l: T = try!(json::decode(&buf));
    Ok(l)
}


#[cfg(test)]
mod tests {
    use super::*;
    use rustc_serialize::json;

    #[test]
    fn test_encode_order() {
        let o = Order {
            account: "tstacc".to_string(),
            venue: "testven".to_string(),
            stock: "FB".to_string(),
            price: 1000,
            qty: 5000,
            direction: "buy".to_string(),
            orderType: "immediate-or-cancel".to_string(),
        };
        let res = json::encode(&o).unwrap();
        println!("res = {}", res);
    }
}
