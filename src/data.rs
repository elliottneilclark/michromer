use error::Error;
use serde::de::Deserialize;
use serde_json;
use std::collections::HashMap;
use chrono::*;
use std::fmt;
use serde;

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
    pub last_trade: Option<DateTime<UTC>>,
    #[serde(rename="quoteTime")]
    pub quote_time: Option<DateTime<UTC>>,
}

#[derive(Debug, Clone)]
pub enum OrderDirection {
    Buy,
    Sell,
}

impl fmt::Display for OrderDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &OrderDirection::Buy => "buy",
            &OrderDirection::Sell => "sell",
        };
        write!(f, "{}", s)
    }
}

impl serde::Serialize for OrderDirection {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}


impl serde::Deserialize for OrderDirection {
    fn deserialize<D>(deserializer: &mut D) -> Result<OrderDirection, D::Error>
        where D: serde::de::Deserializer
    {
        struct OrderDirectionVisitor;

        impl serde::de::Visitor for OrderDirectionVisitor {
            type Value = OrderDirection;

            fn visit_str<E>(&mut self, value: &str) -> Result<OrderDirection, E>
                where E: serde::de::Error
            {
                match value {
                    "buy" => Ok(OrderDirection::Buy),
                    "sell" => Ok(OrderDirection::Sell),
                    _ => Err(serde::de::Error::custom("expected Buy or Sell")),
                }
            }
        }

        deserializer.deserialize(OrderDirectionVisitor)
    }
}


#[derive(Debug, Clone)]
pub enum OrderType {
    Limit,
    Market,
    FillOrKill,
    ImmediateOrCancel,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &OrderType::Market => "market",
            &OrderType::Limit => "limit",
            &OrderType::FillOrKill => "fill-or-kill",
            &OrderType::ImmediateOrCancel => "immediate-or-cancel",

        };
        write!(f, "{}", s)
    }
}

impl serde::Serialize for OrderType {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}


impl serde::Deserialize for OrderType {
    fn deserialize<D>(deserializer: &mut D) -> Result<OrderType, D::Error>
        where D: serde::de::Deserializer
    {
        struct OrderTypeVisitor;

        impl serde::de::Visitor for OrderTypeVisitor {
            type Value = OrderType;

            fn visit_str<E>(&mut self, value: &str) -> Result<OrderType, E>
                where E: serde::de::Error
            {
                match value {
                    "limit" => Ok(OrderType::Limit),
                    "market" => Ok(OrderType::Market),
                    "fill-or-kill" => Ok(OrderType::FillOrKill),
                    "immediate-or-cancel" => Ok(OrderType::ImmediateOrCancel),
                    _ => Err(serde::de::Error::custom("expected known order type")),
                }
            }
        }

        deserializer.deserialize(OrderTypeVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub account: String,
    pub venue: String,
    pub stock: String,
    pub price: u64,
    pub qty: u64,
    pub direction: OrderDirection,
    #[serde(rename="orderType")]
    pub order_type: OrderType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderResponse {
    pub ok: bool,
    pub symbol: String,
    pub venue: String,
    pub direction: Option<OrderDirection>,
    #[serde(rename="originalQty")]
    pub original_qty: u64,
    pub qty: u64,
    pub price: u64,
    #[serde(rename="orderType")]
    pub order_type: OrderType,
    pub id: u64,
    pub account: String,
    pub ts: DateTime<UTC>,
    pub fills: Vec<Fill>,
    #[serde(rename="totalFilled")]
    pub total_filled: u64,
    pub open: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fill {
    pub price: u64,
    pub qty: u64,
    pub ts: DateTime<UTC>,
}

pub fn parse_response<T: Deserialize>(buf: &str) -> Result<T, Error> {
    let l: T = try!(serde_json::from_str(&buf));
    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::*;
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
            direction: OrderDirection::Buy,
            order_type: OrderType::ImmediateOrCancel,
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
            last_trade: Some("2015-12-04T09:02:16.680986205Z".parse().unwrap()),
            quote_time: Some("2015-12-04T09:02:16.680986205Z".parse().unwrap()),
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
        let or = OrderResponse {
            ok: true,
            direction: None,
            symbol: "BOOK".to_string(),
            venue: "MYEX".to_string(),
            original_qty: 10,
            qty: 9,
            price: 1045,
            order_type: OrderType::Limit,
            id: 99803,
            account: "MYACC".to_string(),
            ts: "2015-12-04T09:02:16.680986205Z".parse().unwrap(),
            fills: vec![],
            total_filled: 1,
            open: true,
        };
        let res_string = serde_json::to_string(&or).unwrap();
        println!("{:?}", res_string);
        assert!(res_string.contains("originalQty"));
        assert!(res_string.contains("orderType"));
        assert!(res_string.contains("totalFilled"));
    }


    #[test]
    fn test_decode_date_time() {
        let dt_one: DateTime<UTC> = "2015-12-04T09:02:16.680986205Z".parse().unwrap();
        println!("dt = {:?}", dt_one);
        let dt_two: DateTime<UTC> = "2015-07-05T22:16:18+00:00".parse().unwrap();
        println!("dt = {:?}", dt_two);
    }

    #[test]
    fn test_decode_order_response() {
        let o_json = "{\"account\": \"testacc\", \"price\": 26382757, \"id\": 2138, \"open\": \
                      false, \"venue\": \"TESTEX\", \"orderType\": \"limit\", \"qty\": 0, \
                      \"direction\": \"buy\", \"fills\": [{\"qty\": 5000, \"ts\": \
                      \"2016-06-02T16:20:53.024563Z\", \"price\": 26382747}], \"totalFilled\": \
                      5000, \"originalQty\": 5000, \"symbol\": \"FOOBAR\", \"ts\": \
                      \"2016-06-02T16:20:53.024542Z\", \"ok\": true}";

        let o: OrderResponse = parse_response(&o_json).unwrap();
        assert!(o.ok);
    }
}
