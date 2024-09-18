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
    fn create_order(&mut self, order_request: Box<dyn OrderRequest>) -> Result<(), String>;

    /// Retrieves orders for a specific trading pair.
    ///
    /// # Arguments
    /// * `coins` - An array of two coin names representing the trading pair.
    fn get_orders(&self, coins: &[String; 2]) -> Option<&Vec<Box<dyn Order>>>;
}

/// Represents an order in the market.
pub trait Order: Any {
    fn fulfill(&mut self, buyer: String, quantity: u128);
    fn get_coins(&self) -> &[String; 2];
    fn get_price(&self) -> u128;
    fn get_writer(&self) -> &str;
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
    coins: [String; 2],
    strike_price: u128,
    premium: u128,
    writer: String,
    buyer: String,
    side: OptionSide,
    expiry: u128,
    quantity: u128,
}

impl Order for OptionsOrder {
    fn fulfill(&mut self, buyer: String, quantity: u128) {
        self.buyer = buyer;
        self.quantity += quantity;
    }

    fn get_coins(&self) -> &[String; 2] {
        &self.coins
    }

    fn get_price(&self) -> u128 {
        self.premium
    }

    fn get_writer(&self) -> &str {
        &self.writer
    }
}

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

    fn create_order(&mut self, order_request: Box<dyn OrderRequest>) -> Result<(), String> {
        if let Some(request) = order_request.as_any().downcast_ref::<OptionsOrderRequest>() {
            let pair = self
                .pairs_map
                .get_mut(&request.coins)
                .ok_or_else(|| "Pair not found".to_string())?;
            pair.push_order(OptionsOrder {
                coins: request.coins.clone(),
                strike_price: request.strike_price,
                premium: request.premium,
                writer: request.user.clone(),
                buyer: String::new(),
                side: request.side.clone(),
                expiry: request.expiry,
                quantity: 0,
            });
            Ok(())
        } else {
            Err("Invalid order request type".to_string())
        }
    }

    fn get_orders(&self, coins: &[String; 2]) -> Option<&Vec<Box<dyn Order>>> {
        None // This needs to be implemented properly
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
    coins: [String; 2],
    price: u128,
    writer: String,
    buyer: String,
    side: FuturesSide,
    expiry: u128,
    quantity: u128,
}

impl Order for FuturesOrder {
    fn fulfill(&mut self, buyer: String, quantity: u128) {
        self.buyer = buyer;
        self.quantity += quantity;
    }

    fn get_coins(&self) -> &[String; 2] {
        &self.coins
    }

    fn get_price(&self) -> u128 {
        self.price
    }

