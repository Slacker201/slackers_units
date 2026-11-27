use std::{
    array::from_fn,
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, LitByteStr, parse::Parse};

use crate::data::{
    Id, RatioRepr,
    parser_data::{EqOpt, Op},
    unit_context::UnitContext,
};
static CACHE: LazyLock<DashMap<u64, Arc<UnitContext>>> = LazyLock::new(DashMap::new);

#[derive(Debug)]
pub struct ParsedUnit<const ALLOW_NUMBER: bool> {
    multiplier: RatioRepr,
    path: String,
    pub unit_vector: Box<[i32]>,
}

impl<const ALLOW_NUMBER: bool> Parse for ParsedUnit<ALLOW_NUMBER> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let hash = input.parse::<Hash>()?;
        let lit_str = &input.parse::<LitByteStr>()?;
        let uc = if let Some(v) = CACHE.get(&hash.0) {
            v
        } else {
            let uc = Arc::new(UnitContext::deserialize(&lit_str.value()));
            CACHE.insert(hash.0, uc);
            CACHE.get(&hash.0).unwrap()
        };

        let true_uc = uc.to_self();
        let expr = EqOpt::parse_mul(input, ALLOW_NUMBER)?;
        let resolved_expr = expr.resolve_suffixes(true_uc)?;
        let (multiplier, unit_vector) = resolved_expr.flatten_unit_vector(&uc)?;

        Ok(Self {
            path: uc.path.clone(),
            multiplier,
            unit_vector,
        })
    }
}

struct Hash(u64);

impl Parse for Hash {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse::<LitByteStr>()?.value();
        let bytes = from_fn(|idx| value[idx]);
        Ok(Self(u64::from_be_bytes(bytes)))
    }
}

impl EqOpt<Id> {
    fn flatten_unit_vector(&self, uc: &UnitContext) -> syn::Result<(RatioRepr, Box<[i32]>)> {
        match self {
            EqOpt::Multiply {
                first,
                next,
                span: _,
            } => {
                let (mut mult, mut uv) = first.flatten_unit_vector(uc)?;
                for (op, next) in next {
                    let (n_mult, n_uv) = next.flatten_unit_vector(uc)?;
                    if *op == Op::Mul {
                        mult *= n_mult;
                        uv.iter_mut().zip(n_uv).for_each(|(s, other)| *s += other);
                    } else {
                        mult /= n_mult;
                        uv.iter_mut().zip(n_uv).for_each(|(s, other)| *s -= other);
                    }
                }
                Ok((mult, uv))
            }
            EqOpt::Pow { lhs, pow, span: _ } => {
                let (old_mult, mut uv) = lhs.flatten_unit_vector(uc)?;
                let mut new_mult = RatioRepr::ONE;
                uv.iter_mut().for_each(|s| *s *= *pow);
                if pow.is_negative() {
                    for _ in 0..pow.abs() {
                        new_mult /= old_mult;
                    }
                } else {
                    for _ in 0..pow.abs() {
                        new_mult *= old_mult;
                    }
                }
                Ok((old_mult, uv))
            }
            EqOpt::Atom {
                multiplier,
                unit,
                span: _,
            } => {
                let (m2, uv) = uc.unit_id_to_vector.get(unit).cloned().unwrap_or((
                    RatioRepr::ONE,
                    uc.unit_id_to_vector
                        .values()
                        .next()
                        .unwrap()
                        .1
                        .iter()
                        .map(|_| 0i32)
                        .collect(),
                ));
                Ok((*multiplier * m2, uv))
            }
        }
    }
}

impl<const ALLOW_NUMBER: bool> ToTokens for ParsedUnit<ALLOW_NUMBER> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let dw = DecimalWrapper(self.multiplier);
        let uvw = UnitVectorWrapper(&self.unit_vector);
        let mut path = self.path.clone();
        if path != "crate" {
            path = format!("crate::{path}");
        }
        let path = path.parse::<TokenStream>().unwrap();

        tokens.extend(quote! {
            {
                let value = #dw;

                #path::Unit::<#uvw f64>::new(::slackers_units::conversion::Conversion::convert(value))
            }
        });
    }
}

struct DecimalWrapper(RatioRepr);
pub struct UnitVectorWrapper<'a>(pub &'a [i32]);

impl ToTokens for DecimalWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let integer = quote! { ::slackers_units::Bint::<true, 8, 0, 0> };
        let (numer, denom) = self.0.into_raw();
        let numer_bytes = numer.to_be_bytes();
        let denom_bytes = denom.to_be_bytes();

        tokens.extend(quote! {
            const {
                let numer = #integer::from_be_bytes([
                    #(
                        #numer_bytes,
                    )*
                ]);
                let denom = #integer::from_be_bytes([
                    #(
                        #denom_bytes,
                    )*
                ]);

                ::slackers_units::Ratio::new_raw(numer, denom)
            }
        });
    }
}

impl ToTokens for UnitVectorWrapper<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let idents: Box<[Ident]> = self
            .0
            .iter()
            .map(|v| match v.cmp(&0) {
                std::cmp::Ordering::Less => format_ident!("N{}", v.abs().cast_unsigned()),
                std::cmp::Ordering::Equal => format_ident!("Z0"),
                std::cmp::Ordering::Greater => format_ident!("P{}", v.cast_unsigned()),
            })
            .collect();

        tokens.extend(quote! {
            #(
                ::slackers_units::typenum::#idents,
            )*
        });
    }
}

impl UnitContext {
    fn to_self(&self) -> &Self {
        self
    }
}

impl<const V: bool> ParsedUnit<V> {
    pub fn get_path(&self) -> TokenStream {
        self.path.parse().unwrap()
    }
}
