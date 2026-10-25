use slk_tokenstream::TokenStream;

mod parser;
pub use crate::parser::TotalExpr;
#[test]
fn t() {
    let r = TotalExpr::parse("5/s^4 / mol^16");

    println!("{:?}", r);
}
