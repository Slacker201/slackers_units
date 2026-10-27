use syn::{Ident, parse::Parse};

use crate::unit_system::TypeIdentifier;

impl Parse for TypeIdentifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            raw: input.parse::<Ident>()?.to_string(),
        })
    }
}
