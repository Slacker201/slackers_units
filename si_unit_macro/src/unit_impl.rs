use proc_macro2::TokenStream;
use quote::quote;



mod unit_context;

pub(crate) fn unit_impl(tokens: TokenStream) -> TokenStream {
    eprintln!("{tokens}");
    quote! {
        {
            0
        }
    }
}