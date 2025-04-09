extern crate proc_macro;

use convert_case::{Boundary, Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, DeriveInput, Expr, Fields, Ident, Lit, Meta, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro_derive(DeExchange)]
pub fn de_exchange_derive(input: TokenStream) -> TokenStream {
    // Parse Rust code abstract syntax tree with Syn from TokenStream -> DeriveInput
    let ast: DeriveInput =
        syn::parse(input).expect("de_exchange_derive() failed to parse input TokenStream");

    // Determine execution name
    let exchange = &ast.ident;

    let generated = quote! {
        impl<'de> serde::Deserialize<'de> for #exchange {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>
            {
                let input = <String as serde::Deserialize>::deserialize(deserializer)?;
                let exchange = #exchange::ID;
                let expected = exchange.as_str();

                if input.as_str() == expected {
                    Ok(Self::default())
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(input.as_str()),
                        &expected
                    ))
                }
            }
        }
    };

    TokenStream::from(generated)
}

#[proc_macro_derive(SerExchange)]
pub fn ser_exchange_derive(input: TokenStream) -> TokenStream {
    // Parse Rust code abstract syntax tree with Syn from TokenStream -> DeriveInput
    let ast: DeriveInput =
        syn::parse(input).expect("ser_exchange_derive() failed to parse input TokenStream");

    // Determine Exchange
    let exchange = &ast.ident;

    let generated = quote! {
        impl serde::Serialize for #exchange {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                serializer.serialize_str(#exchange::ID.as_str())
            }
        }
    };

    TokenStream::from(generated)
}

#[proc_macro_derive(DeSubKind)]
pub fn de_sub_kind_derive(input: TokenStream) -> TokenStream {
    // Parse Rust code abstract syntax tree with Syn from TokenStream -> DeriveInput
    let ast: DeriveInput =
        syn::parse(input).expect("de_sub_kind_derive() failed to parse input TokenStream");

    // Determine SubKind name
    let sub_kind = &ast.ident;

    let expected_sub_kind = sub_kind
        .to_string()
        .from_case(Case::Pascal)
        .without_boundaries(&Boundary::letter_digit())
        .to_case(Case::Snake);

    let generated = quote! {
        impl<'de> serde::Deserialize<'de> for #sub_kind {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>
            {
                let input = <String as serde::Deserialize>::deserialize(deserializer)?;

                if input == #expected_sub_kind {
                    Ok(Self)
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(input.as_str()),
                        &#expected_sub_kind
                    ))
                }
            }
        }
    };

    TokenStream::from(generated)
}

