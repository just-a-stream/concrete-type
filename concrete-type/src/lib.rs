#![doc(html_root_url = "https://docs.rs/concrete-type")]
#![warn(missing_docs)]

//! # Concrete Type
//!
//! A procedural macro library for mapping enum variants to concrete types.
//!
//! This crate provides two main derive macros:
//!
//! - [`Concrete`] - For enums where each variant maps to a specific concrete type
//! - [`ConcreteConfig`] - For enums where each variant has associated configuration data
//!   and maps to a specific concrete type
//!
//! These macros enable type-level programming based on runtime enum values by generating
//! helper methods and macros that provide access to the concrete types associated with
//! enum variants.
//!
//! ## Examples
//!
//! ### Basic Usage with `Concrete`
//!
//! ```rust,ignore
//! use concrete_type::Concrete;
//!
//! #[derive(Concrete, Clone, Copy)]
//! enum Exchange {
//!     #[concrete = "exchanges::Binance"]
//!     Binance,
//!     #[concrete = "exchanges::Coinbase"]
//!     Coinbase,
//! }
//!
//! mod exchanges {
//!     pub struct Binance;
//!     pub struct Coinbase;
//!     
//!     impl Binance {
//!         pub fn new() -> Self { Binance }
//!         pub fn name(&self) -> &'static str { "binance" }
//!     }
//!     
//!     impl Coinbase {
//!         pub fn new() -> Self { Coinbase }
//!         pub fn name(&self) -> &'static str { "coinbase" }
//!     }
//! }
//!
//! // Use the auto-generated exchange! macro for type-level dispatch
//! let exchange = Exchange::Binance;
//! let name = exchange!(exchange; ExchangeImpl => {
//!     // ExchangeImpl is aliased to the concrete type (exchanges::Binance)
//!     let instance = ExchangeImpl::new();
//!     instance.name()
//! });
//! assert_eq!(name, "binance");
//! ```
//!
//! ### Using `ConcreteConfig` with Configuration Data
//!
//! ```rust,ignore
//! use concrete_type::ConcreteConfig;
//!
//! // Define concrete types and configuration types
//! mod exchanges {
//!     pub trait ExchangeApi {
//!         type Config;
//!         fn new(config: Self::Config) -> Self;
//!         fn name(&self) -> &'static str;
//!     }
//!
//!     pub struct Binance;
//!     pub struct BinanceConfig {
//!         pub api_key: String,
//!     }
//!
//!     impl ExchangeApi for Binance {
//!         type Config = BinanceConfig;
//!         fn new(_: Self::Config) -> Self { Self }
//!         fn name(&self) -> &'static str { "binance" }
//!     }
//! }
//!
//! // Define the enum with concrete type mappings and config data
//! #[derive(ConcreteConfig)]
//! enum ExchangeConfig {
//!     #[concrete = "exchanges::Binance"]
//!     Binance(exchanges::BinanceConfig),
//! }
//!
//! // Using the auto-generated macro with access to both type and config
//! let config = ExchangeConfig::Binance(
//!     exchanges::BinanceConfig { api_key: "secret".to_string() }
//! );
//!
//! let name = exchange_config!(config; (Exchange, cfg) => {
//!     // Inside this block:
//!     // - Exchange is the concrete type (exchanges::Binance)
//!     // - cfg is the configuration instance (BinanceConfig)
//!     use exchanges::ExchangeApi;
//!     Exchange::new(cfg).name()
//! });
//! ```
//!
//! See the crate documentation and examples for more details.

extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Expr, Fields, Lit, Meta, parse_macro_input};

/// Helper function to extract concrete type path from an attribute
fn extract_concrete_type_path(attrs: &[Attribute]) -> Option<syn::Path> {
    for attr in attrs {
        if attr.path().is_ident("concrete") {
            if let Meta::NameValue(meta) = &attr.meta {
                if let Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        return syn::parse_str::<syn::Path>(&lit_str.value()).ok();
                    }
                }
            }
        }
    }
    None
}

