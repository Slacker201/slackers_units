use proc_macro2::Span;
use syn::{Ident, Token, bracketed, parse::Parse, punctuated::Punctuated};

use crate::data::{
    Id,
    parser_data::{Aliases, aliases},
    registries::NameRegistry,
};

impl Aliases {
    pub fn new_empty() -> Self {
        Self {
            aliases: Box::default(),
            span: Span::call_site(),
        }
    }
}

impl Parse for Aliases {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<aliases>()?.span;
        let _ = input.parse::<Token![=]>()?;
        let body;
        let _ = bracketed!(body in input);
        let end_span = input.parse::<Token![,]>()?.span;

        let aliases = Punctuated::<Ident, Token![,]>::parse_terminated(&body)?
            .into_iter()
            .collect();

        Ok(Self {
            aliases,
            span: start_span.join(end_span).unwrap_or(start_span),
        })
    }
}

impl Aliases {
    pub fn register_aliases(&self, id: Id, ur: &mut NameRegistry) {
        for alias in &self.aliases {
            ur.add_alias(id, alias.to_string(), alias.span());
        }
    }
}
