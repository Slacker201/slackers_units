use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PrefixTrie {
    root: PrefixTrieNode,
}

#[derive(Debug)]
struct PrefixTrieNode {
    items: HashMap<u8, TrieNode>,
}

#[derive(Debug)]
enum TrieNode {
    Leaf {
        t: Box<[u8]>,
    },
    Node {
        next: PrefixTrieNode,
        terminal: Option<Box<[u8]>>,
    },
}

impl PrefixTrie {
    pub fn new() -> Self {
        Self {
            root: PrefixTrieNode::new(),
        }
    }

    pub fn add_word(&mut self, word: &str, value: &str) {
        self.root.add_word(word.as_bytes(), value.as_bytes());
    }

    pub fn to_const_map(&self, ty: &str) -> TokenStream {
        self.root.to_const_map(ty)
    }

    pub fn find_longest<'a>(&'a self, bytes: &'a [u8]) -> Option<(&'a [u8], &'a [u8])> {
        self.root.find_longest(bytes)
    }
}

impl PrefixTrieNode {
    fn new() -> Self {
        PrefixTrieNode {
            items: HashMap::new(),
        }
    }
    pub fn add_word(&mut self, s: &[u8], value: &[u8]) {
        let Some(first_byte) = s.first() else {
            return;
        };

        if let Some(trie_node) = self.items.get_mut(first_byte) { match trie_node {
            TrieNode::Leaf { t } => {
                let mut trie = PrefixTrieNode::new();
                trie.add_word(&s[1..], value);
                let old_terminal = core::mem::take(t);
                *trie_node = TrieNode::Node {
                    next: trie,
                    terminal: Some(old_terminal),
                }
            }
            TrieNode::Node { next, terminal } => {
                let next_slice = &s[1..];
                if next_slice.is_empty() {
                    *terminal = Some(Box::from(value));
                } else {
                    next.add_word(next_slice, value);
                }
            }
        } } else {
            let trie = if s[1..].is_empty() {
                TrieNode::Leaf { t: value.into() }
            } else {
                let mut trie = PrefixTrieNode::new();
                trie.add_word(&s[1..], value);
                TrieNode::Node {
                    next: trie,
                    terminal: None,
                }
            };

            self.items.insert(*first_byte, trie);
        }
    }

    pub fn to_const_map(&self, ty: &str) -> TokenStream {
        let mut trie_builder = phf_codegen::Map::new();

        for (key, value) in &self.items {
            match value {
                TrieNode::Leaf { t } => {
                    let terminal = String::from_utf8_lossy(t);
                    trie_builder.entry(
                        *key,
                        format!("ConstTrie::<{ty}>::Leaf {{ value: {ty}::{terminal} }}"),
                    );
                }
                TrieNode::Node { next, terminal } => {
                    let term = if let Some(t) = terminal {
                        format!("Some({ty}::{})", String::from_utf8_lossy(t))
                    } else {
                        "None".to_string()
                    };

                    let nt = next.to_const_map(ty);

                    trie_builder.entry(
                        *key,
                        format!("ConstTrie::<{ty}>::Node {{ next: {nt}, terminal: {term} }}"),
                    );
                }
            }
        }

        trie_builder.phf_path("::slackers_units");

        trie_builder.build().into_token_stream()
    }

    pub(crate) fn find_longest<'a, 'b>(&'b self, bytes: &'a [u8]) -> Option<(&'b [u8], &'a [u8])> {
        let byte = bytes.first()?;

        if let Some(value) = self.items.get(byte) {
            match value {
                TrieNode::Leaf { t } => {
                    return Some((t, &bytes[1..]));
                }
                TrieNode::Node { next, terminal } => {
                    if let Some(next) = next.find_longest(&bytes[1..]) {
                        return Some(next);
                    }

                    if let Some(term) = terminal {
                        return Some((term, &bytes[1..]));
                    }
                }
            }
        }
        None
    }
}
