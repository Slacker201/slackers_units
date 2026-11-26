use syn::{Ident, Token, bracketed, parse::Parse, punctuated::Punctuated};

use crate::data::parser_data::{PrefixClasses, prefix_classes};

impl Parse for PrefixClasses {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<prefix_classes>()?;
        let _ = input.parse::<Token![=]>()?;

        let bracketed;
        let _ = bracketed!(bracketed in input);
        let _ = input.parse::<Token![,]>()?;

        let allowed_groups: Box<[_]> =
            Punctuated::<Ident, Token![,]>::parse_terminated(&bracketed)?
                .into_iter()
                .map(|v| v.to_string())
                .collect();

        Ok(Self { allowed_groups })
    }
}
