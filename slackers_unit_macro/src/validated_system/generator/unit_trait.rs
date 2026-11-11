use proc_macro2::TokenStream;
use quote::quote;

use crate::validated_system::ValidatedSystem;

impl ValidatedSystem {
    pub(crate) fn generate_unit_trait(&self) -> TokenStream {
        let dim_count = self.base_types.len();
        quote! {
            pub(crate) trait UnitTrait {
                type Underlying: ::slackers_units::conversion::Conversion;
                fn vector() -> [i64; #dim_count];
                fn new(value: Self::Underlying) -> Self;
            }
        }
    }
}
