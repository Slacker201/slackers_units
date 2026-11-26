use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct PrefixTrie<T> {
    root: TrieNode<T>,
}

#[derive(PartialEq, Clone)]
pub struct TrieNode<T> {
    next: Option<HashMap<u8, TrieNode<T>>>,
    value: Option<T>,
}

impl<T> PrefixTrie<T> {
    pub fn add_item(&mut self, key: &[u8], item: T) {
        self.root.add_item(key, item);
    }
    pub fn find_longest<'a, 'b>(&'a self, key: &'b [u8]) -> Option<(&'a T, &'b [u8])> {
        self.root.find_longest(key)
    }
}

impl<T> TrieNode<T> {
    pub fn add_item(&mut self, key: &[u8], item: T) {
        let Some(next_byte) = key.first() else {
            if self.value.is_none() {
                self.value = Some(item);
            }
            return;
        };
        if let Some(map) = &mut self.next {
            let next_map = map.entry(*next_byte).or_default();
            next_map.add_item(&key[1..], item);
        } else {
            let mut map = HashMap::new();
            let next_map: &mut TrieNode<T> = map.entry(*next_byte).or_default();
            next_map.add_item(&key[1..], item);
            self.next = Some(map);
        }
    }
    pub fn find_longest<'a, 'b>(&'a self, key: &'b [u8]) -> Option<(&'a T, &'b [u8])> {
        let Some(byte) = key.first() else {
            return self.value.as_ref().map(|value| (value, key));
        };
        let Some(next) = self.next.as_ref() else {
            return self.value.as_ref().map(|v| (v, key));
        };
        match next.get(byte) {
            Some(value) => {
                if let Some(r) = value.find_longest(&key[1..]) {
                    Some(r)
                } else {
                    self.value.as_ref().map(|v| (v, key))
                }
            }
            None => self.value.as_ref().map(|v| (v, key)),
        }
    }
}

impl<T> Default for TrieNode<T> {
    fn default() -> Self {
        Self {
            next: None,
            value: None,
        }
    }
}

impl<T> Default for PrefixTrie<T> {
    fn default() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }
}

impl<T: Debug> Debug for TrieNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TrieNode")
            .field("next", &self.next)
            .field("value", &self.value)
            .finish()
    }
}

impl<T: Hash> Hash for TrieNode<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self.next {
            Some(value) => {
                let mut kv_refs: Box<[_]> = value.iter().collect();
                kv_refs.sort_by_key(|(byte, _)| **byte);
                kv_refs.hash(state);
            }
            None => None::<()>.hash(state),
        }
        self.value.hash(state);
    }
}
