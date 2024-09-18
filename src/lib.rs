use std::any::Any;
use std::collections::HashMap;

/// Represents a trading market with basic operations.
pub trait Market {
    /// Adds a new trading pair to the market.
    ///
    /// # Arguments
    /// * `coins` - An array of two coin names representing the trading pair.
    /// * `initial_price` - The initial price for the trading pair.
    fn add_pair(&mut self, coins: &[String; 2], initial_price: u128);

    /// Returns a list of all trading pairs in the market.
    fn get_pairs(&self) -> Vec<[String; 2]>;

    /// Creates a new order in the market.
    ///
    /// # Arguments
    /// * `order_request` - A boxed trait object representing the order request.
    fn create_order(&mut self, order_request: Box<dyn OrderRequest>);

    fn get_orders(&self, coins: &[String; 2]) -> Option<&Vec<Box<dyn Order>>>;
}

/// Represents an order in the market.
pub trait Order {
    pub fn fulfill(quantity: u128) {}
}

/// Represents a request to create an order.
pub trait OrderRequest: Any {
    /// Converts the order request to a trait object of Any.
    fn as_any(&self) -> &dyn Any;
}

/// Represents a trading pair in the market.
#[derive(Clone, Debug, Default)]
struct Pair<T: Order + Clone> {
    coins: [String; 2],
    price: u128,
    orders: Vec<T>,
}

impl<T: Order + Clone> Pair<T> {
    /// Adds a new order to the pair.
    fn push_order(&mut self, order: T) {
        self.orders.push(order);
    }
}

/// Represents the side of an option order.
#[derive(Clone, Debug)]
pub enum OptionSide {
    PUT,
    CALL,
}

/// Represents an options order.
#[derive(Clone, Debug)]
struct OptionsOrder {
    strike_price: u128,
    premium: u128,
    writer: String,
    buyer: String,
    side: OptionSide,
    expiry: u128,
    quantity: u128,
}

impl Order for OptionsOrder {}

/// Represents a request to create an options order.
#[derive(Debug, Clone)]
struct OptionsOrderRequest {
    user: String,
    coins: [String; 2],
    side: OptionSide,
    strike_price: u128,
    premium: u128,
    expiry: u128,
}

impl OrderRequest for OptionsOrderRequest {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents an options trading market.
#[derive(Clone, Debug, Default)]
pub struct OptionsMarket {
    all_pairs: Vec<Pair<OptionsOrder>>,
    pairs_map: HashMap<[String; 2], Pair<OptionsOrder>>,
}

impl Market for OptionsMarket {
    fn add_pair(&mut self, coins: &[String; 2], initial_price: u128) {
        let pair = Pair {
            coins: coins.clone(),
            price: initial_price,
            orders: vec![],
        };
        self.all_pairs.push(pair.clone());
        self.pairs_map.insert(coins.clone(), pair);
    }

    fn get_pairs(&self) -> Vec<[String; 2]> {
        self.pairs_map.keys().cloned().collect()
    }

    fn create_order(&mut self, order_request: Box<dyn OrderRequest>) {
        if let Some(request) = order_request.as_any().downcast_ref::<OptionsOrderRequest>() {
            let pair = self
                .pairs_map
                .get_mut(&request.coins)
                .expect("Pair not found");
            pair.push_order(OptionsOrder {
                strike_price: request.strike_price,
                premium: request.premium,
                writer: request.user.clone(),
                buyer: String::new(),
                side: request.side.clone(),
                expiry: request.expiry,
                quantity: 0,
            });
        } else {
            panic!("Invalid order request type");
        }
    }

