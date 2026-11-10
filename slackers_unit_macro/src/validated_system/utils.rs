use proc_macro2::TokenStream;
use quote::quote;

use crate::validated_system::ValidatedSystem;

impl ValidatedSystem {
    pub fn get_integer_repr() -> TokenStream {
        let amount: usize = cfg_select! {
            feature = "converter_256" => {
                256
            }
            feature = "converter_128" => {
                128
            }
            feature = "converter_64" => {
                64
            }
            feature = "converter_32" => {
                32
            }
            feature = "converter_16" => {
            16
            }
            feature = "converter_8" => {
                8
            }
            feature = "converter_4" => {
                4
            }
            feature = "converter_2" => {
                2
            }
            feature = "converter_1" => {
                1
            }
            _ => {
                8
            }
        };

        quote! {
            ::slackers_units::bnum::Integer::<true, #amount, 0, 0>
        }
    }
}
