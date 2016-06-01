use error::Error;
use serde::de::Deserialize;
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Level {
    pub ok: bool,
    #[serde(rename="instanceId")]
    pub instance_id: i64,
    pub account: String,
    pub instructions: HashMap<String, String>,
    pub tickers: Vec<String>,
    pub venues: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeartBeatResponse {
    pub ok: bool,
    pub error: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VenueHeartBeatResponse {
    pub ok: bool,
    pub venue: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StockListResponse {
    pub ok: bool,
    pub symbols: Vec<StockSymbol>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StockSymbol {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderbookResponse {
    pub ok: bool,
    pub venue: String,
    pub symbol: String,
    pub bids: Option<Vec<BidAsk>>,
    pub asks: Option<Vec<BidAsk>>,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAsk {
    pub price: u64,
    pub qty: u64,
    #[serde(rename="isBuy")]
    pub is_buy: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteResponse {
    pub ok: bool,
    pub symbol: String,
    pub venue: String,
    pub bid: Option<u64>,
    pub ask: Option<u64>,
    #[serde(rename="bidSize")]
    pub bid_size: u64,
    #[serde(rename="askSize")]
    pub ask_size: u64,
    #[serde(rename="bidDepth")]
    pub bid_depth: u64,
    #[serde(rename="askDepth")]
    pub ask_depth: u64,
    pub last: Option<u64>,
    #[serde(rename="lastSize")]
    pub last_size: Option<u64>,
    #[serde(rename="lastTrade")]
    pub last_trade: Option<String>,
    #[serde(rename="quoteTime")]
    pub quote_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub account: String,
    pub venue: String,
    pub stock: String,
    pub price: u64,
    pub qty: u64,
    // [TODO]: Come back and make this type safe. - 2016-05-20 10:42P
    pub direction: String,
    #[serde(rename="orderType")]
    pub order_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderResponse {
    pub ok: bool,
    pub symbol: String,
    pub venue: String,
    #[serde(rename="originalQty")]
    pub original_qty: u64,
    pub qty: u64,
    pub price: u64,
    #[serde(rename="orderType")]
    pub order_type: String,
    pub id: u64,
    pub account: String,
    pub ts: String,
    pub fills: Vec<Fill>,
    #[serde(rename="totalFilled")]
    pub total_filled: u64,
    pub open: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fill {
    pub price: u64,
    pub qty: u64,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderStatusResponse {
    pub ok: bool,
    pub symbol: String,
    pub venue: String,
    pub direction: String,
    #[serde(rename="originalQty")]
    pub original_qty: u64,
    pub qty: u64,
    pub price: u64,
    #[serde(rename="orderType")]
    pub order_type: String,
    pub id: u64,
    pub account: String,
    pub fills: Vec<Fill>,
    pub ts: String,
    pub open: bool,
    #[serde(rename="totalFilled")]
    pub total_filled: u64,
}



pub fn parse_response<T: Deserialize>(buf: &str) -> Result<T, Error> {
    let l: T = try!(serde_json::from_str(&buf));
    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn test_encode_order() {
        let o = Order {
            account: "tstacc".to_string(),
            venue: "testven".to_string(),
            stock: "BOOK".to_string(),
            price: 1000,
            qty: 5000,
            direction: "buy".to_string(),
            order_type: "immediate-or-cancel".to_string(),
        };
        let res = serde_json::to_string(&o).unwrap();
        assert!(res.len() > 0);
    }

    #[test]
    fn test_encode_level_field_names() {
        let l = Level {
            ok: true,
            instance_id: 998,
            account: "tacc".to_string(),
            instructions: HashMap::new(),
            tickers: vec![],
            venues: vec![],
        };
        let res_string = serde_json::to_string(&l).unwrap();
        assert!(res_string.contains("instanceId"));
    }

    #[test]
    fn test_bid_ask_encode_field_name() {
        let ba = BidAsk {
            price: 1009,
            qty: 100,
            is_buy: true,
        };
        let res_string = serde_json::to_string(&ba).unwrap();
        assert!(res_string.contains("isBuy"));
    }

    #[test]
    fn test_quote_response_fieled_name() {
        let qr = QuoteResponse {
            ok: true,
            symbol: "BOOK".to_string(),
            venue: "MYEX".to_string(),
            bid: Some(101),
            ask: Some(105),
            bid_size: 50,
            ask_size: 1,
            bid_depth: 9,
            ask_depth: 9,
            last: Some(102),
            last_size: Some(3),
            last_trade: Some("Sometime".to_string()),
            quote_time: Some("Someothertime".to_string()),
        };
        let res_string = serde_json::to_string(&qr).unwrap();
        assert!(res_string.contains("bidSize"));
        assert!(res_string.contains("askSize"));
        assert!(res_string.contains("bidDepth"));
        assert!(res_string.contains("askDepth"));
        assert!(res_string.contains("lastSize"));
        assert!(res_string.contains("lastTrade"));
        assert!(res_string.contains("quoteTime"));
    }


    #[test]
    fn test_order_response_field_name() {
        let or = OrderResponse{
            ok: true,
            symbol: "BOOK".to_string(),
            venue: "MYEX".to_string(),
            original_qty: 10,
            qty: 9,
            price: 1045,
            order_type: "limit".to_string(),
            id: 99803,
            account: "MYACC".to_string(),
            ts: "TIME".to_string(),
            fills: vec![],
            total_filled: 1,
            open: true
        };
        let res_string = serde_json::to_string(&or).unwrap();
        assert!(res_string.contains("originalQty"));
        assert!(res_string.contains("orderType"));
        assert!(res_string.contains("totalFilled"));
    }
}
