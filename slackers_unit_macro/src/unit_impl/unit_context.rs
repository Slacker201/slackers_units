use num_rational::Ratio;
use std::array::from_fn;
use syn::{Error, Ident, LitInt, Token, braced, bracketed, parse::Parse, punctuated::Punctuated};

use crate::unit_system::NumberRepr;

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct UnitContext {
    pub(crate) units: Box<[UnitPiece]>,
    pub(crate) prefixes: Box<[PrefixPiece]>,
}

impl Parse for UnitContext {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let unit_pieces;
        let prefix_pieces;
        braced!(unit_pieces in input);
        braced!(prefix_pieces in input);
        let units: Vec<_> = Punctuated::<UnitPiece, Token![,]>::parse_terminated(&unit_pieces)?
            .into_iter()
            .collect();
        let prefixes: Vec<_> =
            Punctuated::<PrefixPiece, Token![,]>::parse_terminated(&prefix_pieces)?
                .into_iter()
                .collect();
        Ok(Self {
            units: units.into_boxed_slice(),
            prefixes: prefixes.into_boxed_slice(),
        })
    }
}
#[derive(Debug)]
#[allow(unused)]
pub(crate) struct UnitPiece {
    pub(crate) abbrev: String,
    pub(crate) vector: Box<[i64]>,
}

impl Parse for UnitPiece {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let abbrev = input.parse::<Ident>()?.to_string();
        let _ = input.parse::<Token![=]>()?;
        let _ = input.parse::<Token![>]>()?;
        let content;
        bracketed!(content in input);

        let vector = Punctuated::<LitInt, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .map(|litint| litint.base10_parse::<i64>())
            .collect::<Result<_, _>>()?;

        Ok(Self { abbrev, vector })
    }
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct PrefixPiece {
    pub(crate) abbrev: String,
    pub(crate) multiplier: Ratio<NumberRepr>,
}

impl Parse for PrefixPiece {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let abbrev = input.parse::<Ident>()?.to_string();
        let _ = input.parse::<Token![=]>()?;
        let _ = input.parse::<Token![>]>()?;
        let bracketed_stuff;
        bracketed!(bracketed_stuff in input);
        let start_span = bracketed_stuff.span();
        let mut last_span = start_span;
        let mult_bytes: Box<[_]> =
            Punctuated::<LitInt, Token![,]>::parse_terminated(&bracketed_stuff)?
                .iter()
                .map(|value| {
                    last_span = value.span();
                    value.base10_parse::<u8>()
                })
                .collect::<Result<_, _>>()?;
        let span = start_span.join(last_span).unwrap_or(start_span);

        if mult_bytes.len() != size_of::<NumberRepr>() * 2 {
            return Err(Error::new(
                span,
                format!(
                    "Expected vector of len {}, got len {}",
                    size_of::<NumberRepr>(),
                    mult_bytes.len()
                ),
            ));
        }

        let top_bytes: [u8; size_of::<NumberRepr>()] = from_fn(|v| mult_bytes[v]);
        let bottom_bytes: [u8; size_of::<NumberRepr>()] =
            from_fn(|v| mult_bytes[v + size_of::<NumberRepr>()]);

        let multiplier = Ratio::new(
            NumberRepr::from_be_bytes(top_bytes),
            NumberRepr::from_be_bytes(bottom_bytes),
        );

        Ok(PrefixPiece { abbrev, multiplier })
    }
}
