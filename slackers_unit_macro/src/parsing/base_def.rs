use syn::{Ident, Token, parse::Parse};

use crate::data::parser_data::{BaseDefinition, base};

impl Parse for BaseDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<base>()?;
        let unit = input.parse::<Ident>()?;
        let _ = input.parse::<Token![;]>()?;

        Ok(Self { unit })
    }
}
