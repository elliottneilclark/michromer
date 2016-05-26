use error::Result;
use http::AuthHttpClient;
use rustc_serialize::json;
use data::{Level, HeartBeatResponse, VenueHeartBeatResponse, StockListResponse, OrderbookResponse,
           QuoteResponse, Order, OrderResponse, OrderStatusResponse};

static VENUE_URL: &'static str = "https://api.stockfighter.io/ob/api/venues/";
static HEARTBEAT_URL: &'static str = "https://api.stockfighter.io/ob/api/heartbeat";

/// Client for starting a new level of Stockfighter.
pub struct Client {
    http_client: AuthHttpClient,
}

impl Client {

    /// Start a new level
    ///
    /// It appears that this will also continue a current level
    /// if there is already a level ongoing.
    pub fn start_level(&self, level: &str) -> Result<LevelClient> {
        // Start a level
        let url = "https://www.stockfighter.io/gm/levels/".to_string() + level;
        let level: Level = try!(self.http_client.post(&url, None));
        // Give it back.
        Ok(LevelClient::new(self.http_client.clone(), level))
    }

    /// Given an api key construct a new Client that can interact with stockfighter's game api.
    pub fn new(api_key: &str) -> Client {
        Client { http_client: AuthHttpClient::new(api_key) }
    }
}

/// Stockfighter client for a specific level. The
/// stock api is defined [here](https://starfighter.readme.io/docs)
/// It's wrapper around an http client. As such it can return
/// errors for parsing or for network issues.
#[derive(Debug)]
pub struct LevelClient {
    http_client: AuthHttpClient,
    pub level: Level,
}

impl LevelClient {
    /// See if this Level is up.
    ///
    /// This should really only be used to sanity check that the level
    /// hasn't been torn down.
    pub fn heart_beat(&self) -> Result<HeartBeatResponse> {
        let url = HEARTBEAT_URL;
        self.http_client.get::<HeartBeatResponse>(url)
    }

    /// Check if a venue is ok.
    pub fn venue_heart_beat(&self, venue: &str) -> Result<VenueHeartBeatResponse> {
        let url = VENUE_URL.to_string() + venue + "/heartbeat";
        self.http_client.get::<VenueHeartBeatResponse>(&url)
    }

    /// Get a list of all the stocks this venue can accept trades for.
    ///
    /// # Errors
    ///
    /// Errors out when:
    /// - http fails
    /// - parsing fails
    pub fn stock_list(&self, venue: &str) -> Result<StockListResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks";
        self.http_client.get::<StockListResponse>(&url)
    }

    /// Get a copy of the venue's order book. Stockfighter suggests
    /// that this will be a slow operation that should be done
    /// as little as possible.
    pub fn orderbook(&self, venue: &str, stock: &str) -> Result<OrderbookResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock;
        self.http_client.get::<OrderbookResponse>(&url)
    }

    /// Ask a venue about the current state of a stock.
    pub fn quote(&self, venue: &str, stock: &str) -> Result<QuoteResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock + "/quote";
        self.http_client.get::<QuoteResponse>(&url)
    }


    /// Send in an order, and get back a response.
    pub fn order(&self, o: &Order) -> Result<OrderResponse> {
        let url = VENUE_URL.to_string() + &o.venue + "/stocks/" + &o.stock + "/orders";
        debug!("Placing  {:?}", o);
        let encoded = try!(json::encode(o));
        let resp = self.http_client.post(&url, Some(&encoded));
        debug!("Placed {:?}", resp);
        resp
    }

    /// Find out how a specific order on a specific venue is doing.
    pub fn order_status(&self, venue: &str, stock: &str, id: u64) -> Result<OrderStatusResponse> {
        let url = VENUE_URL.to_string() + venue + "/stocks/" + stock + "/orders/" + &id.to_string();
        self.http_client.get::<OrderStatusResponse>(&url)
    }

    /// Try and cancel an order.
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


    /// Constructs a new level client.
    fn new(http_client: AuthHttpClient, level: Level) -> LevelClient {
        LevelClient {
            http_client: http_client.clone(),
            level: level,
        }
    }
}