/// A derive macro that implements the mapping between enum variants and concrete types.
///
/// This macro is designed for enums where each variant maps to a specific concrete type.
/// Each variant must be annotated with the `#[concrete = "path::to::Type"]` attribute that
/// specifies the concrete type that the variant represents.
///
/// # Generated Code
///
/// The macro generates a macro with the snake_case name of the enum
/// (e.g., `exchange!` for `Exchange`, `strategy_kind!` for `StrategyKind`) that can be used
/// to execute code with the concrete type.
///
/// # Example
///
/// ```rust,ignore
/// use concrete_type::Concrete;
///
/// #[derive(Concrete)]
/// enum StrategyKind {
///     #[concrete = "strategies::StrategyA"]
///     StrategyA,
///     #[concrete = "strategies::StrategyB"]
///     StrategyB,
/// }
///
/// // The generated macro is named after the enum in snake_case
/// let strategy = StrategyKind::StrategyA;
/// let result = strategy_kind!(strategy; T => {
///     // T is aliased to strategies::StrategyA here
///     std::any::type_name::<T>()
/// });
/// ```
///
/// This enables type-level programming with enums, where you can define enum variants and
/// map them to concrete type implementations.
#[proc_macro_derive(Concrete, attributes(concrete))]
pub fn derive_concrete(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the type
    let type_name = &input.ident;

    // Create a snake_case version of the type name for the macro_rules! name
    let type_name_str = type_name.to_string();
    let macro_name_str = type_name_str.to_case(Case::Snake);
    let macro_name = syn::Ident::new(&macro_name_str, type_name.span());

    // Handle enum case
    let data_enum = match &input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => {
            return syn::Error::new_spanned(
                type_name,
                "Concrete can only be derived for enums or structs with type parameters",
            )
            .to_compile_error()
            .into();
        }
    };

    // Extract variant names and their concrete types
    let mut variant_mappings = Vec::new();

    for variant in &data_enum.variants {
        let variant_name = &variant.ident;

        // Extract the concrete type path from the variant's attributes
        if let Some(concrete_type) = extract_concrete_type_path(&variant.attrs) {
            variant_mappings.push((variant_name, concrete_type));
        } else {
            // Variant is missing the #[concrete = "..."] attribute
            return syn::Error::new_spanned(
                variant_name,
                format!(
                    "Enum variant `{}` is missing the #[concrete = \"...\"] attribute",
                    variant_name
                ),
            )
            .to_compile_error()
            .into();
        }
    }

    // Generate match arms for the macro_rules! version
    let macro_match_arms = variant_mappings
        .iter()
        .map(|(variant_name, concrete_type)| {
            quote! {
                #type_name::#variant_name => {
                    type $type_param = #concrete_type;
                    $code_block
                }
            }
        });

    // Generate a top-level macro with the snake_case name of the enum
    let macro_def = quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($enum_instance:expr; $type_param:ident => $code_block:block) => {
                match $enum_instance {
                    #(#macro_match_arms),*
                }
            };
        }
    };

    // Combine the macro definition and methods implementation
    let expanded = quote! {
        // Define the macro outside any module to make it directly accessible
        #macro_def
    };

    // Return the generated implementation
    TokenStream::from(expanded)
}

