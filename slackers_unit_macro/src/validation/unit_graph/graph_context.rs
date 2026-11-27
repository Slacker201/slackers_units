use std::collections::HashMap;

use crate::data::Id;

pub(crate) struct GraphContext {
    pub base_to_vector_mapping: HashMap<Id, Box<[i32]>>,
}
