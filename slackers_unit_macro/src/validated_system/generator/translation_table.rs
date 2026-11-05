use proc_macro2::TokenStream;
use quote::quote;

use crate::validated_system::ValidatedSystem;

impl ValidatedSystem {
    pub(crate) fn translation_table(&self) -> TokenStream {
        let vector_len = self.base_type_mapping.len();
        let toks = self
            .base_type_mapping
            .iter()
            .enumerate()
            .map(|(vector_pos, index)| {
                let mut vector: Box<[_]> = (0..vector_len).map(|_| 0i64).collect();

                vector[vector_pos] = 1;

                let enum_name = &self.unit_enum_names[*index];

                quote! {
                    (
                        crate::UnitType::#enum_name,
                        [
                            #(
                                #vector,
                            )*
                        ]
                    )
                }
            });

        quote! {
            const __TRANSLATION_TABLE__: [(crate::UnitType, [i64; #vector_len]); #vector_len] = [
                #(
                    #toks,
                )*
            ];
        }
    }
}
