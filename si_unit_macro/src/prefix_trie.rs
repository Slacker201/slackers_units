use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PrefixTrie {
    root: PrefixTrieNode,
}

#[derive(Debug)]
struct PrefixTrieNode {
    items: HashMap<u8, PrefixTrieNode>,
    is_terminal: bool,
}

impl PrefixTrie {
    pub fn new() -> Self {
        Self {
            root: PrefixTrieNode::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        let slice = word.as_bytes();
        self.root.add_word(slice);
    }

    pub fn to_const_map(&self) -> TokenStream {
        self.root.to_const_map()
    }
}

impl PrefixTrieNode {
    fn new() -> Self {
        PrefixTrieNode {
            items: HashMap::new(),
            is_terminal: false,
        }
    }
    pub fn add_word(&mut self, s: &[u8]) {
        let latest_byte = if let Some(latest_byte) = s.first() {
            latest_byte
        } else {
            return;
        };
        let item = self
            .items
            .entry(*latest_byte)
            .or_insert_with(PrefixTrieNode::new);
        let s = &s[1..];
        if s.is_empty() {
            self.is_terminal = true;
        } else {
            item.add_word(s);
        }
    }

    pub fn to_const_map(&self) -> TokenStream {
        let mut q = phf_codegen::Map::new();

        for (key, value) in self.items.iter().map(|(k, v)| {
            if v.items.is_empty() {
                (*k, "::slackers_si_units::ConstTrie::Leaf".to_string())
            } else {
                let v_items = v.to_const_map();
                let is_terminal = v.is_terminal;
                (
                    *k,
                    format!(
                        "::slackers_si_units::ConstTrie::Node {{
                            is_terminal: {is_terminal},
                            node: {v_items},
                }},"
                    ),
                )
            }
        }) {
            q.entry(key, value);
        }

        q.phf_path("::slackers_si_units");

        q.build().into_token_stream()
    }
}
