pub use si_unit_macro::{__unit_impl__, system};
pub mod fastnum;
pub use phf::Map;
pub use typenum::Integer;
pub use typenum::consts::*;

pub enum ConstTrie {
    Leaf,
    Node {
        node: Map<u8, ConstTrie>,
        is_terminal: bool,
    },
}

pub trait FindFirst {
    fn find_longest<'a>(
        &self,
        bytes: &'a [u8],
        is_terminal: bool,
    ) -> Result<(Vec<u8>, &'a [u8]), ParseError>;
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum ParseError {
    NotPresent,
    UnknownUnit(Box<[u8]>),
    UnknownPrefix(Box<[u8]>),
    TrailingJunk(Box<[u8]>),
    PrefixedUnsupportedUnit(Box<[u8]>),
}

impl FindFirst for Map<u8, ConstTrie> {
    fn find_longest<'a>(
        &self,
        bytes: &'a [u8],
        is_terminal: bool,
    ) -> Result<(Vec<u8>, &'a [u8]), ParseError> {
        let first_byte = bytes.first().ok_or(ParseError::NotPresent)?;

        match self.get(first_byte).ok_or(ParseError::NotPresent)? {
            ConstTrie::Leaf => Ok((vec![*first_byte], &bytes[1..])),
            ConstTrie::Node {
                node,
                is_terminal: is_child_terminal,
            } => match node.find_longest(&bytes[1..], *is_child_terminal) {
                Ok((mut found_bytes, remainder)) => {
                    found_bytes.push(*first_byte);
                    Ok((found_bytes, remainder))
                }
                Err(e) => {
                    if is_terminal {
                        Ok((vec![*first_byte], &bytes[1..]))
                    } else {
                        Err(e)
                    }
                }
            },
        }
    }
}
