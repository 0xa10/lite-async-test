use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn async_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let new_name = format!("{}_wrapper", name);
    let new_ident = syn::Ident::new(&new_name, name.span());

    let output = quote! {
        #input

        #[test]
        fn #new_ident() {
            futures_lite::future::block_on(async {
                #name().await;
            });
        }
    };

    TokenStream::from(output)
}


