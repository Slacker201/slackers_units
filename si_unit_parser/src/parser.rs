use std::collections::HashMap;

use slk_tokenstream::TokenStream;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum UnitType {
    Second,
    Meter,
    Gram,
    Ampere,
    Kelvin,
    Mol,
    Candela,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Prefix {
    Milli,
    Centi,
    Deci,
    Null,
    Deca,
    Hecto,
    Kilo,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum CombSym {
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct TotalExpr {
    base: f64,

    unit_exprs: HashMap<UnitType, UnitExpr>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct UnitExpr {
    ty: UnitType,
    exponent: i64,
}

impl UnitExpr {
    pub fn parse(ts: &mut TokenStream<char>) -> Option<Self> {
        let cs = CombSym::parse(ts)?;

        let unit = UnitType::parse(ts)?;

        let mut exponent = parse_exponent(ts);

        if cs == CombSym::Div {
            exponent = -exponent;
        }

        Some(Self { ty: unit, exponent })
    }
}

impl CombSym {
    fn parse(ts: &mut TokenStream<char>) -> Option<Self> {
        ts.skip_while(|c| c.is_whitespace());
        let r = match *ts.peek()? {
            '*' => Self::Mul,
            '/' => Self::Div,
            _ => return None,
        };
        ts.skip();
        Some(r)
    }
}

impl UnitType {
    fn parse(ts: &mut TokenStream<char>) -> Option<Self> {
        ts.skip_while(|c| c.is_whitespace());
        let start = ts.mark();
        let r = match *ts.peek()? {
            's' => Self::Second,
            'm' => {
                ts.advance(1);
                match ts.peek_if(|c| *c == 'o') {
                    Some(_) => {
                        ts.advance(1);
                        let _ = ts.peek_if(|c| *c == 'l')?;

                        Self::Mol
                    }
                    None => {
                        ts.rewind();
                        Self::Meter
                    }
                }
            }
            'g' => Self::Gram,
            'A' => Self::Ampere,
            'K' => Self::Kelvin,
            'c' => {
                ts.skip();
                let _ = ts.expect(|c| *c == 'd', start)?;
                Self::Candela
            }

            _ => return None,
        };
        ts.skip();

        Some(r)
    }
}

fn parse_exponent(ts: &mut TokenStream<char>) -> i64 {
    ts.skip_while(|c| c.is_whitespace());
    match ts.consume_if(|c| *c == '^') {
        Some(_) => {
            let start = ts.mark();
            ts.consume_while(|c| c.is_numeric());
            let end = ts.mark();

            let chars: String = ts.slice_from_marks(start, end).iter().collect();
            
            chars.parse::<i64>().unwrap()
        }
        None => 0,
    }
}

impl TotalExpr {
    pub fn parse(s: impl Into<String>) -> Option<Self> {
        let backer: Box<[_]> = s.into().chars().collect();

        let mut ts = TokenStream::new(&backer);

        ts.skip_while(|c| c.is_whitespace());

        let start = ts.mark();
        ts.skip_while(|c| c.is_numeric());
        let end = ts.mark();

        let numbas: String = ts.slice_from_marks(start, end).iter().collect();

        let base = numbas.parse::<f64>().ok()?;
        let mut unit_exprs = HashMap::new();
        while let Some(expr) = UnitExpr::parse(&mut ts) {
            ts.skip_while(|c| c.is_whitespace());
            if unit_exprs.insert(expr.ty, expr).is_some() {
                return None;
            }
        }

        Some(Self { base, unit_exprs })
    }
}
