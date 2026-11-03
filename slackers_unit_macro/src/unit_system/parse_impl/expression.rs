use syn::parse::Parse;

use crate::unit_system::{Expression, parse_impl::expression::expr::MExpr};

pub(crate) mod expr;

impl Parse for Expression {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let m_expr = input.parse::<MExpr>()?;

        Ok(Self {
            span: m_expr.span(),
            expr: m_expr,
        })
    }
}
