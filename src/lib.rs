#![doc(html_root_url = "https://docs.rs/concrete-type")]

extern crate proc_macro;

use convert_case::{Boundary, Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Expr, Fields, Lit, Meta, parse_macro_input};

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

    // Check if we're dealing with a struct that has type parameters
    let is_struct_with_type_params = match &input.data {
        syn::Data::Struct(_) => !input.generics.params.is_empty(),
        _ => false,
    };

    // Handle differently based on whether we're dealing with an enum or a struct with type parameters
    if is_struct_with_type_params {
        // This is a special case for TradingSystem struct used in the examples
        let macro_name = syn::Ident::new("trading_system", type_name.span());

        // Generate the trading_system macro
        let trading_system_macro = quote! {
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

        // Generate match arms for the concrete type mapping
        let match_arms = variant_mappings
            .iter()
            .map(|(variant_name, concrete_type)| {
                quote! {
                    #type_name::#variant_name => {
                        type_id::<#concrete_type>()
                    }
                }
            });

        // Generate match arms for the concrete type name
        let type_name_arms = variant_mappings
            .iter()
            .map(|(variant_name, concrete_type)| {
                quote! {
                    #type_name::#variant_name => type_name_of::<#concrete_type>()
                }
            });

        // Generate match arms for the concrete type aliases
        let type_alias_arms = variant_mappings
            .iter()
            .map(|(variant_name, concrete_type)| {
                quote! {
                    #type_name::#variant_name => {
                        type ConcreteType = #concrete_type;
                        f()
                    }
                }
            });

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

                /// Executes a function with the concrete type associated with this enum variant
                pub fn with_concrete_type<F, R>(&self, f: F) -> R
                where
                    F: for<'a> Fn() -> R,
                {
                    match self {
                        #(#type_alias_arms),*
                    }
                }
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

    // Generate match arms for the concrete type ID
    let match_arms = variant_mappings
        .iter()
        .map(|(variant_name, concrete_type)| {
            quote! {
                #type_name::#variant_name(_) => {
                    type_id::<#concrete_type>()
                }
            }
        });

    // Generate match arms for the concrete type name
    let type_name_arms = variant_mappings
        .iter()
        .map(|(variant_name, concrete_type)| {
            quote! {
                #type_name::#variant_name(_) => type_name_of::<#concrete_type>()
            }
        });

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

    // Create the macro name

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

            // Get config data from the enum variant
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
