use std::cmp::Ordering;

use num_rational::Ratio;
use quote::{ToTokens, format_ident, quote};

use crate::{unit_system::NumberRepr, validated_system::ValidatedSystem};

pub(crate) struct DimensionArray(pub Box<[i64]>);
pub(crate) struct DecWrapper(pub Ratio<NumberRepr>);

impl ToTokens for DecWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (numer, denom) = self.0.into_raw();
        let numer = numer.to_be_bytes();
        let denom = denom.to_be_bytes();

        let int_type = ValidatedSystem::get_integer_repr();

        tokens.extend(quote! {
            {
                let numer = #int_type::from_be_bytes(
                    [
                        #(
                            #numer,
                        )*
                    ]
                );
                let denom = #int_type::from_be_bytes(
                    [
                        #(
                            #denom,
                        )*
                    ]
                );
                ::slackers_units::num_rational::Ratio::new(numer, denom)
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
