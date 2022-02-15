/// Overview: Node model.
use std::collections::HashMap;

pub struct Node {
    pub rule_index: Option<usize>,
    // pub children_map_index: usize,
    pub children_map: HashMap<char, usize>,
}