#[proc_macro_derive(SerSubKind)]
pub fn ser_sub_kind_derive(input: TokenStream) -> TokenStream {
    // Parse Rust code abstract syntax tree with Syn from TokenStream -> DeriveInput
    let ast: DeriveInput =
        syn::parse(input).expect("ser_sub_kind_derive() failed to parse input TokenStream");

    // Determine SubKind name
    let sub_kind = &ast.ident;
    let sub_kind_string = sub_kind.to_string().to_case(Case::Snake);
    let sub_kind_str = sub_kind_string.as_str();

    let generated = quote! {
        impl serde::Serialize for #sub_kind {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                serializer.serialize_str(#sub_kind_str)
            }
        }
    };

    TokenStream::from(generated)
}

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
/// This derive macro is designed for enums where each variant maps to a specific concrete type.
/// Each variant must be annotated with the `#[concrete = "path::to::Type"]` attribute that
/// specifies the concrete type that the variant represents.
///
/// The macro generates:
/// 1. A `concrete_type_id` method that returns the `TypeId` of the concrete type for a variant
/// 2. A `concrete_type_name` method that returns the name of the concrete type as a string
/// 3. A `with_concrete_type` method that executes a function with knowledge of the concrete type
/// 4. A macro with the snake_case name of the enum (e.g., `exchange!` for `Exchange`,
///    `strategy!` for `Strategy`) that can be used to execute code with the concrete type
///
/// This enables type-level programming with enums, where you can define enum variants and
/// map them to concrete type implementations.
///
/// # Example
/// ```
/// use concrete_type::Concrete;
/// use std::marker::PhantomData;
///
/// // Define concrete types
/// mod exchanges {
///     pub struct Binance;
///     pub struct Okx;
///
///     impl Binance {
///         pub fn new() -> Self { Self }
///         pub fn name(&self) -> &'static str { "binance" }
///     }
///
///     impl Okx {
///         pub fn new() -> Self { Self }
///         pub fn name(&self) -> &'static str { "okx" }
///     }
/// }
///
/// // Define the exchange enum with concrete type mappings
/// #[derive(Concrete, Clone, Copy)]
/// enum Exchange {
///     #[concrete = "exchanges::Binance"]
///     Binance,
///     #[concrete = "exchanges::Okx"]
///     Okx,
/// }
///
/// // Using the auto-generated exchange! macro:
/// let exchange = Exchange::Binance;
/// let name = exchange!(exchange; ExchangeImpl => {
///     // Inside this block, ExchangeImpl is aliased to exchanges::Binance
///     let instance = ExchangeImpl::new();
///     instance.name()
/// });
/// assert_eq!(name, "binance");
///
/// // Using the Exchange enum's methods directly
/// let exchange = Exchange::Okx;
/// // Get the TypeId of the concrete type
/// let type_id = exchange.concrete_type_id();
/// // Get the name of the concrete type (as a string)
/// let type_name = exchange.concrete_type_name();
/// assert!(type_name.ends_with("exchanges::Okx"));
/// ```
#[proc_macro_derive(Concrete, attributes(concrete))]
pub fn derive_concrete(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the type
    let type_name = &input.ident;

    // Create a snake_case version of the type name for the macro_rules! name
    let type_name_str = type_name.to_string();

    // Check if we're dealing with a struct that has type parameters
    let is_struct_with_type_params = match &input.data {
        syn::Data::Struct(_) => !input.generics.params.is_empty(),
        _ => false,
    };

    // Handle differently based on whether we're dealing with an enum or a struct with type parameters
    if is_struct_with_type_params {
        // The macro name should be 'trading_system' based on the file content, not derived from the struct name
        let macro_name = syn::Ident::new("trading_system", type_name.span());

        // Generate a macro that accepts multiple arguments and type parameters
        // This handles syntax like: trading_system!(exchange, strategy; (Exchange, Strategy) => { ... })
        let trading_system_macro = quote! {
            /// Generates a macro for working with a struct that has type parameters.
            /// This macro composes multiple enum-based macros to provide concrete types for generic parameters.
            ///
            /// # Example
            ///
            /// ```
            /// use concrete_type::Concrete;
            /// use std::marker::PhantomData;
            ///
            /// // Define the concrete implementation types
            /// mod exchanges {
            ///     pub struct Binance;
            ///     pub struct Okx;
            /// }
            ///
            /// mod strategies {
            ///     pub struct StrategyA;
            ///     pub struct StrategyB;
            ///
            ///     impl StrategyA {
            ///         pub fn name() -> &'static str { "strategy_a" }
            ///     }
            ///
            ///     impl StrategyB {
            ///         pub fn name() -> &'static str { "strategy_b" }
            ///     }
            /// }
            ///
            /// // Define enums that map to concrete types
            /// #[derive(Concrete, Clone, Copy)]
            /// enum Exchange {
            ///     #[concrete = "exchanges::Binance"]
            ///     Binance,
            ///     #[concrete = "exchanges::Okx"]
            ///     Okx,
            /// }
            ///
            /// #[derive(Concrete)]
            /// enum Strategy {
            ///     #[concrete = "strategies::StrategyA"]
            ///     StrategyA,
            ///     #[concrete = "strategies::StrategyB"]
            ///     StrategyB,
            /// }
            ///
            /// // A struct with type parameters that will be resolved at runtime
            /// #[derive(Concrete)]
            /// struct TradingSystem<E, S> {
            ///     phantom: PhantomData<(E, S)>,
            /// }
            ///
            /// // Implement for concrete type combinations
            /// impl TradingSystem<exchanges::Binance, strategies::StrategyA> {
            ///     pub fn new() -> Self {
            ///         Self { phantom: PhantomData }
            ///     }
            ///
            ///     pub fn name(&self) -> &'static str { "binance_strategy_a" }
            /// }
            ///
            /// impl TradingSystem<exchanges::Okx, strategies::StrategyB> {
            ///     pub fn new() -> Self {
            ///         Self { phantom: PhantomData }
            ///     }
            ///
            ///     pub fn name(&self) -> &'static str { "okx_strategy_b" }
            /// }
            ///
            /// // Use the trading_system macro
            /// let exchange = Exchange::Okx;
            /// let strategy = Strategy::StrategyB;
            ///
            /// let name = trading_system!(exchange, strategy; (E, S) => {
            ///     // Here E is exchanges::Okx and S is strategies::StrategyB
            ///     TradingSystem::<E, S>::new().name()
            /// });
            /// assert_eq!(name, "okx_strategy_b");
            /// ```
            #[macro_export]
            macro_rules! #macro_name {
                // Match the pattern with two enum instances and two type parameters
                ($exchange_enum:expr, $strategy_enum:expr; ($exchange_type:ident, $strategy_type:ident) => $code_block:block) => {
                    exchange!($exchange_enum; $exchange_type => {
                        strategy!($strategy_enum; $strategy_type => {
                            $code_block
                        })
                    })
                };
            }
        };

        TokenStream::from(trading_system_macro)
    } else {
        // Handle enum case
        // Ensure we're dealing with an enum
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
        let mut has_data = false;

        for variant in &data_enum.variants {
            let variant_name = &variant.ident;

            // Check if any variant has data
            if !variant.fields.is_empty() {
                match &variant.fields {
                    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        has_data = true;
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
            }

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

        // Decide which macro name to use based on whether variants have data
        let macro_name_str = if has_data {
            // Strip "Config" suffix if present for cleaner macro names
            let base_name = if type_name_str.ends_with("Config") {
                &type_name_str[0..type_name_str.len() - 6]
            } else {
                &type_name_str
            };
            format!("{}_config", base_name.to_case(Case::Snake))
        } else {
            type_name_str.to_case(Case::Snake)
        };
        let macro_name = syn::Ident::new(&macro_name_str, type_name.span());

        // Generate match arms for the concrete type mapping
        let match_arms = variant_mappings
            .iter()
            .map(|(variant_name, concrete_type)| {
                if has_data {
                    quote! {
                        #type_name::#variant_name(_) => {
                            type_id::<#concrete_type>()
                        }
                    }
                } else {
                    quote! {
                        #type_name::#variant_name => {
                            type_id::<#concrete_type>()
                        }
                    }
                }
            });

        // Generate match arms for the concrete type name
        let type_name_arms = variant_mappings
            .iter()
            .map(|(variant_name, concrete_type)| {
                if has_data {
                    quote! {
                        #type_name::#variant_name(_) => type_name_of::<#concrete_type>()
                    }
                } else {
                    quote! {
                        #type_name::#variant_name => type_name_of::<#concrete_type>()
                    }
                }
            });

        // Generate match arms for the config method if enum has data
        let config_arms = variant_mappings
            .iter()
            .map(|(variant_name, _concrete_type)| {
                quote! {
                    #type_name::#variant_name(config) => config
                }
            });

        // Generate match arms for the concrete type aliases
        let type_alias_arms = variant_mappings
            .iter()
            .map(|(variant_name, concrete_type)| {
                if has_data {
                    quote! {
                        #type_name::#variant_name(config) => {
                            type ConcreteType = #concrete_type;
                            f(config)
                        }
                    }
                } else {
                    quote! {
                        #type_name::#variant_name => {
                            type ConcreteType = #concrete_type;
                            f()
                        }
                    }
                }
            });

        // Generate match arms for the macro_rules! version
        let macro_match_arms = variant_mappings
            .iter()
            .map(|(variant_name, concrete_type)| {
                if has_data {
                    quote! {
                        #type_name::#variant_name(config) => {
                            type $type_param = #concrete_type;
                            let $config_param = config;
                            $code_block
                        }
                    }
                } else {
                    quote! {
                        #type_name::#variant_name => {
                            type $type_param = #concrete_type;
                            $code_block
                        }
                    }
                }
            });

        // Generate a top-level macro with the appropriate name and pattern
        let macro_def = if has_data {
            quote! {
                #[macro_export]
                macro_rules! #macro_name {
                    ($enum_instance:expr; ($type_param:ident, $config_param:ident) => $code_block:block) => {
                        match $enum_instance {
                            #(#macro_match_arms),*
                        }
                    };
                }
            }
        } else {
            quote! {
                #[macro_export]
                macro_rules! #macro_name {
                    ($enum_instance:expr; $type_param:ident => $code_block:block) => {
                        match $enum_instance {
                            #(#macro_match_arms),*
                        }
                    };
                }
            }
        };

        // Generate the with_concrete_type method with appropriate signature
        let with_concrete_type_method = if has_data {
            quote! {
                /// Executes a function with the concrete type associated with this enum variant
                /// and provides access to the config data
                pub fn with_concrete_type<F, R>(&self, f: F) -> R
                where
                    F: FnOnce(&dyn std::any::Any) -> R,
                {
                    match self {
                        #(#type_alias_arms),*
                    }
                }
            }
        } else {
            quote! {
                /// Executes a function with the concrete type associated with this enum variant
                pub fn with_concrete_type<F, R>(&self, f: F) -> R
                where
                    F: FnOnce() -> R,
                {
                    match self {
                        #(#type_alias_arms),*
                    }
                }
            }
        };

        // Generate the config method if enum has data
        let config_method = if has_data {
            quote! {
                /// Get config data from the enum variant
                pub fn config(&self) -> &dyn std::any::Any {
                    match self {
                        #(#config_arms),*
                    }
                }
            }
        } else {
            quote! {}
        };

        // Generate the methods implementation
        let methods_impl = quote! {
            impl #type_name {
                /// Returns the TypeId of the concrete type associated with this enum variant
                pub fn concrete_type_id(&self) -> std::any::TypeId {
                    use std::any::TypeId;

                    fn type_id<T: 'static>() -> TypeId {
                        TypeId::of::<T>()
                    }

                    match self {
                        #(#match_arms),*
                    }
                }

                /// Returns the name of the concrete type associated with this enum variant
                pub fn concrete_type_name(&self) -> &'static str {
                    use std::any::type_name;

                    fn type_name_of<T: 'static>() -> &'static str {
                        type_name::<T>()
                    }

                    match self {
                        #(#type_name_arms),*
                    }
                }

                #with_concrete_type_method

                #config_method
            }
        };

        // Combine the macro definition and methods implementation
        let expanded = quote! {
            // Define the macro outside any module to make it directly accessible
            #macro_def

            // Implement methods on the enum
            #methods_impl
        };

        // Return the generated implementation
        TokenStream::from(expanded)
    }
}

