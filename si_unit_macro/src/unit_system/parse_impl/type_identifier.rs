use syn::{Ident, parse::Parse};

use crate::unit_system::TypeIdentifier;

impl Parse for TypeIdentifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        let raw = ident.to_string();
        Ok(Self {
            raw,
            span: ident.span(),
        })
    }
}
