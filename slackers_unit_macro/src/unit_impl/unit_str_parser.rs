use num_rational::Ratio;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Error, Ident, LitInt, Token, parse::discouraged::Speculative};

use crate::{
    unit_impl::{
        DerivedParseData,
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

impl ParsedUnit {
    pub(crate) fn parse(
        input: syn::parse::ParseStream,
        uc: &DerivedParseData,
    ) -> syn::Result<Self> {
        let num = input.parse::<Number>()?;
        let first_modifier = Modifier::parse_first(input, &num.suffix, num.span, uc)?;

        let mut modifiers = vec![first_modifier];
        let fork = input.fork();
        while let Ok(modifier) = Modifier::parse(&fork, uc) {
            modifiers.push(modifier);
            input.advance_to(&fork);
        }

        Ok(Self {
            value: num,
            modifiers,
            call_path: syn::parse_str(&uc.call_path)?,
        })
    }
}

impl Modifier {
    fn parse(input: syn::parse::ParseStream, uc: &DerivedParseData) -> syn::Result<Self> {
        let op = if input.peek(Token![*]) {
            let _ = input.parse::<Token![*]>()?;
            ModOp::Multiply
        } else if input.peek(Token![/]) {
            let _ = input.parse::<Token![/]>()?;
            ModOp::Divide
        } else {
            return Err(Error::new(input.span(), "Expected `*` or `/`"));
        };

        let ident = input.parse::<Ident>()?;
        let (mult, vec) = uc.parse_suffix_to_parts(&ident.to_string(), ident.span())?;
        let pow = Self::parse_pow(input)?;

        Ok(Self { op, mult, vec, pow })
    }

    fn parse_first(
        input: syn::parse::ParseStream,
        suffix: &str,
        span: Span,
        uc: &DerivedParseData,
    ) -> syn::Result<Self> {
        let op = ModOp::Multiply;
        let (mult, vec) = uc.parse_suffix_to_parts(suffix, span)?;
        let pow = Self::parse_pow(input)?;

        Ok(Self { op, mult, vec, pow })
    }

    fn parse_pow(input: syn::parse::ParseStream) -> syn::Result<i64> {
        if input.peek(Token![^]) {
            let _ = input.parse::<Token![^]>()?;
            let litint = input.parse::<LitInt>()?;
            litint.base10_parse::<i64>()
        } else {
            Ok(1)
        }
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
