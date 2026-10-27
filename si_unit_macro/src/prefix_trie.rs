use std::collections::{HashMap, VecDeque};
use proc_macro2::TokenStream;
use quote::quote;


#[derive(Debug)]
pub struct PrefixTrie {
    root: PrefixTrieNode,
}

#[derive(Debug)]
struct PrefixTrieNode {
    items: HashMap<char, PrefixTrieNode>,
    is_terminal: bool,
}

impl PrefixTrie {
    pub fn new() -> Self {
        Self {
            root: PrefixTrieNode::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.root.add_word(word);
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
    pub fn add_word(&mut self, s: &str) {
        let latest_char = if let Some(latest_char) = s.chars().next() {
            latest_char
        } else {
            return;
        };
        let item = self
            .items
            .entry(latest_char)
            .or_insert_with(|| PrefixTrieNode::new());
        let s = &s[1..];
        if s.is_empty() {
            self.is_terminal = true;
        } else {
            item.add_word(s);
        }
    }

    pub fn to_const_map(&self) -> TokenStream {
        let ts: Vec<_> = self.items.iter().map(|(k, v)| {
            if v.items.is_empty() {
                quote! {
                    #k => ConstTrie::Leaf,
                }
            } else {
                let v_items = v.to_const_map();
                let is_terminal = v.is_terminal;
                quote! {
                    #k => ConstTrie::Trie {
                        is_terminal: #is_terminal,
                        map: #v_items,
                    },
                }
            }
        }).collect();


        quote! {
            phf::phf_map!(
                #(
                    #ts
                )*
            )
        }
    }
}

/*
TODO put in parser crate

trait FindLongest {
    fn find_longest(&self, s: &str, we_are_terminal: bool) -> Option<VecDeque<char>>;
}

impl FindLongest for Map<char, ConstTrie> {
    fn find_longest(&self, s: &str, we_are_terminal: bool) -> Option<VecDeque<char>> {
        let c= s.chars().next()?;

        if let Some(n) = self.get(&c) {
            match n {
                ConstTrie::Leaf => {
                    Some(VecDeque::from_iter([c]))
                },
                ConstTrie::Trie { is_terminal, map } => {
                    if let Some(mut s) = map.find_longest(&s[1..], *is_terminal) {
                        s.push_front(c);
                        Some(s)
                    } else {
                        if we_are_terminal {
                            Some(VecDeque::from_iter([c]))
                        } else {
                            None
                        }
                    }
                },
            }
        } else {
            Some(VecDeque::from_iter([c]))
        }
    }
}
*/