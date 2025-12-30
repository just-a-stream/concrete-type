use crate::exchanges::ExchangeApi;
use concrete_type::ConcreteConfig;

#[derive(ConcreteConfig)]
enum ExchangeConfig {
    #[concrete = "crate::exchanges::Binance"]
    Binance(exchanges::BinanceConfig),
    #[concrete = "crate::exchanges::Okx"]
    Okx,
    #[concrete = "crate::exchanges::Bitmart<crate::exchanges::BitmartSpotServer>"]
    Bitmart(exchanges::BitmartConfig),
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

    let config = ExchangeConfig::Bitmart(exchanges::BitmartConfig);

    let name = exchange_config!(config; (Exchange, config_param) => {
        Exchange::new(config_param).name()
    });

    assert_eq!(name, "bitmart");
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

    pub struct BitmartSpotServer;
    pub struct Bitmart<Server> {
        _phantom: std::marker::PhantomData<Server>,
    }

    impl ExchangeApi for Bitmart<BitmartSpotServer> {
        type Config = BitmartConfig;

        fn new(_: Self::Config) -> Self {
            Self {
                _phantom: std::marker::PhantomData,
            }
        }

        fn name(&self) -> &'static str {
            "bitmart"
        }
    }

    pub struct BitmartConfig;
}
