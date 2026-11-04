use std::{cmp::Ordering, collections::HashMap};

use fastnum::decimal::{Decimal, Sign};
use proc_macro2::Span;
use quote::{ToTokens, format_ident, quote};
use syn::Error;

use crate::{prefix_trie::PrefixTrie, unit_system::Number};

#[derive(Debug)]
pub struct ParsedUnit {
    value: Number,
    modifiers: Vec<(
        ModOp,      // Operation
        Decimal<8>, // PrefixMultiplier
        Box<[i64]>, // Unit vector
        i64,        // Group power
    )>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModOp {
    Multiply,
    #[allow(unused)]
    Divide,
}

/*
How do we want our modifiers?

(Option<PrefixMulti>, Unit, Power)
*/

impl ParsedUnit {
    pub fn parse(
        input: syn::parse::ParseStream,
        unit_trie: &PrefixTrie,
        prefix_trie: &PrefixTrie,
        unit_lookup_table: &HashMap<&str, &[i64]>,
        prefix_lookup_table: &HashMap<&str, Decimal<8>>,
        dimension_count: i64,
    ) -> syn::Result<Self> {
        let num = input.parse::<Number>()?;

        let suffix = num.suffix.clone();

        let (prefix, suffix) = Self::get_prefix_suffix(num.span, &suffix, prefix_trie, unit_trie)?;

        let unit_vector = suffix
            .and_then(|suffix| unit_lookup_table.get(&suffix as &str))
            .map_or((0..dimension_count).into_iter().map(|_| 0).collect(), |v| Box::from(*v));
        let prefix_multi = prefix
            .clone()
            .and_then(|prefix| prefix_lookup_table.get(&prefix as &str).copied())
            .unwrap_or_else(|| {
                Decimal::from_i32(1)
            });

        Ok(Self {
            value: num,
            modifiers: vec![(ModOp::Multiply, prefix_multi, unit_vector, 1)],
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

        let mut pstate = PState::ParsingPrefix { all_bytes: bytes };

        loop {
            match pstate {
                PState::ParsingPrefix { all_bytes } => {
                    if let Some((mut prefix, remainder)) = ptrie.find_longest(all_bytes) {
                        prefix.reverse();
                        let actual_prefix = String::from_utf8_lossy(&prefix).into_owned();

                        pstate = PState::ParsedPrefix {
                            prefix: actual_prefix,
                            remainder_bytes: remainder,
                            all_bytes,
                        }
                    } else {
                        pstate = PState::ParsingUnit { all_bytes }
                    }
                }
                PState::ParsedPrefix {
                    prefix,
                    remainder_bytes,
                    all_bytes,
                } => {
                    if let Some((mut unit, remainder)) = utrie.find_longest(remainder_bytes) {
                        if remainder.is_empty() {
                            unit.reverse();
                            let actual_unit = String::from_utf8_lossy(&unit).into_owned();
                            return Ok((Some(prefix), Some(actual_unit)));
                        }
                        return Err(Error::new(span, "Failed to parse {suffix}"));
                    } 
                    pstate = PState::ParsingUnit { all_bytes }
                }
                PState::ParsingUnit { all_bytes } => {
                    if let Some((mut unit, remainder)) = utrie.find_longest(all_bytes)
                        && remainder.is_empty()
                    {
                        unit.reverse();
                        let actual_unit = String::from_utf8_lossy(&unit).into_owned();
                        return Ok((None, Some(actual_unit)));
                    }
                    return Err(Error::new(span, format!("Failed to parse {suffix}")));
                }
            }
        }
    }
}

enum PState<'a> {
    ParsingPrefix {
        all_bytes: &'a [u8],
    },
    ParsedPrefix {
        prefix: String,
        remainder_bytes: &'a [u8],
        all_bytes: &'a [u8],
    },
    ParsingUnit {
        all_bytes: &'a [u8],
    },
}

struct DecWrapper(Decimal<8>);
struct DimensionArray(Box<[i64]>);

impl ToTokens for DecWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let v = self.0;

        let uint = v.digits();
        let digits = uint.digits();
        let exp = -i32::from(v.fractional_digits_count());
        let sign = v.sign();

        let sign_toks = if sign == Sign::Minus {
            quote! {
                ::slackers_units::fastnum::decimal::Sign::Minus
            }
        } else {
            quote! {
                ::slackers_units::fastnum::decimal::Sign::Plus
            }
        };

        tokens.extend(quote! {
            {
                let uint = ::slackers_units::fastnum::bint::UInt::<8>::from_digits(
                    [
                        #(
                            #digits,
                        )*
                    ]
                );

                ::slackers_units::fastnum::decimal::Decimal::<8>::from_parts(
                    uint,
                    #exp,
                    #sign_toks,
                    ::slackers_units::fastnum::decimal::Context::default()
                )
            }
        });
    }
}

impl ToTokens for ParsedUnit {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut value = self.value.value;
        let unit_vec = self.modifiers.iter().fold(
            vec![],
            |mut a, (operation, multiplier, vector, group_power)| {
                if a.len() != vector.len() {
                    a.resize(vector.len(), 0);
                }
                for (idx, q) in vector.iter().enumerate() {
                    if *operation == ModOp::Multiply {
                        a[idx] += *q * group_power;
                    } else {
                        a[idx] -= *q * group_power;
                    }
                }

                value *= multiplier.powi(i32::try_from(*group_power).unwrap_or({
                    if *group_power < 0 {
                        i32::MIN
                    } else {
                        i32::MAX
                    }
                }));
                a
            },
        );

        let r = DecWrapper(value);

        let dvec = DimensionArray(unit_vec.into_boxed_slice());

        tokens.extend(quote! {
            {
                crate::Unit::<#dvec>::new(
                    #r.to_f64()
                )
            }
        });
    }
}

impl ToTokens for DimensionArray {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let idents = self.0.iter().map(|value| match value.cmp(&0) {
            Ordering::Less => {
                format_ident!("N{}", value.unsigned_abs())
            }
            Ordering::Equal => {
                format_ident!("Z0")
            }
            Ordering::Greater => {
                format_ident!("P{}", value.cast_unsigned())
            }
        });

        tokens.extend(quote! {
            #(
                ::slackers_units::#idents,
            )*
        });
    }
}
