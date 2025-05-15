use crate::exchanges::ExchangeApi;
use concrete_type::ConcreteConfig;

#[derive(ConcreteConfig)]
enum ExchangeConfig {
    #[concrete = "exchanges::Binance"]
    Binance(exchanges::BinanceConfig),
    #[concrete = "exchanges::Okx"]
    Okx,
}

fn main() {
    let config = ExchangeConfig::Binance(exchanges::BinanceConfig);

    let name = exchange_config!(config; (Exchange, config_param) => {
        Exchange::new(config_param).name()
    });

    assert_eq!(name, "binance");

    let config = ExchangeConfig::Okx;

    let name = exchange_config!(config; (Exchange, config_param) => {
        Exchange::new(config_param).name()
    });

    assert_eq!(name, "okx");
}

mod exchanges {
    pub trait ExchangeApi {
        type Config;

        fn new(config: Self::Config) -> Self;
        fn name(&self) -> &'static str;
    }

    pub struct Binance;

    impl ExchangeApi for Binance {
        type Config = BinanceConfig;

        fn new(_: Self::Config) -> Self {
            Self
        }

        fn name(&self) -> &'static str {
            "binance"
        }
    }

    pub struct BinanceConfig;

    pub struct Okx;

    impl ExchangeApi for Okx {
        type Config = ();

        fn new(_: Self::Config) -> Self {
            Self
        }

        fn name(&self) -> &'static str {
            "okx"
        }
    }
}
