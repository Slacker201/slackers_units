#![warn(clippy::pedantic)]
use proc_macro::TokenStream;

use crate::unit_system::System;

mod prefix_trie;
pub(crate) mod system_impl;
mod unit_system;
mod validated_system;

mod unit_impl;

#[proc_macro]
pub fn system(ts: TokenStream) -> TokenStream {
    system_impl::system_impl(ts.into()).into()
}

#[proc_macro]
pub fn __unit_impl__(ts: TokenStream) -> TokenStream {
    unit_impl::unit_impl(ts.into()).into()
}