use proc_macro2::TokenStream;
use quote::quote;

use crate::validated_system::ValidatedSystem;

impl ValidatedSystem {
    pub(crate) fn generate_unit_prefix_enums(&self) -> TokenStream {
        let unit_names = &self.unit_enum_names;
        let prefix_names = &self.prefix_enum_names;

        quote! {
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
            pub(crate) enum UnitType {
                #(
                    #unit_names,
                )*
            }
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
            pub(crate) enum Prefix {
                #(
                    #prefix_names,
                    None,
                )*
            }
        }
    }
}
