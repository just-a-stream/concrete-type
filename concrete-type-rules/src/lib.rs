#![doc(html_root_url = "https://docs.rs/concrete-type-rules")]
#![warn(missing_docs)]

//! # Concrete Type Rules
//!
//! Utilities and extensions for working with the `concrete-type` crate.
//!
//! This crate provides additional tools for composing multiple concrete types together,
//! including macros for generating combined matchers that can handle multiple enum types
//! at once.
//!
//! ## Features
//!
//! - `gen_match_concretes_macro!` - Generates macros for matching multiple enum instances
//!   simultaneously, with support for 2-5 enum types.
//!
//! ## Examples
//!
//! ### Combined Matcher for Two Enum Types
//!
//! ```rust,ignore
//! use concrete_type::Concrete;
//! use concrete_type_rules::gen_match_concretes_macro;
//!
//! #[derive(Concrete)]
//! enum Exchange {
//!     #[concrete = "crate::exchanges::Binance"]
//!     Binance,
//! }
//!
//! #[derive(Concrete)]
//! enum Strategy {
//!     #[concrete = "crate::strategies::StrategyA"]
//!     StrategyA,
//! }
//!
//! mod exchanges {
//!     pub struct Binance;
//! }
//!
//! mod strategies {
//!     pub struct StrategyA;
//! }
//!
//! // Generate a combined matcher macro
//! gen_match_concretes_macro!(Exchange, Strategy);
//!
//! // Now you can use the generated macro with both enum instances
//! let exchange = Exchange::Binance;
//! let strategy = Strategy::StrategyA;
//!
//! // This uses a single match expression for both enums
//! let result = match_exchange_strategy!(exchange, strategy; E, S => {
//!     // E is exchanges::Binance, S is strategies::StrategyA
//!     format!("{} + {}", std::any::type_name::<E>(), std::any::type_name::<S>())
//! });
//! ```
//!
//! ### Using With More Enum Types
//!
//! ```rust,ignore
//! // For 3 enum types:
//! gen_match_concretes_macro!(Exchange, Strategy, Market);
//!
//! // Generated macro name combines all enum names in snake_case
//! // E.g., match_exchange_strategy_market!
//!
//! // For 4 or 5 enum types:
//! gen_match_concretes_macro!(Exchange, Strategy, Market, Asset, TimeFrame);
//! ```

/// A macro that generates a combined matcher macro for multiple concrete enums.
///
/// This macro creates a new macro that allows you to match multiple enum instances
/// simultaneously, providing type parameters for each concrete type associated with
/// the enum variants.
///
/// # Arguments
///
/// * First argument: First enum type name
/// * Second argument: Second enum type name
/// * Optionally: Third, fourth, and fifth enum type names
///
/// The generated macro will be named using the snake_case of all provided enum names,
/// joined with underscores and prefixed with "match_".
///
/// # Generated Macro Usage
///
/// The generated macro accepts:
///
/// * Enum instances as positional parameters (one for each enum type)
/// * Type parameters and a code block after a semicolon
///
/// Inside the code block, each type parameter is aliased to the concrete type
/// associated with the corresponding enum variant.
///
/// # Examples
///
/// ```rust,ignore
/// use concrete_type::Concrete;
/// use concrete_type_rules::gen_match_concretes_macro;
///
/// #[derive(Concrete, Clone, Copy)]
/// enum Exchange {
///     #[concrete = "crate::BinanceType"]
///     Binance,
/// }
///
/// #[derive(Concrete)]
/// enum Strategy {
///     #[concrete = "crate::StrategyAType"]
///     StrategyA,
/// }
///
/// struct BinanceType;
/// struct StrategyAType;
///
/// // Generate a combined matcher macro
/// gen_match_concretes_macro!(Exchange, Strategy);
///
/// // Now you can use the generated macro
/// let exchange = Exchange::Binance;
/// let strategy = Strategy::StrategyA;
///
/// let result = match_exchange_strategy!(exchange, strategy; E, S => {
///     // Here E is BinanceType and S is StrategyAType
///     format!("{}", std::any::type_name::<(E, S)>())
/// });
/// ```
#[macro_export]
macro_rules! gen_match_concretes_macro {
    // For 2 enum types
    ($first_enum:ident, $second_enum:ident) => {
        paste::paste! {
            #[macro_export]
            macro_rules! [<match_ $first_enum:snake _ $second_enum:snake>] {
                ($first_var:expr, $second_var:expr; $first_type:ident, $second_type:ident => $code_block:block) => {
                    [<$first_enum:snake>]!($first_var; $first_type => {
                        [<$second_enum:snake>]!($second_var; $second_type => {
                            $code_block
                        })
                    })
                };
            }
        }
    };

    // For 3 enum types
    ($first_enum:ident, $second_enum:ident, $third_enum:ident) => {
        paste::paste! {
            #[macro_export]
            macro_rules! [<match_ $first_enum:snake _ $second_enum:snake _ $third_enum:snake>] {
                ($first_var:expr, $second_var:expr, $third_var:expr; $first_type:ident, $second_type:ident, $third_type:ident => $code_block:block) => {
                    [<$first_enum:snake>]!($first_var; $first_type => {
                        [<$second_enum:snake>]!($second_var; $second_type => {
                            [<$third_enum:snake>]!($third_var; $third_type => {
                                $code_block
                            })
                        })
                    })
                };
            }
        }
    };

    // For 4 enum types
    ($first_enum:ident, $second_enum:ident, $third_enum:ident, $fourth_enum:ident) => {
        paste::paste! {
            #[macro_export]
            macro_rules! [<match_ $first_enum:snake _ $second_enum:snake _ $third_enum:snake _ $fourth_enum:snake>] {
                ($first_var:expr, $second_var:expr, $third_var:expr, $fourth_var:expr;
                 $first_type:ident, $second_type:ident, $third_type:ident, $fourth_type:ident => $code_block:block) => {
                    [<$first_enum:snake>]!($first_var; $first_type => {
                        [<$second_enum:snake>]!($second_var; $second_type => {
                            [<$third_enum:snake>]!($third_var; $third_type => {
                                [<$fourth_enum:snake>]!($fourth_var; $fourth_type => {
                                    $code_block
                                })
                            })
                        })
                    })
                };
            }
        }
    };

    // For 5 enum types
    ($first_enum:ident, $second_enum:ident, $third_enum:ident, $fourth_enum:ident, $fifth_enum:ident) => {
        paste::paste! {
            #[macro_export]
            macro_rules! [<match_ $first_enum:snake _ $second_enum:snake _ $third_enum:snake _ $fourth_enum:snake _ $fifth_enum:snake>] {
                ($first_var:expr, $second_var:expr, $third_var:expr, $fourth_var:expr, $fifth_var:expr;
                 $first_type:ident, $second_type:ident, $third_type:ident, $fourth_type:ident, $fifth_type:ident => $code_block:block) => {
                    [<$first_enum:snake>]!($first_var; $first_type => {
                        [<$second_enum:snake>]!($second_var; $second_type => {
                            [<$third_enum:snake>]!($third_var; $third_type => {
                                [<$fourth_enum:snake>]!($fourth_var; $fourth_type => {
                                    [<$fifth_enum:snake>]!($fifth_var; $fifth_type => {
                                        $code_block
                                    })
                                })
                            })
                        })
                    })
                };
            }
        }
    };
}