/// A derive macro that implements the mapping between enum variants with associated data and
/// concrete types.
///
/// This macro is designed for enums where each variant has associated configuration data and maps
/// to a specific concrete type. Each variant must be annotated with the
/// `#[concrete = "path::to::Type"]` attribute and contain a single tuple field
/// that holds the configuration data for that concrete type.
///
/// # Generated Code
///
/// The macro generates:
/// 1. A `config` method that returns a reference to the configuration data
/// 2. A macro with the snake_case name of the enum + "_config" (with "Config" suffix removed if present)
///    that allows access to both the concrete type and configuration data
///
/// # Example
///
/// ```rust,ignore
/// use concrete_type::ConcreteConfig;
///
/// // Define concrete types and configuration types
/// struct BinanceConfig {
///     api_key: String,
/// }
///
/// struct Binance;
///
/// #[derive(ConcreteConfig)]
/// enum ExchangeConfig {
///     #[concrete = "Binance"]
///     Binance(BinanceConfig),
/// }
///
/// // Using the generated macro
/// let config = ExchangeConfig::Binance(BinanceConfig { api_key: "key".to_string() });
/// let result = exchange_config!(config; (Exchange, cfg) => {
///     // Exchange is aliased to Binance
///     // cfg is a reference to BinanceConfig
///     format!("{} with key: {}", std::any::type_name::<Exchange>(), cfg.api_key)
/// });
/// ```
#[proc_macro_derive(ConcreteConfig, attributes(concrete))]
pub fn derive_concrete_config(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the type
    let type_name = &input.ident;

    // Create a snake_case version of the type name for the macro_rules! name
    let type_name_str = type_name.to_string();
    // Strip "Config" suffix if present for cleaner macro names
    let base_name = if type_name_str.ends_with("Config") {
        &type_name_str[0..type_name_str.len() - 6]
    } else {
        &type_name_str
    };
    let macro_name_str = format!("{}_config", base_name.to_case(Case::Snake));
    let macro_name = syn::Ident::new(&macro_name_str, type_name.span());

    // Ensure we're dealing with an enum
    let data_enum = match &input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => {
            return syn::Error::new_spanned(
                type_name,
                "ConcreteConfig can only be derived for enums with data",
            )
            .to_compile_error()
            .into();
        }
    };

    // Extract variant names, their concrete types, and field types
    let mut variant_mappings = Vec::new();

    for variant in &data_enum.variants {
        let variant_name = &variant.ident;

        // Extract the concrete type path from the variant's attributes
        if let Some(concrete_type) = extract_concrete_type_path(&variant.attrs) {
            // Verify the variant has a tuple field
            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    variant_mappings.push((variant_name, concrete_type));
                }
                _ => {
                    return syn::Error::new_spanned(
                        variant_name,
                        format!(
                            "Enum variant `{}` must have exactly one unnamed field for the config",
                            variant_name
                        ),
                    )
                    .to_compile_error()
                    .into();
                }
            }
        } else {
            // Variant is missing the #[concrete = "..."] attribute
            return syn::Error::new_spanned(
                variant_name,
                format!(
                    "Enum variant `{}` is missing the #[concrete = \"...\"] attribute",
                    variant_name
                ),
            )
            .to_compile_error()
            .into();
        }
    }

    // Generate match arms for the config method
    let config_arms = variant_mappings
        .iter()
        .map(|(variant_name, _concrete_type)| {
            quote! {
                #type_name::#variant_name(config) => config
            }
        });

    // Generate match arms for the macro_rules! version
    let macro_match_arms = variant_mappings
        .iter()
        .map(|(variant_name, concrete_type)| {
            quote! {
                #type_name::#variant_name(config) => {
                    type $type_param = #concrete_type;
                    let $config_param = config;
                    $code_block
                }
            }
        });

    // Generate a top-level macro with the snake_case name of the enum + "_config"
    let macro_def = quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($enum_instance:expr; ($type_param:ident, $config_param:ident) => $code_block:block) => {
                match $enum_instance {
                    #(#macro_match_arms),*
                }
            };
        }
    };

    // Generate the methods implementation
    let methods_impl = quote! {
        impl #type_name {
            /// Returns a reference to the configuration data associated with this enum variant
            pub fn config(&self) -> &dyn std::any::Any {
                match self {
                    #(#config_arms),*
                }
            }
        }
    };

    // Combine the macro definition and methods implementation
    let expanded = quote! {
        // Define the macro
        #macro_def

        // Implement methods on the enum
        #methods_impl
    };

    TokenStream::from(expanded)
}
