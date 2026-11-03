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
    is_terminal: bool,
}

#[derive(Debug)]
enum TrieNode {
    Leaf,
    Node(PrefixTrieNode),
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
    #[allow(unused)]
    pub fn find_longest<'a>(&self, s: &'a [u8]) -> Option<(Vec<u8>, &'a [u8])> {
        self.root.find_longest(s)
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
        let Some(latest_byte) = s.first() else {
            return;
        };
        let item = self.items.entry(*latest_byte).or_insert(TrieNode::Leaf);
        let s = &s[1..];
        if s.is_empty() {
            self.is_terminal = true;
        } else {
            let mut trie = PrefixTrieNode::new();
            trie.add_word(s);
            *item = TrieNode::Node(trie);
        }
    }

    pub fn to_const_map(&self) -> TokenStream {
        let mut q = phf_codegen::Map::new();

        for (key, value) in self.items.iter().map(|(k, v)| match v {
            TrieNode::Leaf => (*k, "::slackers_si_units::ConstTrie::Leaf".to_string()),
            TrieNode::Node(prefix_trie_node) => {
                let v_items = prefix_trie_node.to_const_map();
                let is_terminal = prefix_trie_node.is_terminal;
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

    fn find_longest<'a>(&self, bytes: &'a [u8]) -> Option<(Vec<u8>, &'a [u8])> {
        let first_byte = bytes.first()?;

        match self.items.get(first_byte)? {
            TrieNode::Leaf => Some((vec![*first_byte], &bytes[1..])),
            TrieNode::Node(node) => match node.find_longest(&bytes[1..]) {
                Some((mut found_bytes, remainder)) => {
                    found_bytes.push(*first_byte);
                    Some((found_bytes, remainder))
                }
                None => {
                    if self.is_terminal {
                        Some((vec![*first_byte], &bytes[1..]))
                    } else {
                        None
                    }
                }
            },
        }
    }
}
