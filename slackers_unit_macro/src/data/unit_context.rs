use crate::data::{Id, unit_context::prefix_trie::PrefixTrie};

pub mod prefix_trie;

pub struct UnitContext {
    unit_trie: PrefixTrie<Id>,
    prefix_trie: PrefixTrie<Id>,
}

impl UnitContext {
    
}