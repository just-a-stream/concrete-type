#![doc(html_root_url = "https://docs.rs/concrete-type")]

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

    // Combine the macro definition and methods implementation
    let expanded = quote! {
        // Define the macro
        #macro_def
    };

    TokenStream::from(expanded)
}
