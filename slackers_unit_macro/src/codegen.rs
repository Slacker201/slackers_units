use quote::ToTokens;

use crate::data::validated_sys::ValidatedSystem;

mod unit_definition;
mod unit_macro;

impl ToTokens for ValidatedSystem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.generate_unit_definition());
        tokens.extend(self.generate_unit_arithmatic_impl());
        tokens.extend(self.generate_new());
        tokens.extend(self.generate_unit_debug());
        tokens.extend(self.generate_unit_macro_impl());
    }
}
