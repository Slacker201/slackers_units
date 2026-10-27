use proc_macro::TokenStream;

use crate::unit_system::System;

mod unit_system;

#[proc_macro]
pub fn system(ts: TokenStream) -> TokenStream {
    let m = System::default();

    println!("{:?}", m);

    ts
}
