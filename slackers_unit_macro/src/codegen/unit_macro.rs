use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitByteStr;

use crate::data::validated_sys::ValidatedSystem;

impl ValidatedSystem {
    pub fn generate_unit_macro_impl(&self) -> TokenStream {
        let byte_str = LitByteStr::new(&self.unit_context.serialize(), Span::call_site());

        quote! {
            macro_rules! unit {
                ($($data: tt)*) => {
                    ::slackers_units::__unit_impl!(#byte_str $($data)*)
                }
            }
        }
    }
}
