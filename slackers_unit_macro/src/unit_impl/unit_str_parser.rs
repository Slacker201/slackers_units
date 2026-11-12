use num_rational::Ratio;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::Error;

use crate::{
    prefix_trie::PrefixTrie,
    unit_impl::{
        unit_context::UnitContext,
        unit_str_parser::decimal_constructor::{DecWrapper, DimensionArray},
    },
    unit_system::{Number, NumberRepr},
};

pub(crate) mod decimal_constructor;

#[derive(Debug)]
pub struct ParsedUnit {
    call_path: TokenStream,
    value: Number,
    modifiers: Vec<Modifier>,
}

#[derive(Debug)]
struct Modifier {
    op: ModOp,               // Operation
    mult: Ratio<NumberRepr>, // PrefixMultiplier
    vec: Box<[i64]>,         // Unit vector
    pow: i64,                // Group power
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModOp {
    Multiply,
    #[allow(unused)]
    Divide,
}

impl Modifier {
    #[allow(clippy::large_types_passed_by_value)]
    fn new(op: ModOp, mult: Ratio<NumberRepr>, vec: Box<[i64]>, pow: i64) -> Self {
        Self { op, mult, vec, pow }
    }
}

impl ParsedUnit {
    pub fn parse(input: syn::parse::ParseStream, context: &UnitContext) -> syn::Result<Self> {
        let num = input.parse::<Number>()?;

        let suffix = num.suffix.clone();

        let (prefix, suffix) =
            Self::get_prefix_suffix(num.span, &suffix, &context.prefix_trie, &context.unit_trie)?;

        let unit_vector = suffix
            .and_then(|suffix| context.unit_lookup_table.get(&suffix as &str))
            .map_or(
                (0..context.dimension_count)
                    .into_iter()
                    .map(|_| 0)
                    .collect(),
                |vector| vector.clone(),
            );
        let prefix_multi = prefix
            .and_then(|prefix| context.prefix_lookup_table.get(&prefix as &str).copied())
            .unwrap_or(Ratio::from_integer(bnum::n!(1)));

        Ok(Self {
            value: num,
            modifiers: vec![Modifier::new(ModOp::Multiply, prefix_multi, unit_vector, 1)],
            call_path: syn::parse_str(&context.call_path)?,
        })
    }

    fn get_prefix_suffix(
        span: Span,
        suffix: &str,
        ptrie: &PrefixTrie,
        utrie: &PrefixTrie,
    ) -> syn::Result<(Option<String>, Option<String>)> {
        if suffix.is_empty() {
            return Ok((None, None));
        }

        let bytes = suffix.as_bytes();

        if let Some((prefix, prefix_remainder)) = ptrie.find_longest(bytes)
            && let Some((unit, unit_remainder)) = utrie.find_longest(prefix_remainder)
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
        if let Some((unit, unit_remainder)) = utrie.find_longest(bytes) {
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

impl ToTokens for ParsedUnit {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut value = self.value.ratio;
        let unit_vec = self.modifiers.iter().fold(vec![], |mut a, modifier| {
            if a.len() != modifier.vec.len() {
                a.resize(modifier.vec.len(), 0);
            }
            for (idx, q) in modifier.vec.iter().enumerate() {
                if modifier.op == ModOp::Multiply {
                    a[idx] += *q * modifier.pow;
                } else {
                    a[idx] -= *q * modifier.pow;
                }
            }

            let pow_mult = (0..modifier.pow.abs().saturating_sub(1))
                .fold(modifier.mult, |a, _| a * modifier.mult);
            value *= if modifier.pow.is_negative() {
                Ratio::ONE / pow_mult
            } else {
                pow_mult
            };
            a
        });

        let r = DecWrapper(value);
        let dvec = DimensionArray(unit_vec.into_boxed_slice());
        let call_path = &self.call_path;

        tokens.extend(quote! {
            {
                #call_path::Unit::<#dvec>::new(
                    ::slackers_units::conversion::Conversion::convert(#r)
                )
            }
        });
    }
}
