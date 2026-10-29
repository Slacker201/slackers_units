pub use si_unit_macro::system;
pub use phf::Map;


pub enum ConstTrie {
    Leaf,
    Node {
        node: Map<u8, ConstTrie>,
        is_terminal: bool,
    }
}