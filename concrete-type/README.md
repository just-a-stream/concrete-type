# concrete-type

[![Crates.io](https://img.shields.io/crates/v/concrete-type.svg)](https://crates.io/crates/concrete-type)
[![Documentation](https://docs.rs/concrete-type/badge.svg)](https://docs.rs/concrete-type)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/justastream/concrete-type/blob/main/LICENSE)

A procedural macro library for mapping enum variants to concrete types, enabling type-level programming based on runtime enum values.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Features](#features)
  - [`#[derive(Concrete)]`](#deriveconcrete)
  - [`#[derive(ConcreteConfig)]`](#deriveconcreteconfig)
- [Examples](#examples)
  - [Basic Usage](#basic-usage)
  - [Enums with Config Data](#enums-with-config-data)
- [Contributing](#contributing)
- [License](#license)

## Overview

`concrete-type` provides procedural macros that create a relationship between enum variants and specific concrete types. 
This enables:
- Type-level programming with enums
- Executing code with concrete type knowledge at compile time based on runtime enum values
- Optionally carrying configuration data with enum variants

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
concrete-type = "0.2.0"
```

## Features

### `#[derive(Concrete)]`

The `Concrete` derive macro is designed for enums where each variant maps to a specific concrete type.

- Map enum variants to concrete types with `#[concrete = "path::to::Type"]` attribute
- Auto-generated macros for type-level dispatch using the snake_case name of the enum

Example:

```rust
#[derive(Concrete)]
enum StrategyKind {
    #[concrete = "strategies::StrategyA"]
    StrategyA,
    #[concrete = "strategies::StrategyB"]
    StrategyB,
}

// Generated macro is named 'strategy_kind!'
```

### `#[derive(ConcreteConfig)]`

The `ConcreteConfig` derive macro is designed for enums where each variant has associated configuration data and maps to a specific concrete type.

- Map enum variants with configuration data to concrete types
- Variants without configuration provided default to using the unit type `()`.
- Variants with configuration must have a single field (not a tuple).
- Generated methods:
  - `config()`: Returns a reference to the configuration data
- Auto-generated macros for type-level dispatch with access to both the concrete type and config data

Example:

```rust
#[derive(ConcreteConfig)]
enum ExchangeConfig {
    #[concrete = "exchanges::Binance"]
    Binance(exchanges::BinanceConfig),  // With config
    #[concrete = "exchanges::Okx"]
    Okx,                                // Without config (defaults to unit type)
}

// Generated macro is named 'exchange_config!'
```

## Examples

### Basic Usage

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

// Use the auto-generated 'exchange!' macro to work with concrete types
let exchange = Exchange::Binance;
let name = exchange!(exchange; ExchangeImpl => {
    // Here, ExchangeImpl is aliased to the concrete type (exchanges::Binance)
    let instance = ExchangeImpl::new();
    instance.name()
});
assert_eq!(name, "binance");
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

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT