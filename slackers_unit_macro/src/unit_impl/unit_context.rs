use num_rational::Ratio;
use proc_macro2::Span;
use std::{array::from_fn, collections::HashMap};
use syn::{Error, Ident, LitInt, Token, bracketed, parse::Parse, punctuated::Punctuated};

use crate::{prefix_trie::PrefixTrie, unit_impl::DerivedParseData, unit_system::NumberRepr};

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct UnitContext {
    pub(crate) units: Box<[UnitPiece]>,
    pub(crate) prefixes: Box<[PrefixPiece]>,
    pub(crate) unit_trie: PrefixTrie,
    pub(crate) prefix_trie: PrefixTrie,
    pub(crate) unit_lookup_table: HashMap<String, Box<[i64]>>,
    pub(crate) prefix_lookup_table: HashMap<String, Ratio<NumberRepr>>,
    pub(crate) dimension_count: i64,
    pub(crate) call_path: String,
}

impl DerivedParseData {
    pub(crate) fn parse_suffix_to_parts(
        &self,
        suffix: &str,
        span: Span,
    ) -> syn::Result<(Ratio<NumberRepr>, Box<[i64]>)> {
        let (prefix, unit) = self.get_prefix_suffix(suffix, span)?;

        let prefix_multi = prefix
            .and_then(|v| self.prefix_lookup_table.get(&v).copied())
            .unwrap_or(Ratio::ONE);
        let unit_vector = unit
            .and_then(|unit| self.unit_lookup_table.get(&unit).cloned())
            .unwrap_or((0..self.dimension_count).map(|_| 0).collect());

        Ok((prefix_multi, unit_vector))
    }

    fn get_prefix_suffix(
        &self,
        suffix: &str,
        span: Span,
    ) -> syn::Result<(Option<String>, Option<String>)> {
        if suffix.is_empty() {
            return Ok((None, None));
        }

        let bytes = suffix.as_bytes();

        if let Some((prefix, prefix_remainder)) = self.prefix_trie.find_longest(bytes)
            && let Some((unit, unit_remainder)) = self.unit_trie.find_longest(prefix_remainder)
        {
            let prefix = String::from_utf8_lossy(prefix).to_string();
            let unit = String::from_utf8_lossy(unit).to_string();
            if unit_remainder.is_empty() {
                return Ok((Some(prefix), Some(unit)));
            }
            return Err(Error::new(
                span,
                format!(
                    "Found trailing text '{}' after parsing {prefix}{unit}",
                    String::from_utf8_lossy(unit_remainder)
                ),
            ));
        }
        if let Some((unit, unit_remainder)) = self.unit_trie.find_longest(bytes) {
            let unit = String::from_utf8_lossy(unit).to_string();
            if unit_remainder.is_empty() {
                return Ok((None, Some(unit)));
            }
            return Err(Error::new(
                span,
                format!(
                    "Found trailing text '{}' after parsing {unit}",
                    String::from_utf8_lossy(unit_remainder)
                ),
            ));
        }

        Err(Error::new(span, format!("Could not parse {suffix}")))
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
