use std::fmt::Debug;

use syn::{Error, Ident, LitFloat, LitInt, Token, parenthesized, parse::Parse};

use crate::unit_system::Expression;

impl Parse for Expression {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let q = input.parse::<MulExpr>()?;
        eprintln!("{:?}", q);
        Err(Error::new(input.span(), "test"))
    }
}
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum MulExpr {
    Pow { p_expr: PowerExpr },
    Mul { lhs: PowerExpr, rhs: PowerExpr },
    Div { lhs: PowerExpr, rhs: PowerExpr },
}
#[derive(Clone)]
pub enum PowerExpr {
    Atom { atom: Atom },
    Expr { atom: Atom, num: LitInt },
}
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Atom {
    Item { item: Item },
    MulExpr { expr: Box<MulExpr> },
}
#[derive(Clone)]
pub enum Item {
    SuffixedFloat { value: LitFloat, suffix: String },
    SuffixedInt { value: LitInt, suffix: String },
    Unit { value: Ident },
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SuffixedFloat { value, suffix } => f
                .debug_struct("SuffixedFloat")
                .field("value", &value.to_string())
                .field("suffix", suffix)
                .finish(),
            Self::SuffixedInt { value, suffix } => f
                .debug_struct("SuffixedInt")
                .field("value", &value.to_string())
                .field("suffix", suffix)
                .finish(),
            Self::Unit { value } => f.debug_struct("Unit").field("value", value).finish(),
        }
    }
}

impl Debug for PowerExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom { atom } => f.debug_struct("Atom").field("atom", atom).finish(),
            Self::Expr { atom, num } => f
                .debug_struct("Expr")
                .field("atom", atom)
                .field("num", &num.to_string())
                .finish(),
        }
    }
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitFloat) {
            let value = input.parse::<LitFloat>()?;
            let suffix = value.suffix().to_string();
            Ok(Self::SuffixedFloat { value, suffix })
        } else if input.peek(LitInt) {
            let value = input.parse::<LitInt>()?;
            let suffix = value.suffix().to_string();
            Ok(Self::SuffixedInt { value, suffix })
        } else if input.peek(Ident) {
            Ok(Self::Unit {
                value: input.parse()?,
            })
        } else {
            Err(Error::new(
                input.span(),
                "Failed to find LitFloat, LitInt, or Identifier",
            ))
        }
    }
}

impl Parse for PowerExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<Item>()?;
        if input.peek(Token![^]) {
            let _ = input.parse::<Token![^]>()?;
            let number = input.parse::<Item>()?;

            let (num, s) = match number {
                Item::SuffixedInt { value, suffix } => (value, suffix),
                Item::Unit { value } => {
                    return Err(Error::new(
                        value.span(),
                        "Only unitless integers are allowed as a power",
                    ));
                }
                Item::SuffixedFloat { value, suffix: _ } => {
                    return Err(Error::new(
                        value.span(),
                        "Only unitless integers are allowed as a power",
                    ));
                }
            };

            if !s.is_empty() {
                Err(Error::new(
                    num.span(),
                    "Only unitless integers are allowed as a power",
                ))
            } else {
                Ok(PowerExpr::Expr {
                    atom: Atom::Item { item },
                    num,
                })
            }
        } else {
            Ok(PowerExpr::Atom {
                atom: Atom::Item { item },
            })
        }
    }
}

impl Parse for MulExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let p_expr = input.parse::<PowerExpr>()?;

        if input.peek(Token![*]) {
            let _ = input.parse::<Token![*]>()?;

            let rhs = input.parse::<PowerExpr>()?;
            Ok(MulExpr::Mul { lhs: p_expr, rhs })
        } else if input.peek(Token![/]) {
            let _ = input.parse::<Token![/]>()?;

            let rhs = input.parse::<PowerExpr>()?;
            Ok(MulExpr::Div { lhs: p_expr, rhs })
        } else {
            Ok(Self::Pow { p_expr })
        }
    }
}

impl Parse for Atom {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if let Ok(item) = input.parse::<Item>() {
            Ok(Self::Item { item })
        } else {
            let content;
            parenthesized!(content in fork);

            Ok(Self::MulExpr {
                expr: Box::new(content.parse::<MulExpr>()?),
            })
        }
    }
}
