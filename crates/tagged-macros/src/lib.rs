// `proc-macro` crates can only export functions with a `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]` attribute
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Tagged)]
pub fn derive_tagged(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Assume: tuple struct like `struct Email(Tagged<String, EmailTag>);`
    let g = quote! {
        impl std::convert::From<i32> for #struct_name {
            fn from(val: i32) -> Self {
                Self(tagged_core::Tagged::new(val))
            }
        }

        impl From<#struct_name> for i32 {
            fn from(tagged: #struct_name) -> i32 {
                tagged.0.into_inner()
            }
        }

        impl std::ops::Deref for #struct_name {
            type Target = i32;
            fn deref(&self) -> &Self::Target {
                &self.0.value()
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.value())
            }
        }
    };

    g.into()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
