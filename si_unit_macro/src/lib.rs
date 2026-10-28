use proc_macro::TokenStream;

use crate::unit_system::System;

mod prefix_trie;
mod system_impl;
mod unit_system;

#[proc_macro]
pub fn system(ts: TokenStream) -> TokenStream {
    system_impl::system_impl(ts.into()).into()
}
