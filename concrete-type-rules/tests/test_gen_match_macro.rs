use concrete_type::Concrete;
use concrete_type_rules::gen_match_concretes_macro;
use std::marker::PhantomData;

// Define our enums with Concrete derive for testing
#[derive(Concrete, Clone, Copy)]
enum Exchange {
    #[concrete = "test_types::Binance"]
    Binance,
    #[concrete = "test_types::Okx"]
    Okx,
}

#[derive(Concrete, Clone, Copy)]
enum Strategy {
    #[concrete = "test_types::StrategyA"]
    StrategyA,
    #[concrete = "test_types::StrategyB"]
    StrategyB,
}

#[derive(Concrete, Clone, Copy)]
enum TimeFrame {
    #[concrete = "test_types::Minute"]
    Minute,
    #[concrete = "test_types::Hour"]
    Hour,
}

#[derive(Concrete, Clone, Copy)]
enum Market {
    #[concrete = "test_types::Spot"]
    Spot,
    #[concrete = "test_types::Futures"]
    Futures,
}

#[derive(Concrete, Clone, Copy)]
enum RiskLevel {
    #[concrete = "test_types::Low"]
    Low,
    #[concrete = "test_types::High"]
    High,
}

// All our concrete types in a test-specific module
mod test_types {
    pub struct Binance;
    pub struct Okx;
    pub struct StrategyA;
    pub struct StrategyB;
    pub struct Minute;
    pub struct Hour;
    pub struct Spot;
    pub struct Futures;
    pub struct Low;
    pub struct High;
}

// Define system structs for testing
struct DualSystem<E, S> {
    phantom: PhantomData<(E, S)>,
}

struct TripleSystem<E, S, T> {
    phantom: PhantomData<(E, S, T)>,
}

struct QuadSystem<E, S, T, M> {
    phantom: PhantomData<(E, S, T, M)>,
}

struct QuintSystem<E, S, T, M, R> {
    phantom: PhantomData<(E, S, T, M, R)>,
}

// Implement specific combinations for testing
impl DualSystem<test_types::Binance, test_types::StrategyA> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "binance_strategy_a"
    }
}

impl DualSystem<test_types::Binance, test_types::StrategyB> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "binance_strategy_b"
    }
}

impl DualSystem<test_types::Okx, test_types::StrategyA> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "okx_strategy_a"
    }
}

impl DualSystem<test_types::Okx, test_types::StrategyB> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "okx_strategy_b"
    }
}

impl TripleSystem<test_types::Binance, test_types::StrategyA, test_types::Minute> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "binance_strategy_a_minute"
    }
}

impl TripleSystem<test_types::Binance, test_types::StrategyB, test_types::Minute> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "binance_strategy_b_minute"
    }
}

impl TripleSystem<test_types::Binance, test_types::StrategyA, test_types::Hour> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "binance_strategy_a_hour"
    }
}

impl TripleSystem<test_types::Binance, test_types::StrategyB, test_types::Hour> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "binance_strategy_b_hour"
    }
}

impl TripleSystem<test_types::Okx, test_types::StrategyA, test_types::Minute> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "okx_strategy_a_minute"
    }
}

impl TripleSystem<test_types::Okx, test_types::StrategyB, test_types::Minute> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "okx_strategy_b_minute"
    }
}

impl TripleSystem<test_types::Okx, test_types::StrategyA, test_types::Hour> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "okx_strategy_a_hour"
    }
}

impl TripleSystem<test_types::Okx, test_types::StrategyB, test_types::Hour> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        "okx_strategy_b_hour"
    }
}

// impl QuadSystem<test_types::Binance, test_types::StrategyA, test_types::Minute, test_types::Spot> {
//     pub fn new() -> Self {
//         Self {
//             phantom: PhantomData,
//         }
//     }
//
//     pub fn name(&self) -> &'static str {
//         "binance_strategy_a_minute_spot"
//     }
// }
//
// impl
//     QuintSystem<
//         test_types::Binance,
//         test_types::StrategyA,
//         test_types::Minute,
//         test_types::Spot,
//         test_types::Low,
//     >
// {
//     pub fn new() -> Self {
//         Self {
//             phantom: PhantomData,
//         }
//     }
//
//     pub fn name(&self) -> &'static str {
//         "binance_strategy_a_minute_spot_low_risk"
//     }
// }

// Generate the macro combinations for testing
gen_match_concretes_macro!(Exchange, Strategy);
gen_match_concretes_macro!(Exchange, Strategy, TimeFrame);
// gen_match_concretes_macro!(Exchange, Strategy, TimeFrame, Market);
// gen_match_concretes_macro!(Exchange, Strategy, TimeFrame, Market, RiskLevel);

#[test]
fn test_two_enum_match() {
    let exchange = Exchange::Binance;
    let strategy = Strategy::StrategyA;

    let result = match_exchange_strategy!(
        exchange, strategy; E, S => {
            let system = DualSystem::<E, S>::new();
            system.name()
        }
    );

    assert_eq!(result, "binance_strategy_a");

    let exchange = Exchange::Okx;
    let strategy = Strategy::StrategyB;

    let result = match_exchange_strategy!(
        exchange, strategy; E, S => {
            let system = DualSystem::<E, S>::new();
            system.name()
        }
    );

    assert_eq!(result, "okx_strategy_b");
}

#[test]
fn test_three_enum_match() {
    let exchange = Exchange::Binance;
    let strategy = Strategy::StrategyA;
    let timeframe = TimeFrame::Minute;

    let result = match_exchange_strategy_time_frame!(
        exchange, strategy, timeframe; E, S, T => {
            let system = TripleSystem::<E, S, T>::new();
            system.name()
        }
    );

    assert_eq!(result, "binance_strategy_a_minute");
}

// #[test]
// fn test_four_enum_match() {
//     let exchange = Exchange::Binance;
//     let strategy = Strategy::StrategyA;
//     let timeframe = TimeFrame::Minute;
//     let market = Market::Spot;
//
//     let result = match_exchange_strategy_time_frame_market!(
//         exchange, strategy, timeframe, market; E, S, T, M => {
//             let system = QuadSystem::<E, S, T, M>::new();
//             system.name()
//         }
//     );
//
//     assert_eq!(result, "binance_strategy_a_minute_spot");
// }
//
// #[test]
// fn test_five_enum_match() {
//     let exchange = Exchange::Binance;
//     let strategy = Strategy::StrategyA;
//     let timeframe = TimeFrame::Minute;
//     let market = Market::Spot;
//     let risk = RiskLevel::Low;
//
//     let result = match_exchange_strategy_time_frame_market_risk_level!(
//         exchange, strategy, timeframe, market, risk; E, S, T, M, R => {
//             let system = QuintSystem::<E, S, T, M, R>::new();
//             system.name()
//         }
//     );
//
//     assert_eq!(result, "binance_strategy_a_minute_spot_low_risk");
// }
