use proc_macro::TokenStream;

use crate::sys_impl::system_impl;

mod codegen;
mod data;
mod parsing;
mod sys_impl;
mod validation;

#[proc_macro]
pub fn system(ts: TokenStream) -> TokenStream {
    system_impl(ts.into()).into()
}
