use proc_macro::TokenStream;
use quote::quote;

use crate::{compile_parser_impl::compile_parser_impl, sys_impl::system_impl};

mod codegen;
mod data;
mod parsing;
mod sys_impl;
mod validation;
mod compile_parser_impl;

#[proc_macro]
pub fn system(ts: TokenStream) -> TokenStream {
    system_impl(ts.into()).into()
}

#[proc_macro]
pub fn __unit_impl(ts: TokenStream) -> TokenStream {
    compile_parser_impl(ts.into()).into()
}