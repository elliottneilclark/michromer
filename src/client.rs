use error::Result;
use http::AuthHttpClient;
use rustc_serialize::json;
use data::{Level, HeartBeatResponse, VenueHeartBeatResponse, StockListResponse, OrderbookResponse,
           QuoteResponse, Order, OrderResponse, OrderStatusResponse};

static VENUE_URL: &'static str = "https://api.stockfighter.io/ob/api/venues/";
static HEARTBEAT_URL: &'static str = "https://api.stockfighter.io/ob/api/heartbeat";

pub struct Client {
    http_client: AuthHttpClient,
}

impl Client {
    pub fn start_level(&self, level: &str) -> Result<LevelClient> {
        // Start a level
        let url = "https://www.stockfighter.io/gm/levels/".to_string() + level;
        let level: Level = try!(self.http_client.post(&url, None));
        // Give it back.
        Ok(LevelClient::new(self.http_client.clone(), level))
    }
    pub fn new(api_key: &str) -> Client {
        Client { http_client: AuthHttpClient::new(api_key) }
    }
}

#[derive(Debug)]
pub struct LevelClient {
    http_client: AuthHttpClient,
    pub level: Level,
}

impl LevelClient {
    pub fn heart_beat(&self) -> Result<HeartBeatResponse> {
        let url = HEARTBEAT_URL;
        self.http_client.get::<HeartBeatResponse>(url)
    }
    pub fn venue_heart_beat(&self, venue: &str) -> Result<VenueHeartBeatResponse> {
        let url = VENUE_URL.to_string() + venue + "/heartbeat";
        self.http_client.get::<VenueHeartBeatResponse>(&url)
    }
    pub fn stock_list(&self, venue: &str) -> Result<StockListResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks";
        self.http_client.get::<StockListResponse>(&url)
    }
    pub fn orderbook(&self, venue: &str, stock: &str) -> Result<OrderbookResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock;
        self.http_client.get::<OrderbookResponse>(&url)
    }
    pub fn quote(&self, venue: &str, stock: &str) -> Result<QuoteResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock + "/quote";
        self.http_client.get::<QuoteResponse>(&url)
    }
    pub fn order(&self, o: &Order) -> Result<OrderResponse> {
        let url = VENUE_URL.to_string() + &o.venue + "/stocks/" + &o.stock + "/orders";
        debug!("Placing  {:?}", o);
        let encoded = try!(json::encode(o));
        let resp = self.http_client.post(&url, Some(&encoded));
        debug!("Placed {:?}", resp);
        resp
    }
    pub fn order_status(&self, venue: &str, stock: &str, id: u64) -> Result<OrderStatusResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock + "/orders/" + &id.to_string();
        self.http_client.get::<OrderStatusResponse>(&url)
    }
    pub fn delete_order(&self, venue: &str, stock: &str, id: u64) -> Result<OrderStatusResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock + "/orders/" + &id.to_string();
        debug!("Cacneling Order {} for Stock {} at Venue {}",
               id,
               stock,
               venue);
        let status = self.http_client.delete::<OrderStatusResponse>(&url);
        debug!("Cancled Order  {:?}", status);
        status
    }
    fn new(http_client: AuthHttpClient, level: Level) -> LevelClient {
        LevelClient {
            http_client: http_client.clone(),
            level: level,
        }
    }
}
