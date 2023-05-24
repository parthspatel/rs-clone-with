extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derives `CloneWith` for the struct.
///
/// # Note
///
/// The struct must implement `Clone` to correctly use `CloneWith`.
///
/// If your struct does not already implement `Clone`, you must add `#[derive(Clone)]` to your struct definition.
///
/// # Example
///
/// ```rust
/// #[derive(Clone, CloneWith)]
/// struct MyStruct {
///     my_field: u32,
/// }
/// ```
///
/// This will generate a `with_field` method:
///
/// ```rust
/// let original = MyStruct { field: 1 };
/// let modified = original.with_my_field(2);
/// assert_eq!(original.my_field, 1);
/// assert_eq!(modified.my_field, 2);
/// ```
#[proc_macro_derive(CloneWith)]
pub fn clone_with_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used to collect all generated methods
    let mut method_tokens = proc_macro2::TokenStream::new();

    // We only support named fields for now
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in &fields.named {
                let ident = &field.ident;
                let ty = &field.ty;
                let method_ident = syn::Ident::new(&format!("with_{}", ident.as_ref().unwrap()), proc_macro2::Span::call_site());

                let method_token = quote! {
                    pub fn #method_ident(&self, value: #ty) -> Self {
                        let mut new = self.clone();
                        new.#ident = value;
                        new
                    }
                };

                method_tokens.extend(method_token);
            }
        }
    }

    let name = &input.ident;

    // Generate the final expanded code
    let expanded = quote! {
        impl #name {
            #method_tokens
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}