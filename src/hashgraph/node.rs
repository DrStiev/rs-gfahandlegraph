use bstr::BString;
use fnv::FnvHashMap;

use crate::handle::Handle;

use super::PathId;

/// New type
/// # Example
/// ```ignore
/// pub struct Node {
///     pub sequence: BString,
///     pub left_edges: Vec<Handle>,
///     pub right_edges: Vec<Handle>,
///     pub occurrences: FnvHashMap<PathId, usize>,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Node {
    pub sequence: BString,
    // use hashmap instead of vectors
    pub left_edges: Vec<Handle>,
    pub right_edges: Vec<Handle>,
    pub occurrences: FnvHashMap<PathId, usize>,
}

impl Node {
    pub fn new(sequence: &[u8]) -> Node {
        Node {
            sequence: sequence.into(),
            left_edges: vec![],
            right_edges: vec![],
            occurrences: FnvHashMap::default(),
        }
    }
}
