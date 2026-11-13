use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::{
    data_transfer::ParserData,
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

    fn unit_context(&self) -> ParserData {
        ParserData::new(
            self.base_types.len(),
            self.units
                .iter()
                .zip(&self.base_types_raw_vectors)
                .map(|(v, vec)| (v.abbreviation.raw.to_owned(), vec.clone()))
                .collect(),
            self.prefixes
                .iter()
                .map(|v| (v.abbreviation.raw.to_owned(), v.multiplier.ratio.to_owned()))
                .collect(),
        )
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
