use fastnum::decimal::Decimal;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::{
    unit_impl::unit_context::{PrefixPiece, UnitContext, UnitPiece},
    validated_system::ValidatedSystem,
};

impl ValidatedSystem {
    pub(crate) fn generate_unit_macro(&self) -> TokenStream {
        let unit_context = self.unit_context();

        quote! {
            #[macro_export]
            macro_rules! unit {
                ($ ($data : tt) *) => {
                    ::slackers_units::__unit_impl__!(
                        #unit_context
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
                multiplier: prefix.multiplier.value,
            })
            .collect();
        UnitContext {
            units: unit_pieces,
            prefixes: prefix_pieces,
        }
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
        let bytes = dec8_to_bytes(&self.multiplier);
        tokens.extend(quote! {
            #a_ident => [
                #(
                    #bytes,
                )*
            ]
        });
    }
}

fn dec8_to_bytes(d: &Decimal<8>) -> [u8; size_of::<Decimal<8>>()] {
    let mut buf: [u8; size_of::<Decimal<8>>()] = [0; size_of::<Decimal<8>>()];

    // use this because serializing and deserializing parts into tokens and back is complex and error-prone
    unsafe {
        std::ptr::copy_nonoverlapping(
            std::ptr::from_ref(d).cast::<u8>(),
            buf.as_mut_ptr(),
            size_of::<Decimal<8>>(),
        );
    }

    buf
}
