use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(ValidatedFields)]
pub fn derive_validated_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const DERIVATION: &str = "ValidatedFields";

    let input = parse_macro_input!(input as DeriveInput);
    let span = input.span();

    let vis = input.vis;
    let original_name = input.ident;
    let derived_name = format_ident!("{}{DERIVATION}", original_name);

    let compile_error =
        |message| proc_macro::TokenStream::from(Error::new(span, message).to_compile_error());

    let original_fields = match input.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => {
                if input.generics.params.is_empty() {
                    fields
                } else {
                    return compile_error(format!(
                        "{DERIVATION} is not supported for structs with generic parameters"
                    ));
                }
            }
            syn::Fields::Unnamed(_) => {
                return compile_error(format!("{DERIVATION} is not supported for tuple structs"));
            }
            syn::Fields::Unit => {
                return compile_error(format!("{DERIVATION} is not supported for unit structs"));
            }
        },
        syn::Data::Enum(_) => {
            return compile_error(format!("{DERIVATION} is not supported for enums"));
        }
        syn::Data::Union(_) => {
            return compile_error(format!("{DERIVATION} is not supported for unions"));
        }
    };

    let derived_fields = original_fields.named.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! { pub #field_ident: ::validated::Validated<#field_ty, E>, }
    });

    let derived_field_initializers = original_fields.named.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { #field_ident: ::validated::Validated::Good(good.#field_ident), }
    });

    let good_values = original_fields.named.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! { #field_ident: ::core::option::Option<#field_ty>, }
    });

    let partition = original_fields.named.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! {
            match validated_fields.#field_ident {
                ::validated::Validated::Good(value) => {
                    good_values.#field_ident = Some(value);
                }
                ::validated::Validated::Fail(errors) => {
                    all_errors.extend(errors);
                }
            };
        }
    });

    let unwrap_good_values = original_fields.named.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! { #field_ident: good_values.#field_ident.unwrap(), }
    });

    proc_macro::TokenStream::from(quote! {
        #vis struct #derived_name<E> {
            #(#derived_fields)*
        }

        impl From<#original_name> for #derived_name<::std::convert::Infallible> {
            fn from(good: #original_name) -> Self {
                Self {
                    #(#derived_field_initializers)*
                }
            }
        }

        impl<E> From<#derived_name<E>> for ::validated::Validated<#original_name, E> {
            fn from(validated_fields: #derived_name<E>) -> Self {
                #[derive(::core::default::Default)]
                struct GoodValues {
                    #(#good_values)*
                }

                let mut good_values = GoodValues::default();
                let mut all_errors = ::std::vec::Vec::new();

                #(#partition)*

                match ::nonempty_collections::vector::NEVec::from_vec(all_errors) {
                    None => ::validated::Validated::Good(#original_name {
                        #(#unwrap_good_values)*
                    }),
                    Some(nonempty_errors) => ::validated::Validated::Fail(nonempty_errors),
                }
            }
        }
    })
}
