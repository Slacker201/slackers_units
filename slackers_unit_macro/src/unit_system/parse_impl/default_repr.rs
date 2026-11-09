use proc_macro2::Span;
use syn::{Path, Token, parse::Parse};

use crate::unit_system::parse_impl::kw::default_repr;

#[derive(Debug, Clone)]
pub(crate) struct DefaultRepr {
    pub path: Path,
    #[allow(unused)]
    pub span: Span,
}

impl Parse for DefaultRepr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start = input.parse::<default_repr>()?.span;
        let _ = input.parse::<Token![:]>()?;
        let path = input.parse::<Path>()?;
        let end = input.parse::<Token![;]>()?.span;

        Ok(DefaultRepr {
            path,
            span: start.join(end).unwrap_or(start),
        })
    }
}
