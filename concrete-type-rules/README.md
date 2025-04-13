# concrete-type-rules

[![Crates.io](https://img.shields.io/crates/v/concrete-type-rules.svg)](https://crates.io/crates/concrete-type-rules)
[![Documentation](https://docs.rs/concrete-type-rules/badge.svg)](https://docs.rs/concrete-type-rules)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/justastream/concrete-type/blob/main/LICENSE)

Utilities and extensions for working with the `concrete-type` crate, providing macros for composing multiple concrete enum types.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Features](#features)
  - [`gen_match_concretes_macro!`](#gen_match_concretes_macro)
- [Examples](#examples)
  - [Combined Matcher for Two Enum Types](#combined-matcher-for-two-enum-types)
  - [Using With More Enum Types](#using-with-more-enum-types)
- [Contributing](#contributing)
- [License](#license)

## Overview

`concrete-type-rules` extends the functionality of the `concrete-type` crate by providing utilities for working with multiple concrete enum types simultaneously. This enables:

- Composing multiple enum types together through generated macros
- Reducing nesting and improving code readability
- Creating type-safe interfaces for generic components
- Supporting up to 5 enum types in a single match expression

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
concrete-type = "0.2.0"
concrete-type-rules = "0.1.0"
```

## Features

### `gen_match_concretes_macro!`

The `gen_match_concretes_macro!` macro generates a new macro that allows you to match multiple enum instances simultaneously, providing type parameters for each concrete type associated with the enum variants.

Supports from 2 to 5 enum types.

## Examples

### Combined Matcher for Two Enum Types

```rust
use concrete_type::Concrete;
use concrete_type_rules::gen_match_concretes_macro;

#[derive(Concrete)]
enum Exchange {
    #[concrete = "exchanges::Binance"]
    Binance,
}

#[derive(Concrete)]
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

// Generate a combined matcher macro
gen_match_concretes_macro!(Exchange, Strategy);

// Now you can use the generated macro with both enum instances
let exchange = Exchange::Binance;
let strategy = Strategy::StrategyA;

// This uses a single match expression for both enums
let result = match_exchange_strategy!(exchange, strategy; E, S => {
    // E is exchanges::Binance, S is strategies::StrategyA
    format!("{} + {}", std::any::type_name::<E>(), std::any::type_name::<S>())
});
```

### Using With More Enum Types

The macro supports up to 5 enum types:

```rust
// For 3 enum types:
gen_match_concretes_macro!(Exchange, Strategy, Market);

// Generated macro name combines all enum names in snake_case
// E.g., match_exchange_strategy_market!

// For 4 or 5 enum types:
gen_match_concretes_macro!(Exchange, Strategy, Market, Asset, TimeFrame);
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT