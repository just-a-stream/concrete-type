use crate::{
    exchanges::{Binance, Okx},
    strategies::{StrategyA, StrategyB},
};
use concrete_type::Concrete;
use std::marker::PhantomData;

#[derive(Concrete, Clone, Copy)]
enum Exchange {
    #[concrete = "crate::exchanges::Binance"]
    Binance,
    #[concrete = "crate::exchanges::Okx"]
    Okx,
    #[concrete = "crate::exchanges::Kraken<crate::exchanges::KrakenSpotServer>"]
    Kraken,
}

mod exchanges {
    pub struct Binance;

    pub struct Okx;

    pub struct KrakenSpotServer;
    pub struct Kraken<Server> {
        pub _phantom: std::marker::PhantomData<Server>,
    }
}

#[derive(Concrete)]
enum Strategy {
    #[concrete = "crate::strategies::StrategyA"]
    StrategyA,

    #[concrete = "crate::strategies::StrategyB"]
    StrategyB,
}

pub mod strategies {
    pub struct StrategyA;

    pub struct StrategyB;
}

pub struct TradingSystem<Exchange, Strategy> {
    phantom: PhantomData<(Exchange, Strategy)>,
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

    let name = exchange!(exchange; Exchange => {
        strategy!(strategy; Strategy => {
            TradingSystem::<Exchange, Strategy>::new().name()
        })
    });
    assert_eq!(name, "okx_strategy_b");

    let exchange = Exchange::Kraken;
    let strategy = Strategy::StrategyA;

    let name = exchange!(exchange; Exchange => {
        strategy!(strategy; Strategy => {
            TradingSystem::<Exchange, Strategy>::new().name()
        })
    });
    assert_eq!(name, "kraken_strategy_a");
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

use crate::exchanges::{Kraken, KrakenSpotServer};

impl TradingSystem<Kraken<KrakenSpotServer>, StrategyA> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
        }
    }

    pub fn name(&self) -> &'static str {
        "kraken_strategy_a"
    }
}

impl TradingSystem<Kraken<KrakenSpotServer>, StrategyB> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
        }
    }

    pub fn name(&self) -> &'static str {
        "kraken_strategy_b"
    }
}
