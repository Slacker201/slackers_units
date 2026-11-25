use proc_macro::TokenStream;

use crate::{compile_parser_impl::compile_parser_impl, sys_impl::system_impl};

mod codegen;
mod compile_parser_impl;
mod data;
mod parsing;
mod sys_impl;
mod validation;

#[proc_macro]
pub fn system(ts: TokenStream) -> TokenStream {
    system_impl(ts.into()).into()
}

#[proc_macro]
pub fn __unit_impl(ts: TokenStream) -> TokenStream {
    compile_parser_impl(ts.into()).into()
}
