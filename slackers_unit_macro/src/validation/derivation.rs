use quote::format_ident;
use syn::Ident;

use crate::data::{parser_data::System, validated_sys::ValidatedSystem};



impl System {
    pub fn get_unit_generic_names(&self) -> Box<[Ident]> {
        (0..self.bases.len()).map(|idx| format_ident!("V{idx}")).collect()
    }
    pub fn get_unit_phantom_data_names(&self) -> Box<[Ident]> {
        (0..self.bases.len()).map(|idx| format_ident!("_pd_marker_{idx}")).collect()
    }
    pub fn get_unit_arith_generic_names(&self) -> (Box<[Ident]>, Box<[Ident]>, Box<[Ident]>) {
        (
            (0..self.bases.len()).map(|idx| format_ident!("Operand1{idx}")).collect(),
            (0..self.bases.len()).map(|idx| format_ident!("Operand2{idx}")).collect(),
            (0..self.bases.len()).map(|idx| format_ident!("Result{idx}")).collect()
        )
    }
}