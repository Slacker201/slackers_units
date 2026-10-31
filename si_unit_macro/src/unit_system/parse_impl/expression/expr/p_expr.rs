use proc_macro2::Span;
use syn::{Error, LitInt, Token, parse::Parse};

use crate::unit_system::parse_impl::expression::expr::{Atom, PExpr};




impl Parse for PExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let atom = input.parse::<Atom>()?;

        if input.parse::<Token![^]>().is_ok() {
            let amount = input.parse::<LitInt>()?;

            if amount.suffix().is_empty() {
                let span = atom.span().join(amount.span()).unwrap_or(atom.span());

                Ok(
                    Self::Pow { atom, amount, span }
                )
            } else {
                Err(
                    Error::new(
                        amount.span(), 
                        "Suffixed powers are not supported"
                    )
                )
            }
        } else {
            Ok(
                Self::Atom { span: atom.span(), atom,  }
            )
        }
    }
}

impl PExpr {
    pub(crate) fn span(&self) -> Span {
        match self {
            PExpr::Atom { atom: _, span } => *span,
            PExpr::Pow { atom: _, amount: _, span } => *span,
        }
    }

    pub(crate) fn contains_power(&self) -> Option<Span> {
        match self {
            PExpr::Atom { atom, span: _ } => atom.contains_power(),
            PExpr::Pow { atom: _, amount: _, span } => Some(*span),
        }
    }
}