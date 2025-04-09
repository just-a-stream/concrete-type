use crate::{
    exchanges::{Binance, Okx},
    strategies::{StrategyA, StrategyB},
};
use concrete_type::Concrete;
use std::marker::PhantomData;

#[derive(Concrete, Clone, Copy)]
enum Exchange {
    #[concrete = "exchanges::Binance"]
    Binance,
    #[concrete = "exchanges::Okx"]
    Okx,
}

#[derive(Concrete)]
enum Strategy {
    #[concrete = "strategies::StrategyA"]
    StrategyA,

    #[concrete = "strategies::StrategyB"]
    StrategyB,
}

trait ExchangeApi {
    fn new() -> Self;
    fn name(&self) -> &'static str;
}

pub trait TradingStrategy {
    fn name() -> &'static str;
}

fn main() {
    let exchange = Exchange::Binance;
    let strategy = Strategy::StrategyA;

    let name = exchange!(exchange; Exchange => {
        strategy!(strategy; Strategy => {
            TradingSystem::<Exchange, Strategy>::new().name()
        })
    });
    assert_eq!(name, "binance_strategy_a");

    let exchange = Exchange::Okx;
    let strategy = Strategy::StrategyB;
    let name = trading_system!(exchange, strategy; (Exchange, Strategy) => {
       TradingSystem::<Exchange, Strategy>::new().name()
    });
    assert_eq!(name, "okx_strategy_b");
}

mod exchanges {
    use crate::ExchangeApi;

    pub struct Binance;

    impl ExchangeApi for Binance {
        fn new() -> Self {
            Binance
        }

        fn name(&self) -> &'static str {
            "SomeBinanceName"
        }
    }

    pub struct Okx;

    impl ExchangeApi for Okx {
        fn new() -> Self {
            Self
        }

        fn name(&self) -> &'static str {
            "SomeOkxName"
        }
    }
}

pub mod strategies {
    use crate::TradingStrategy;

    pub struct StrategyA;

    impl TradingStrategy for StrategyA {
        fn name() -> &'static str {
            "strategy_a"
        }
    }

    pub struct StrategyB;

    impl TradingStrategy for StrategyB {
        fn name() -> &'static str {
            "strategy_b"
        }
    }
}

// Concrete on an enum with type parameters will create a macro_rules in the following format:
// trading_system_config!(exchange_enum, strategy_enum; (Exchange, Strategy) => { ... })
#[derive(Concrete)]
pub struct TradingSystem<Exchange, Strategy> {
    phantom: PhantomData<(Exchange, Strategy)>,
}

impl TradingSystem<Binance, StrategyA> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
        }
    }

    pub fn name(&self) -> &'static str {
        "binance_strategy_a"
    }
}

impl TradingSystem<Binance, StrategyB> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
        }
    }

    pub fn name(&self) -> &'static str {
        "binance_strategy_b"
    }
}

impl TradingSystem<Okx, StrategyA> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
        }
    }

    pub fn name(&self) -> &'static str {
        "okx_strategy_a"
    }
}

impl TradingSystem<Okx, StrategyB> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
        }
    }

    pub fn name(&self) -> &'static str {
        "okx_strategy_b"
    }
}
