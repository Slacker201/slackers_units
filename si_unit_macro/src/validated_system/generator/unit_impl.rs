use proc_macro2::TokenStream;
use quote::quote;

use crate::validated_system::ValidatedSystem;

impl ValidatedSystem {
    pub(crate) fn generate_units(&self) -> TokenStream {
        let dv_names = &self.dimensional_vector_names;
        let dv_pd_names = &self.dimensional_phantom_data_names;

        quote! {
            struct Unit<#(#dv_names,)*> where
                #(
                    #dv_names: slackers_si_units::Integer,
                )*
            {
                #(#dv_pd_names: std::marker::PhantomData<#dv_names>,)*
                value: f64,
            }
        }
    }
}
