/// define a custom error type for the program
use std::{error, fmt};

pub type GraphResult<T> = Result<T, GraphError>;

#[derive(Debug)]
pub enum GraphError {
    IdAlreadyExist(String),
    EmptySequence,
    NodeNotExist(String),
    EdgeNotExist(String, String),
    EdgeAlreadyExist(String, String),
    PathNotExist(String),
    OrientationNotExists(String),
    PositionNotFound(String, String),
    Unknown,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphError as GE;
        match self {
            GE::IdAlreadyExist(id) => write!(f, "The Id provided ({}) already exists", id),
            GE::EmptySequence => write!(f, "Empty sequence"),
            GE::NodeNotExist(node) => write!(f, "Cannot find the node: {}", node),
            GE::EdgeNotExist(l, r) => write!(f, "The Edge ({} -> {}) did not exist", l, r),
            GE::EdgeAlreadyExist(l, r) => write!(f, "The Edge ({} -> {}) already exists", l, r),
            GE::PathNotExist(path) => write!(f, "The Path ({}) did not exist", path),
            GE::PositionNotFound(pos_list, lr) => {
                write!(f, "Not found node {} in {} list", pos_list, lr)
            }
            GE::OrientationNotExists(orientation) => write!(
                f,
                "Segment reference Id ({}) did not include orientation",
                orientation
            ),
            GE::Unknown => write!(f, "Unknown error while operating on the graph"),
        }
    }
}

impl error::Error for GraphError {}
