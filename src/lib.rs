use std::collections::HashMap;

pub trait Market {
    fn add_pair(&mut self, coins: &[String; 2], initial_price: u128);
    fn get_pairs(&self) -> Vec<[String; 2]>;
}

pub trait Order {}

#[derive(Clone, Debug, Default)]
struct Pair<T: Order + Clone> {
    coins: [String; 2],
    price: u128,
    orders: Vec<T>,
}

impl<T: Order + Clone> Pair<T> {
    fn push_order(&mut self, order: T) {
        self.orders.push(order);
    }
}

#[derive(Clone, Debug)]
pub enum OptionSide {
    PUT,
    CALL,
}

#[derive(Clone, Debug)]
struct OptionsOrder {
    strike_price: u128,
    premium: u128,
    writer: String,
    buyer: String,
    side: OptionSide,
    deadline: u128,
}

impl Order for OptionsOrder {}

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
}

impl OptionsMarket {
    fn write_option(
        &mut self,
        user: &String,
        coins: &[String; 2],
        side: OptionSide,
        strike_price: u128,
        premium: u128,
        deadline: u128,
    ) {
        let pair = self.pairs_map.get_mut(coins).ok_or("Pair not found");
        pair.expect("Pair not found").push_order(OptionsOrder {
            strike_price,
            premium,
            writer: user.clone(),
            buyer: String::new(),
            side,
            deadline,
        });
    }
}

#[derive(Clone, Debug, Default)]
pub struct FuturesMarket {
    all_pairs: Vec<Pair<OptionsOrder>>,
    pairs_map: HashMap<[String; 2], Pair<OptionsOrder>>,
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
}

pub enum MarketKind {
    OPTIONS,
    FUTURES,
}

pub struct TradingEngine {
    market: Box<dyn Market>,
    balances: HashMap<String, HashMap<String, u128>>,
}

impl TradingEngine {
    pub fn new(params: MarketKind) -> TradingEngine {
        let market: Box<dyn Market> = match params {
            MarketKind::FUTURES => Box::new(FuturesMarket::default()),
            MarketKind::OPTIONS => Box::new(OptionsMarket::default()),
        };
        TradingEngine {
            market,
            balances: HashMap::new(),
        }
    }

    // Update other methods to work with Box<dyn Market> instead of T
    pub fn add_pair(&mut self, coins: &[String; 2], initial_price: u128) {
        self.market.add_pair(coins, initial_price);
    }

    pub fn add_balance(&mut self, user: &String, coin: &String, amount: u128) {
        // Access or create the user's balance map
        let user_balances = self
            .balances
            .entry(user.clone())
            .or_insert_with(HashMap::new);

        // Add or update the coin balance
        let balance = user_balances.entry(coin.clone()).or_insert(0);
        *balance += amount;
    }

    pub fn get_market(&self) -> &dyn Market {
        &*self.market
    }
}