    fn get_writer(&self) -> &str {
        &self.writer
    }
}

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
    all_pairs: Vec<Pair<FuturesOrder>>,
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

    fn create_order(&mut self, order_request: Box<dyn OrderRequest>) -> Result<(), String> {
        if let Some(request) = order_request.as_any().downcast_ref::<FuturesOrderRequest>() {
            let pair = self
                .pairs_map
                .get_mut(&request.coins)
                .ok_or_else(|| "Pair not found".to_string())?;
            pair.push_order(FuturesOrder {
                coins: request.coins.clone(),
                price: request.price,
                writer: request.user.clone(),
                buyer: String::new(),
                side: request.side.clone(),
                expiry: request.expiry,
                quantity: 0,
            });
            Ok(())
        } else {
            Err("Invalid order request type".to_string())
        }
    }

    fn get_orders(&self, coins: &[String; 2]) -> Option<&Vec<Box<dyn Order>>> {
        None // This needs to be implemented properly
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
    pub fn add_pair(&mut self, coins: &[String; 2], initial_price: u128) -> Result<(), String> {
        self.market.add_pair(coins, initial_price);
        Ok(())
    }

    /// Adds balance for a user's coin.
    ///
    /// # Arguments
    /// * `user` - The user's identifier.
    /// * `coin` - The coin to add balance for.
    /// * `amount` - The amount to add to the balance.
    pub fn add_balance(&mut self, user: &str, coin: &str, amount: u128) -> Result<(), String> {
        let user_balances = self
            .balances
            .entry(user.to_string())
            .or_insert_with(HashMap::new);

        let balance = user_balances.entry(coin.to_string()).or_insert(0);
        *balance = balance.checked_add(amount).ok_or("Balance overflow")?;
        Ok(())
    }

    /// Subtracts balance from a user's coin.
    ///
    /// # Arguments
    /// * `user` - The user's identifier.
    /// * `coin` - The coin to subtract balance from.
    /// * `amount` - The amount to subtract from the balance.
    pub fn subtract_balance(&mut self, user: &str, coin: &str, amount: u128) -> Result<(), String> {
        let user_balances = self.balances.get_mut(user).ok_or("User not found")?;

        let balance = user_balances.get_mut(coin).ok_or("Coin not found")?;
        *balance = balance.checked_sub(amount).ok_or("Insufficient balance")?;
        Ok(())
    }

    /// Gets orders for a specific trading pair.
    ///
    /// # Arguments
    /// * `coins` - An array of two coin names representing the trading pair.
    pub fn get_orders(&self, coins: &[String; 2]) -> Result<&Vec<Box<dyn Order>>, String> {
        self.market
            .get_orders(coins)
            .ok_or("Pair not found".to_string())
    }

    /// Fulfills an order.
    ///
    /// # Arguments
    /// * `order` - The order to fulfill.
    /// * `user` - The user fulfilling the order.
    /// * `quantity` - The quantity to fulfill.
    pub fn fulfill_order(
        &mut self,
        order: &mut Box<dyn Order>,
        user: &str,
        quantity: u128,
    ) -> Result<(), String> {
        let payment_amount = quantity
            .checked_mul(order.get_price())
            .ok_or("Overflow in payment calculation")?;
        let payment_coin = &order.get_coins()[1];
        self.subtract_balance(user, payment_coin, payment_amount)?;
        self.add_balance(order.get_writer(), payment_coin, payment_amount)?;
        order.fulfill(user.to_string(), quantity);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_pair() {
        let mut engine = TradingEngine::new(MarketKind::OPTIONS);
        let coins = ["BTC".to_string(), "USD".to_string()];
        assert!(engine.add_pair(&coins, 50000).is_ok());
        assert_eq!(engine.market.get_pairs().len(), 1);
    }

    #[test]
    fn test_add_balance() {
        let mut engine = TradingEngine::new(MarketKind::OPTIONS);
        assert!(engine.add_balance("Alice", "BTC", 100).is_ok());
        assert!(engine.add_balance("Alice", "BTC", 50).is_ok());
        let balance = engine.balances.get("Alice").unwrap().get("BTC").unwrap();
        assert_eq!(*balance, 150);
    }

    #[test]
    fn test_subtract_balance() {
        let mut engine = TradingEngine::new(MarketKind::OPTIONS);
        assert!(engine.add_balance("Bob", "ETH", 100).is_ok());
        assert!(engine.subtract_balance("Bob", "ETH", 30).is_ok());
        let balance = engine.balances.get("Bob").unwrap().get("ETH").unwrap();
        assert_eq!(*balance, 70);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut engine = TradingEngine::new(MarketKind::OPTIONS);
        assert!(engine.add_balance("Charlie", "USDT", 50).is_ok());
        assert!(engine.subtract_balance("Charlie", "USDT", 100).is_err());
    }

    #[test]
    fn test_create_order() {
        let mut engine = TradingEngine::new(MarketKind::OPTIONS);
        let coins = ["ETH".to_string(), "USD".to_string()];
        assert!(engine.add_pair(&coins, 3000).is_ok());

        let order_request = Box::new(OptionsOrderRequest {
            user: "Dave".to_string(),
            coins: coins.clone(),
            side: OptionSide::CALL,
            strike_price: 3500,
            premium: 100,
            expiry: 1630000000,
        });

        assert!(engine.market.create_order(order_request).is_ok());
    }

    #[test]
    fn test_futures_market_creation() {
        let engine = TradingEngine::new(MarketKind::FUTURES);
        assert!(matches!(
            engine.market.as_any().downcast_ref::<FuturesMarket>(),
            Some(_)
        ));
    }

    #[test]
    fn test_options_market_creation() {
        let engine = TradingEngine::new(MarketKind::OPTIONS);
        assert!(matches!(
            engine.market.as_any().downcast_ref::<OptionsMarket>(),
            Some(_)
        ));
    }

    #[test]
    fn test_add_multiple_pairs() {
        let mut engine = TradingEngine::new(MarketKind::OPTIONS);
        let pairs = vec![
            (["BTC".to_string(), "USD".to_string()], 50000),
            (["ETH".to_string(), "USD".to_string()], 3000),
            (["XRP".to_string(), "USD".to_string()], 1),
        ];

        for (coins, price) in pairs {
            assert!(engine.add_pair(&coins, price).is_ok());
        }

        assert_eq!(engine.market.get_pairs().len(), 3);
    }

    #[test]
    fn test_add_balance_multiple_users_and_coins() {
        let mut engine = TradingEngine::new(MarketKind::FUTURES);

        assert!(engine.add_balance("Alice", "BTC", 1000).is_ok());
        assert!(engine.add_balance("Alice", "ETH", 50).is_ok());
        assert!(engine.add_balance("Bob", "USD", 100000).is_ok());

        assert_eq!(
            *engine.balances.get("Alice").unwrap().get("BTC").unwrap(),
            1000
        );
        assert_eq!(
            *engine.balances.get("Alice").unwrap().get("ETH").unwrap(),
            50
        );
        assert_eq!(
            *engine.balances.get("Bob").unwrap().get("USD").unwrap(),
            100000
        );
    }

    #[test]
    fn test_subtract_balance_error_handling() {
        let mut engine = TradingEngine::new(MarketKind::OPTIONS);

        assert!(engine.add_balance("Charlie", "USDT", 1000).is_ok());

        // Test subtracting more than available balance
        assert!(engine.subtract_balance("Charlie", "USDT", 1001).is_err());

        // Test subtracting from non-existent user
        assert!(engine.subtract_balance("David", "USDT", 100).is_err());

        // Test subtracting non-existent coin
        assert!(engine.subtract_balance("Charlie", "BTC", 1).is_err());
    }

    #[test]
    fn test_create_and_fulfill_futures_order() {
        let mut engine = TradingEngine::new(MarketKind::FUTURES);
        let coins = ["BTC".to_string(), "USD".to_string()];
        assert!(engine.add_pair(&coins, 50000).is_ok());

        let order_request = Box::new(FuturesOrderRequest {
            user: "Alice".to_string(),
            coins: coins.clone(),
            side: FuturesSide::ASK,
            price: 52000,
            expiry: 1630000000,
        });

        assert!(engine.market.create_order(order_request).is_ok());

        // Add balances for users
        assert!(engine.add_balance("Alice", "BTC", 1).is_ok());
        assert!(engine.add_balance("Bob", "USD", 52000).is_ok());

        // Get the created order and fulfill it
        if let Some(orders) = engine.market.get_orders(&coins) {
            if let Some(order) = orders.last() {
                let mut order_clone = order.clone();
                assert!(engine.fulfill_order(&mut order_clone, "Bob", 1).is_ok());

                // Check balances after fulfillment
                assert_eq!(
                    *engine.balances.get("Alice").unwrap().get("USD").unwrap(),
                    52000
                );
                assert_eq!(*engine.balances.get("Bob").unwrap().get("USD").unwrap(), 0);
            } else {
                panic!("No orders found");
            }
        } else {
            panic!("No orders found for the given pair");
        }
    }

    #[test]
    fn test_order_fulfillment_with_insufficient_balance() {
        let mut engine = TradingEngine::new(MarketKind::OPTIONS);
        let coins = ["ETH".to_string(), "USD".to_string()];
        assert!(engine.add_pair(&coins, 3000).is_ok());

        let order_request = Box::new(OptionsOrderRequest {
            user: "Charlie".to_string(),
            coins: coins.clone(),
            side: OptionSide::CALL,
            strike_price: 3200,
            premium: 100,
            expiry: 1630000000,
        });

        assert!(engine.market.create_order(order_request).is_ok());

        // Add insufficient balance for Dave
        assert!(engine.add_balance("Dave", "USD", 50).is_ok());

        // Try to fulfill the order with insufficient balance
        if let Some(orders) = engine.market.get_orders(&coins) {
            if let Some(order) = orders.last() {
                let mut order_clone = order.clone();
                assert!(engine.fulfill_order(&mut order_clone, "Dave", 1).is_err());
            } else {
                panic!("No orders found");
            }
        } else {
            panic!("No orders found for the given pair");
        }
    }
}
