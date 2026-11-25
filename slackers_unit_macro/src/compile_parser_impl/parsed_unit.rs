use quote::{ToTokens, format_ident, quote};
use syn::{Ident, LitByteStr, parse::Parse};

use crate::data::{
    Decimal, Id,
    parser_data::{EqOpt, Op},
    unit_context::UnitContext,
};

#[derive(Debug)]
pub struct ParsedUnit {
    multiplier: Decimal,
    unit_vector: Box<[i32]>,
}

impl Parse for ParsedUnit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let uc = UnitContext::deserialize(&input.parse::<LitByteStr>()?.value());
        let expr = EqOpt::parse_mul(input)?;
        let resolved_expr = expr.resolve_suffixes(&uc)?;
        let (multiplier, unit_vector) = resolved_expr.flatten_unit_vector(&uc)?;

        Ok(Self {
            multiplier,
            unit_vector,
        })
    }
}

impl EqOpt<Id> {
    fn flatten_unit_vector(&self, uc: &UnitContext) -> syn::Result<(Decimal, Box<[i32]>)> {
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
                let mut new_mult = Decimal::ONE;
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
                let (m2, uv) = uc
                    .unit_id_to_vector
                    .get(unit)
                    .cloned()
                    .expect("Should be correct");
                Ok((*multiplier * m2, uv))
            }
        }
    }
}

impl ToTokens for ParsedUnit {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let dw = DecimalWrapper(self.multiplier);
        let uvw = UnitVectorWrapper(&self.unit_vector);

        tokens.extend(quote! {
            {
                let value = #dw;

                Unit::<#uvw f64>::new(value)
            }
        });
    }
}

struct DecimalWrapper(Decimal);
struct UnitVectorWrapper<'a>(&'a [i32]);

impl ToTokens for DecimalWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let integer = quote! { ::slackers_units::Bint::<true, 8, 0, 0> };
        let (numer, denom) = self.0.into_raw();
        let numer_bytes = numer.to_be_bytes();
        let denom_bytes = denom.to_be_bytes();

        tokens.extend(quote! {
            {
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
            .map(|v| {
                match v.cmp(&0) {
                    std::cmp::Ordering::Less => format_ident!("N{}", v.abs().cast_unsigned()),
                    std::cmp::Ordering::Equal => format_ident!("Z0"),
                    std::cmp::Ordering::Greater => format_ident!("P{}", v.cast_unsigned()),
                }
            })
            .collect();

        tokens.extend(quote! {
            #(
                ::slackers_units::typenum::#idents,
            )*
        });
    }
}
