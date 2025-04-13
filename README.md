# Concrete Type Workspace

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/justastream/concrete-type/blob/main/LICENSE)

A Rust workspace for type-level programming with enum variants, providing procedural macros and utilities for mapping enum variants to concrete types.

## Crates

This workspace contains the following crates:

- [**concrete-type**](./concrete-type/README.md) - Core procedural macros for mapping enum variants to concrete types
- [**concrete-type-rules**](./concrete-type-rules/README.md) - Utilities and extensions for working with multiple concrete enums

## Overview

The Concrete Type ecosystem provides tools for type-level programming based on runtime enum values. This enables:

- Mapping enum variants to specific concrete types
- Static dispatch based on runtime enum values
- Composition of multiple enum types
- Carrying configuration data with enum variants

## Installation

Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
# Core procedural macros
concrete-type = "0.2.0"

# Optional utilities for working with multiple concrete enums
concrete-type-rules = "0.1.0"
```

## Examples

### Basic Usage with `concrete-type`

```rust
use concrete_type::Concrete;

#[derive(Concrete, Clone, Copy)]
enum Exchange {
    #[concrete = "exchanges::Binance"]
    Binance,
    #[concrete = "exchanges::Coinbase"]
    Coinbase,
}

mod exchanges {
    pub struct Binance;
    pub struct Coinbase;
    
    impl Binance {
        pub fn new() -> Self { Binance }
        pub fn name(&self) -> &'static str { "binance" }
    }
    
    impl Coinbase {
        pub fn new() -> Self { Coinbase }
        pub fn name(&self) -> &'static str { "coinbase" }
    }
}

// Use the auto-generated exchange! macro for type-level dispatch
let exchange = Exchange::Binance;
let name = exchange!(exchange; ExchangeImpl => {
    // ExchangeImpl is aliased to the concrete type (exchanges::Binance)
    let instance = ExchangeImpl::new();
    instance.name()
});
assert_eq!(name, "binance");
```

### Enums with Configuration Data

```rust
use concrete_type::ConcreteConfig;

// Define concrete types and configuration types
mod exchanges {
    pub trait ExchangeApi {
        type Config;
        fn new(config: Self::Config) -> Self;
        fn name(&self) -> &'static str;
    }

    pub struct Binance;
    pub struct BinanceConfig {
        pub api_key: String,
    }

    impl ExchangeApi for Binance {
        type Config = BinanceConfig;
        fn new(_: Self::Config) -> Self { Self }
        fn name(&self) -> &'static str { "binance" }
    }
}

// Define the enum with concrete type mappings and config data
#[derive(ConcreteConfig)]
enum ExchangeConfig {
    #[concrete = "exchanges::Binance"]
    Binance(exchanges::BinanceConfig),
}

// Using the auto-generated macro with access to both type and config
let config = ExchangeConfig::Binance(
    exchanges::BinanceConfig { api_key: "secret".to_string() }
);

let name = exchange_config!(config; (Exchange, cfg) => {
    // Inside this block:
    // - Exchange is the concrete type (exchanges::Binance)
    // - cfg is the configuration instance (BinanceConfig)
    use exchanges::ExchangeApi;
    Exchange::new(cfg).name()
});
```

### Composing Multiple Enum Types with `concrete-type-rules`

```rust
use concrete_type::Concrete;
use concrete_type_rules::gen_match_concretes_macro;
use std::marker::PhantomData;

// Define multiple enum types with concrete type mappings
#[derive(Concrete, Clone, Copy)]
enum Exchange {
    #[concrete = "exchanges::Binance"]
    Binance,
}

#[derive(Concrete, Clone, Copy)]
enum Strategy {
    #[concrete = "strategies::StrategyA"]
    StrategyA,
}

mod exchanges {
    pub struct Binance;
}

mod strategies {
    pub struct StrategyA;
}

// A struct with type parameters
struct TradingSystem<E, S> {
    phantom: PhantomData<(E, S)>,
}

impl TradingSystem<exchanges::Binance, strategies::StrategyA> {
    fn new() -> Self {
        Self { phantom: PhantomData }
    }
    
    fn name(&self) -> &'static str {
        "binance_strategy_a"
    }
}

// Generate a combined matcher macro
gen_match_concretes_macro!(Exchange, Strategy);

// Now you can use the generated macro
let exchange = Exchange::Binance;
let strategy = Strategy::StrategyA;

// This handles both enums in a single match expression
let name = match_exchange_strategy!(exchange, strategy; E, S => {
    // E is exchanges::Binance, S is strategies::StrategyA
    let system = TradingSystem::<E, S>::new();
    system.name()
});
assert_eq!(name, "binance_strategy_a");
```

## License

MIT