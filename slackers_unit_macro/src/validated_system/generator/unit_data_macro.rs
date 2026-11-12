use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::{
    unit_impl::unit_context::{PrefixPiece, UnitContext, UnitPiece},
    validated_system::ValidatedSystem,
};

impl ValidatedSystem {
    pub(crate) fn generate_unit_macro(&self) -> TokenStream {
        let unit_context = self.unit_context();
        let call_site = &self.call_site;

        quote! {
            macro_rules! unit {
                ($($data:tt)*) => {
                    ::slackers_units::__unit_impl__!(
                        #unit_context
                        #call_site
                        $($data)*
                    )
                }
            }
        }
    }

    fn unit_context(&self) -> UnitContext {
        let unit_pieces = self
            .base_type_mapping
            .iter()
            .enumerate()
            .map(|(idx, v)| {
                let abbrev = self.units[*v].abbreviation.raw.clone();
                let vector = self.base_types_raw_vectors[idx].clone();

                UnitPiece { abbrev, vector }
            })
            .collect();
        let prefix_pieces = self
            .prefixes
            .iter()
            .map(|prefix| PrefixPiece {
                abbrev: prefix.abbreviation.raw.clone(),
                multiplier: prefix.multiplier.ratio,
            })
            .collect();
        UnitContext::from_minimal(unit_pieces, prefix_pieces, "".to_string())
    }
}

impl ToTokens for UnitContext {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let units = &self.units;
        let prefixes = &self.prefixes;
        tokens.extend(quote! {
            {
                #(#units,)*
            }
            {
                #(#prefixes,)*
            }
        });
    }
}

impl ToTokens for UnitPiece {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let a_ident = format_ident!("{}", &self.abbrev);
        let vec = &self.vector;
        tokens.extend(quote! {
            #a_ident => [
                #(#vec,)*
            ]
        });
    }
}

impl ToTokens for PrefixPiece {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let a_ident = format_ident!("{}", &self.abbrev);
        let (numer, denom) = self.multiplier.into_raw();
        let numer = numer.to_be_bytes();
        let denom = denom.to_be_bytes();
        tokens.extend(quote! {
            #a_ident => [
                #(
                    #numer,
                )*
                #(
                    #denom,
                )*
            ]
        });
    }
}