/// Parser for the concrete macro
/// Format: concrete!(enum_instance; TypeParam => { code_block })
struct ConcreteMacroInput {
    enum_instance: Expr,
    type_param: Ident,
    code_block: syn::Block,
}

impl Parse for ConcreteMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let enum_instance = input.parse()?;
        input.parse::<Token![;]>()?;
        let type_param = input.parse()?;
        input.parse::<Token![=>]>()?;
        let code_block = input.parse()?;

        Ok(ConcreteMacroInput {
            enum_instance,
            type_param,
            code_block,
        })
    }
}

/// A procedural macro for executing code with knowledge of the concrete type mapped to an enum variant.
///
/// This macro takes an enum instance that has been derived with `#[derive(Concrete)]` and uses
/// the concrete type information to execute a block of code with a type alias to the concrete type.
/// This enables type-level programming based on runtime enum values.
///
/// # Usage
/// ```ignore
/// concrete!(enum_instance; TypeParam => { code_block })
/// ```
///
/// # Parameters
/// - `enum_instance`: An instance of an enum with `#[derive(Concrete)]`
/// - `TypeParam`: The name you want to use as the type alias within the code block
/// - `code_block`: The code to execute with knowledge of the concrete type
///
/// # Example
/// ```
/// use concrete_type::Concrete;
/// use std::marker::PhantomData;
///
/// // Define concrete types and trait
/// trait ExchangeApi {
///     fn new() -> Self;
///     fn name(&self) -> &'static str;
/// }
///
/// mod exchanges {
///     pub struct Binance;
///     impl Binance {
///         pub fn new() -> Self { Self }
///         pub fn name(&self) -> &'static str { "binance" }
///     }
///
///     pub struct Okx;
///     impl Okx {
///         pub fn new() -> Self { Self }
///         pub fn name(&self) -> &'static str { "okx" }
///     }
/// }
///
/// // Define the exchange enum with concrete type mappings
/// #[derive(Concrete, Clone, Copy)]
/// enum Exchange {
///     #[concrete = "exchanges::Binance"]
///     Binance,
///     #[concrete = "exchanges::Okx"]
///     Okx,
/// }
///
/// // This example uses the auto-generated `exchange!` macro instead of the `concrete!` macro
/// let exchange = Exchange::Binance;
/// let name = exchange!(exchange; ExchangeImpl => {
///     // Here, ExchangeImpl is aliased to the concrete type (exchanges::Binance)
///     let instance = ExchangeImpl::new();
///     instance.name()
/// });
/// assert_eq!(name, "binance");
///
/// // You can also use with_concrete_type for a similar effect:
/// let exchange = Exchange::Okx;
/// let name = exchange.with_concrete_type(|| {
///     // Inside this block, ConcreteType is aliased to exchanges::Okx
///     type ConcreteType = exchanges::Okx;
///     let instance = ConcreteType::new();
///     instance.name()
/// });
/// assert_eq!(name, "okx");
/// ```
///
/// This allows for executing code that requires the concrete type at compile time,
/// even though the enum variant is only known at runtime.
#[proc_macro]
pub fn concrete(input: TokenStream) -> TokenStream {
    let ConcreteMacroInput {
        enum_instance: _,
        type_param: _,
        code_block: _,
    } = parse_macro_input!(input as ConcreteMacroInput);

    // This is a placeholder implementation
    // In the examples we're using local macro_rules! macros instead
    let expanded = quote! {
        compile_error!("This concrete macro is used for documentation purposes only. The examples use a local macro_rules! for concrete instead.")
    };

    TokenStream::from(expanded)
}

