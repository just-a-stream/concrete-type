# concrete-type

[![Crates.io](https://img.shields.io/crates/v/concrete-type.svg)](https://crates.io/crates/concrete-type)
[![Documentation](https://docs.rs/concrete-type/badge.svg)](https://docs.rs/concrete-type)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/justastream/concrete-type/blob/main/LICENSE)

A Rust procedural macro library for mapping enum variants to concrete types, enabling type-level programming based on runtime enum values.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Features](#features)
- [Examples](#examples)
  - [Basic Usage](#basic-usage)
  - [Enums with Config Data](#enums-with-config-data)
  - [Composing Multiple Enum Types](#composing-multiple-enum-types)
- [License](#license)

## Overview

`concrete-type` provides procedural macros that create a relationship between enum variants and specific concrete types. This enables:

- Type-level programming with enums
- Executing code with concrete type knowledge at compile time based on runtime enum values
- Generating helpful utility methods and macros for working with the concrete types
- Optionally carrying configuration data with enum variants
- Composing multiple enum types together through nested macro calls

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
concrete-type = "0.1.0"
```

## Features

### `#[derive(Concrete)]`

- Map enum variants to concrete types with `#[concrete = "path::to::Type"]` attribute
- Generated methods:
  - `concrete_type_id()`: Returns the `TypeId` of the concrete type for a variant
  - `concrete_type_name()`: Returns the name of the concrete type as a string
  - `with_concrete_type()`: Executes a function with knowledge of the concrete type
- Auto-generated macros for type-level dispatch using the snake_case name of the enum

### `#[derive(ConcreteConfig)]`

- Map enum variants with configuration data to concrete types
- Each variant must have a single tuple field containing the configuration
- Generated methods:
  - `concrete_type_id()`: Returns the `TypeId` of the concrete type for a variant
  - `concrete_type_name()`: Returns the name of the concrete type as a string
  - `config()`: Returns a reference to the configuration data
- Auto-generated macros for type-level dispatch with access to both the concrete type and config data

## Examples

### Basic Usage

```rust
use concrete_type::Concrete;

#[derive(Concrete, Clone, Copy)]
enum Exchange {
    #[concrete = "exchanges::Binance"]
    Binance,
}

mod exchanges {
    use crate::ExchangeApi;

    pub struct Binance;

    impl Binance {
        fn new() -> Self {
            Binance
        }

        fn name(&self) -> &'static str {
            "SomeBinanceName"
        }
    }
}

// Use the auto-generated 'exchange!' macro to work with concrete types
let exchange = Exchange::Binance;
let name = exchange!(exchange; ExchangeImpl => {
    // Here, ExchangeImpl is aliased to the concrete type (exchanges::Binance)
    let instance = ExchangeImpl::new();
    instance.name()
});
```

### Enums with Config Data

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
    pub struct BinanceConfig;

    impl ExchangeApi for Binance {
        type Config = BinanceConfig;
        fn new(_: Self::Config) -> Self { Self }
        fn name(&self) -> &'static str { "binance" }
    }

    pub struct Okx;
    pub struct OkxConfig;

    impl ExchangeApi for Okx {
        type Config = OkxConfig;
        fn new(_: Self::Config) -> Self { Self }
        fn name(&self) -> &'static str { "okx" }
    }
}

// Define the exchange config enum with concrete type mappings and config data
#[derive(ConcreteConfig)]
enum ExchangeConfig {
    #[concrete = "exchanges::Binance"]
    Binance(exchanges::BinanceConfig),
    #[concrete = "exchanges::Okx"]
    Okx(exchanges::OkxConfig),
}

// Import the trait for access to its methods
use exchanges::ExchangeApi;

// Using the auto-generated exchange_config! macro:
let config = ExchangeConfig::Binance(exchanges::BinanceConfig);
let name = exchange_config!(config; (Exchange, config_param) => {
    // Inside this block:
    // - Exchange is aliased to exchanges::Binance
    // - config_param is the BinanceConfig instance
    Exchange::new(config_param).name()
});
```

### Composing Multiple Enum Types

For more complex use cases, you can compose multiple enums together through nested macros:

```rust
use concrete_type::Concrete;
use std::marker::PhantomData;

// Define enums that map to concrete types
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

// A struct with type parameters
struct TradingSystem<Exchange, Strategy> {
    phantom: PhantomData<(Exchange, Strategy)>,
}

// Implement for concrete type combinations
impl TradingSystem<exchanges::Binance, strategies::StrategyA> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }

    pub fn name(&self) -> &'static str { 
        "binance_strategy_a" 
    }
}

// Using multiple enums together with nested macros
let exchange = Exchange::Binance;
let strategy = Strategy::StrategyA;

let name = exchange!(exchange; Exchange => {
    strategy!(strategy; Strategy => {
        TradingSystem::<Exchange, Strategy>::new().name()
    })
});
assert_eq!(name, "binance_strategy_a");
```

## License

MIT