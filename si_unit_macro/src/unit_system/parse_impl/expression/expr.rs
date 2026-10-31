use proc_macro2::Span;
use syn::{Ident, LitFloat, LitInt};

mod m_expr;
mod p_expr;
mod atom;


#[allow(unused)]
#[derive(Debug, Clone)]
enum PExpr {
    Atom {
        atom: Atom,
        span: Span,
    },
    Pow {
        atom: Atom,
        amount: LitInt,
        span: Span,
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MExpr {
    first: PExpr,
    next: Box<[(MOp, PExpr)]>,

    span: Span,
}

#[derive(Debug, Clone)]
enum MOp {
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum Atom {
    Item {
        item: Item,
        span: Span,
    },
    Parenthesized {
        m_expr: Box<MExpr>,
        span: Span,
    }
}

#[derive(Debug, Clone)]
enum Item {
    LitInt {
        value: LitInt,
    },
    LitFloat {
        value: LitFloat,
    },
    Unit {
        value: Ident,
    }
}