use quote::format_ident;
use syn::Ident;

use crate::data::parser_data::System;
type Operand1 = Box<[Ident]>;
type Operand2 = Box<[Ident]>;
type Result = Box<[Ident]>;
impl System {
    pub fn get_unit_generic_names(&self) -> Box<[Ident]> {
        (0..self.bases.len())
            .map(|idx| format_ident!("V{idx}"))
            .collect()
    }
    pub fn get_unit_phantom_data_names(&self) -> Box<[Ident]> {
        (0..self.bases.len())
            .map(|idx| format_ident!("_pd_marker_{idx}"))
            .collect()
    }
    pub fn get_unit_arith_generic_names(&self) -> (Operand1, Operand2, Result) {
        (
            (0..self.bases.len())
                .map(|idx| format_ident!("Operand1{idx}"))
                .collect(),
            (0..self.bases.len())
                .map(|idx| format_ident!("Operand2{idx}"))
                .collect(),
            (0..self.bases.len())
                .map(|idx| format_ident!("Result{idx}"))
                .collect(),
        )
    }
}
