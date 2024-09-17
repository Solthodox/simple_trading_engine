use trading_engine::{MarketKind, TradingEngine};
fn main() {
    let alice = "Alice".to_string();
    let bob = "Bob".to_string();
    let eur = "EUR".to_string();
    let usd = "USD".to_string();

    let eur_usd = [eur.clone(), usd.clone()];
    let mut engine = TradingEngine::new(MarketKind::OPTIONS);
    engine.add_pair(&eur_usd, 1);
    engine.add_balance(&alice, &usd, 1_000_000);
    engine.add_balance(&bob, &usd, 1_000_000);
}