    fn get_orders(&self, coins: &[String; 2]) -> Option<&Vec<Box<dyn Order>>> {
        self.pairs_map.get(coins).map(|pair| &pair.orders)
    }
}

/// Represents the side of a futures order.
#[derive(Clone, Debug)]
pub enum FuturesSide {
    BID,
    ASK,
}

/// Represents a futures order.
#[derive(Clone, Debug)]
struct FuturesOrder {
    price: u128,
    writer: String,
    buyer: String,
    side: FuturesSide,
    expiry: u128,
    quantity: u128,
}

impl Order for FuturesOrder {}

/// Represents a request to create a futures order.
#[derive(Debug, Clone)]
struct FuturesOrderRequest {
    user: String,
    coins: [String; 2],
    side: FuturesSide,
    price: u128,
    expiry: u128,
}

impl OrderRequest for FuturesOrderRequest {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents a futures trading market.
#[derive(Clone, Debug, Default)]
pub struct FuturesMarket {
    all_pairs: Vec<Pair<OptionsOrder>>,
    pairs_map: HashMap<[String; 2], Pair<FuturesOrder>>,
}

impl Market for FuturesMarket {
    fn add_pair(&mut self, coins: &[String; 2], initial_price: u128) {
        let pair = Pair {
            coins: coins.clone(),
            price: initial_price,
            orders: vec![],
        };
        self.all_pairs.push(pair.clone());
        self.pairs_map.insert(coins.clone(), pair);
    }

    fn get_pairs(&self) -> Vec<[String; 2]> {
        self.pairs_map.keys().cloned().collect()
    }

    fn create_order(&mut self, order_request: Box<dyn OrderRequest>) {
        if let Some(request) = order_request.as_any().downcast_ref::<FuturesOrderRequest>() {
            let pair = self
                .pairs_map
                .get_mut(&request.coins)
                .expect("Pair not found");
            pair.push_order(FuturesOrder {
                price: request.price,
                writer: request.user.clone(),
                buyer: String::new(),
                side: request.side.clone(),
                expiry: request.expiry,
                quantity: 0,
            });
        } else {
            panic!("Invalid order request type");
        }
    }

    fn get_orders(&self, coins: &[String; 2]) -> Option<&Vec<Box<dyn Order>>> {
        self.pairs_map.get(coins).map(|pair| &pair.orders)
    }
}

/// Represents the type of market.
pub enum MarketKind {
    OPTIONS,
    FUTURES,
}

/// Main struct representing the trading engine.
pub struct TradingEngine {
    market: Box<dyn Market>,
    balances: HashMap<String, HashMap<String, u128>>,
}

impl TradingEngine {
    /// Creates a new TradingEngine with the specified market kind.
    ///
    /// # Arguments
    /// * `params` - The kind of market to create (OPTIONS or FUTURES).
    pub fn new(params: MarketKind) -> TradingEngine {
        match params {
            MarketKind::FUTURES => TradingEngine {
                market: Box::new(FuturesMarket::default()),
                balances: HashMap::new(),
            },
            MarketKind::OPTIONS => TradingEngine {
                market: Box::new(OptionsMarket::default()),
                balances: HashMap::new(),
            },
        }
    }

    /// Adds a new trading pair to the market.
    ///
    /// # Arguments
    /// * `coins` - An array of two coin names representing the trading pair.
    /// * `initial_price` - The initial price for the trading pair.
    pub fn add_pair(&mut self, coins: &[String; 2], initial_price: u128) {
        self.market.add_pair(coins, initial_price);
    }

    /// Adds balance for a user's coin.
    ///
    /// # Arguments
    /// * `user` - The user's identifier.
    /// * `coin` - The coin to add balance for.
    /// * `amount` - The amount to add to the balance.
    pub fn add_balance(&mut self, user: &String, coin: &String, amount: u128) {
        let user_balances = self
            .balances
            .entry(user.clone())
            .or_insert_with(HashMap::new);

        let balance = user_balances.entry(coin.clone()).or_insert(0);
        *balance += amount;
    }

    pub fn get_orders<T: Order>(&self, coins: &[String; 2]) -> Vec<Pair<T>> {}
}