/// A derive macro that implements the mapping between enum variants with associated data and concrete types.
///
/// This derive macro is designed for enums where each variant has associated configuration data and maps to a specific concrete type.
/// Each variant must be annotated with the `#[concrete = "path::to::Type"]` attribute and contain a single tuple field
/// that holds the configuration data for that concrete type.
///
/// The macro generates:
/// 1. A `concrete_type_id` method that returns the `TypeId` of the concrete type for a variant
/// 2. A `concrete_type_name` method that returns the name of the concrete type as a string
/// 3. A `config` method that returns a reference to the configuration data
/// 4. A macro with the snake_case name of the enum + "_config" (with "Config" suffix removed if present)
///    that allows access to both the concrete type and configuration data
///
/// This enables type-level programming with configuration data, where enum variants map to concrete type
/// implementations and carry the configuration needed by those types.
///
/// # Example
/// ```
/// use concrete_type::ConcreteConfig;
///
/// // Define concrete types and configuration types
/// mod exchanges {
///     pub trait ExchangeApi {
///         type Config;
///         fn new(config: Self::Config) -> Self;
///         fn name(&self) -> &'static str;
///     }
///
///     pub struct Binance;
///     pub struct BinanceConfig;
///
///     impl ExchangeApi for Binance {
///         type Config = BinanceConfig;
///         fn new(_: Self::Config) -> Self { Self }
///         fn name(&self) -> &'static str { "binance" }
///     }
///
///     pub struct Okx;
///     pub struct OkxConfig;
///
///     impl ExchangeApi for Okx {
///         type Config = OkxConfig;
///         fn new(_: Self::Config) -> Self { Self }
///         fn name(&self) -> &'static str { "okx" }
///     }
/// }
///
/// // Define the exchange config enum with concrete type mappings and config data
/// #[derive(ConcreteConfig)]
/// enum ExchangeConfig {
///     #[concrete = "exchanges::Binance"]
///     Binance(exchanges::BinanceConfig),
///     #[concrete = "exchanges::Okx"]
///     Okx(exchanges::OkxConfig),
/// }
///
/// // Import the trait for access to its methods
/// use exchanges::ExchangeApi;
///
/// // Using the auto-generated exchange_config! macro:
/// let config = ExchangeConfig::Binance(exchanges::BinanceConfig);
/// let name = exchange_config!(config; (Exchange, config_param) => {
///     // Inside this block:
///     // - Exchange is aliased to exchanges::Binance
///     // - config_param is the BinanceConfig instance
///     Exchange::new(config_param).name()
/// });
/// assert_eq!(name, "binance");
///
/// // Create a different instance for demonstrating methods
/// let config2 = ExchangeConfig::Binance(exchanges::BinanceConfig);
///
/// // Using the ExchangeConfig enum's methods directly
/// // Get the TypeId of the concrete type
/// let type_id = config2.concrete_type_id();
/// // Get the name of the concrete type (as a string)
/// let type_name = config2.concrete_type_name();
/// assert!(type_name.ends_with("exchanges::Binance"));
/// ```
#[proc_macro_derive(ConcreteConfig, attributes(concrete))]
pub fn derive_concrete_config(input: TokenStream) -> TokenStream {
    // This is now deprecated - we'll use the enhanced derive_concrete instead
    derive_concrete(input)
}
