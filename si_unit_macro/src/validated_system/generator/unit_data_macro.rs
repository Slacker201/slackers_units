use proc_macro2::TokenStream;
use quote::quote;

use crate::validated_system::ValidatedSystem;



impl ValidatedSystem {
    pub(crate) fn generate_unit_macro(&self) -> TokenStream {
        quote! {
            #[macro_export]
            macro_rules! unit {
                ($($data: tt)*) => {
                    ::slackers_si_units::__unit_impl__!(boog, $($data)*)
                }
            }
        }
    }
}