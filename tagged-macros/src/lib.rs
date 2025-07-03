use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, TypePath};

#[proc_macro_derive(WithId)]
pub fn with_id_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let syn::Data::Struct(data) = &input.data else {
        return quote! {
            compile_error!("WithId can only be used on structs");
        }
            .into();
    };

    let fields = match &data.fields {
        syn::Fields::Named(fields) => {
            let rewritten = fields.named.iter().map(|f| {
                let ident = f.ident.as_ref().unwrap();
                let ty = &f.ty;

                let rewritten_ty = match ty {
                    syn::Type::Path(TypePath { path, .. }) if path.segments.len() == 1 && path.segments[0].ident == "Id" => {
                        // Extract generic type from Id<T>
                        if let syn::PathArguments::AngleBracketed(args) = &path.segments[0].arguments {
                            let inner_ty = &args.args.first().unwrap();
                            quote! { rust_tagged::Tagged<#inner_ty, #name> }
                        } else {
                            quote! { rust_tagged::Tagged<(), #name> } // fallback
                        }
                    }
                    _ => quote! { #ty },
                };

                quote! {
                    pub #ident: #rewritten_ty
                }
            });

            quote! {
                pub struct #name {
                    #( #rewritten ),*
                }
            }
        }

        _ => quote! {
            compile_error!("WithId only supports structs with named fields.");
        },
    };

    fields.into()
}
