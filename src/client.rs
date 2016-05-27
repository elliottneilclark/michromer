use error::Result;
use http::HttpClient;
use http::AuthHttpClient;
use rustc_serialize::Decodable;
use rustc_serialize::json;
use data::{HeartBeatResponse, Level, Order, OrderResponse, OrderStatusResponse, OrderbookResponse,
           QuoteResponse, StockListResponse, VenueHeartBeatResponse, parse_response};

static VENUE_URL: &'static str = "https://api.stockfighter.io/ob/api/venues/";
static HEARTBEAT_URL: &'static str = "https://api.stockfighter.io/ob/api/heartbeat";

/// Client for starting a new level of Stockfighter.
#[derive(Debug, Clone)]
pub struct Client<T: HttpClient + Clone> {
    http_client: T,
}

impl<T: HttpClient + Clone> Client<T> {
    /// Start a new level
    ///
    /// It appears that this will also continue a current level
    /// if there is already a level ongoing.
    pub fn start_level(&self, level: &str) -> Result<LevelClient<T>> {
        // Start a level
        let url = "https://www.stockfighter.io/gm/levels/".to_string() + level;
        let res = try!(self.http_client.post(&url, None));
        let level: Level = try!(parse_response(&res));
        // Give it back.
        Ok(LevelClient::new(self.http_client.clone(), level))
    }
}

impl Client<AuthHttpClient> {
    /// Given an api key construct a new Client that can interact with stockfighter's game api.
    pub fn new(api_key: &str) -> Client<AuthHttpClient> {
        Client { http_client: AuthHttpClient::new(api_key) }
    }
}

/// Stockfighter client for a specific level. The
/// stock api is defined [here](https://starfighter.readme.io/docs)
/// It's wrapper around an http client. As such it can return
/// errors for parsing or for network issues.
#[derive(Debug, Clone)]
pub struct LevelClient<T: HttpClient + Clone> {
    http_client: T,
    pub level: Level,
}

impl<T: HttpClient + Clone> LevelClient<T> {
    /// See if this Level is up.
    ///
    /// This should really only be used to sanity check that the level
    /// hasn't been torn down.
    pub fn heart_beat(&self) -> Result<HeartBeatResponse> {
        let url = HEARTBEAT_URL;
        self.do_get(url)
    }

    /// Check if a venue is ok.
    pub fn venue_heart_beat(&self, venue: &str) -> Result<VenueHeartBeatResponse> {
        let url = VENUE_URL.to_string() + venue + "/heartbeat";
        self.do_get(&url)
    }

    /// Get a list of all the stocks this venue can accept trades for.
    ///
    /// # Errors
    ///
    /// Errors out when:
    ///  http fails
    ///  parsing fails
    pub fn stock_list(&self, venue: &str) -> Result<StockListResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks";
        self.do_get(&url)
    }

    /// Get a copy of the venue's order book. Stockfighter suggests
    /// that this will be a slow operation that should be done
    /// as little as possible.
    pub fn orderbook(&self, venue: &str, stock: &str) -> Result<OrderbookResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock;
        self.do_get(&url)
    }

    /// Ask a venue about the current state of a stock.
    pub fn quote(&self, venue: &str, stock: &str) -> Result<QuoteResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock + "/quote";
        self.do_get(&url)
    }


    /// Send in an order, and get back a response.
    pub fn order(&self, o: &Order) -> Result<OrderResponse> {
        let url = VENUE_URL.to_string() + &o.venue + "/stocks/" + &o.stock + "/orders";
        debug!("Placing  {:?}", o);
        let encoded = try!(json::encode(o));
        let res = try!(self.http_client.post(&url, Some(&encoded)));
        parse_response(&res)
    }

    /// Find out how a specific order on a specific venue is doing.
    pub fn order_status(&self, venue: &str, stock: &str, id: u64) -> Result<OrderStatusResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock + "/orders/" + &id.to_string();
        self.do_get(&url)
    }

    /// Try and cancel an order.
    pub fn delete_order(&self, venue: &str, stock: &str, id: u64) -> Result<OrderStatusResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock + "/orders/" + &id.to_string();
        debug!("Cacneling Order {} for Stock {} at Venue {}",
               id,
               stock,
               venue);
        let status = self.do_delete::<OrderStatusResponse>(&url);
        debug!("Cancled Order  {:?}", status);
        status
    }

    fn do_get<D: Decodable>(&self, url: &str) -> Result<D> {
        let res = try!(self.http_client.get(url));
        parse_response(&res)
    }
    fn do_delete<D: Decodable>(&self, url: &str) -> Result<D> {
        let res = try!(self.http_client.delete(url));
        parse_response(&res)
    }


    /// Constructs a new level client.
    fn new(http_client: T, level: Level) -> LevelClient<T> {
        LevelClient {
            http_client: http_client.clone(),
            level: level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::HttpClient;
    use error::Result;
    use data::Level;
    use rustc_serialize::json;
    use std::collections::HashMap;


    #[derive(Debug, Clone)]
    struct TestHttpClient {
        post_result: String,
    }
    #[allow(unused_variables)]
    impl HttpClient for TestHttpClient {
        fn get(&self, url: &str) -> Result<String> {
            Ok("".to_string())
        }
        fn delete(&self, url: &str) -> Result<String> {
            Ok("".to_string())
        }
        fn post(&self, url: &str, body: Option<&str>) -> Result<String> {
            Ok(self.post_result.to_owned())
        }
    }

    #[test]
    #[should_panic]
    fn test_start_level_bad_resp() {
        let json_resp = "{}";
        let c = Client { http_client: TestHttpClient { post_result: json_resp.to_string() } };
        c.start_level("test").unwrap();
    }

    #[test]
    fn test_start_level() {
        let level = Level{
            ok: true,
            instanceId: 1090,
            account: "myac".to_string(),
            instructions: HashMap::new(),
            tickers: vec!("test".to_string()),
            venues: vec!("ven".to_string()),
        };
        let json_resp = json::encode(&level).unwrap();
        let c = Client { http_client: TestHttpClient { post_result: json_resp.to_string() } };
        c.start_level("test").unwrap();
    }
}
