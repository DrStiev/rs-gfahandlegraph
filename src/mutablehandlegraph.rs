use crate::handle::{Edge, Handle, NodeId};
use crate::handlegraph::{error::GraphError, HandleGraph, HandleGraphRef};

pub trait SubtractiveHandleGraph {
    /// Function that remove a
    /// [`Node`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/hashgraph/node/struct.Node.html)
    /// and all its occurrencies
    /// # Example
    /// ```ignore
    /// // Nodes: 11, 12, 13
    /// // Edges: 11 -> 12, 11 -> 13, 12 -> 13
    ///
    /// let remove_id: NodeId = 12.into();
    /// graph.remove_handle(remove_id);
    ///
    /// // Nodes: 11, 13
    /// // Edges: 11 -> 13
    /// ```
    fn remove_handle<T: Into<NodeId>>(
        &mut self,
        node: T,
    ) -> Result<bool, GraphError>;

    /// Function that removes an
    /// [`Edge`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/handle/struct.Edge.html)
    /// between 2 nodes
    /// # Example
    /// ```ignore
    /// // Nodes: 11, 12, 13
    /// // Edges: 11 -> 12, 11 -> 13, 12 -> 13
    ///
    /// let h1: NodeId = 11.into();
    /// let h3: NodeId = 13.into();
    /// graph.remove_edge(Edge(h1, h3));
    ///
    /// // Nodes: 11, 12, 13
    /// // Edges: 11 -> 12, 12 -> 13
    /// ```
    fn remove_edge(&mut self, edge: Edge) -> Result<bool, GraphError>;

    /// Function that clears a Graph
    /// and set max_id to 0 and min_id to u64::MAX
    /// like the Default implementation for
    /// [`HashGraph`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/hashgraph/graph/struct.HashGraph.html)
    fn clear_graph(&mut self);
}

pub trait AdditiveHandleGraph {
    fn append_handle(&mut self, seq: &[u8]) -> Result<Handle, GraphError>;

    fn create_handle<T: Into<NodeId>>(
        &mut self,
        node_id: T,
        seq: &[u8],
    ) -> Result<Handle, GraphError>;

    fn create_edge(&mut self, edge: Edge) -> Result<bool, GraphError>;
}

pub trait ModdableHandleGraph {
    /// This function will replace the sequence associated to the specified
    /// [`NodeId`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/handle/struct.NodeId.html)
    /// # Example
    /// ```ignore
    /// if graph.modify_handle(14 as u64, b"TEST_SEQUENCE"){
    ///     println!("Graph AFTER modify Node");
    ///     graph.print_graph();
    /// } else {
    ///     println!("Failed to modify Node");
    /// }
    /// ```
    fn modify_handle<T: Into<NodeId>>(
        &mut self,
        node_id: T,
        seq: &[u8],
    ) -> Result<bool, GraphError>;

    /// given an [`Edge`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/handle/struct.Edge.html),
    /// this function will replace the left, the right or both
    /// [`Handle`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/handle/struct.Handle.html)
    /// with the provided ones
    /// # Example
    /// ```ignore
    /// let h1 = graph.create_handle(b"1", 1);
    /// let h3 = graph.create_handle(b"3", 3);
    ///
    /// if graph.modify_edge(Edge(h1, h3), Some(h1), Some(h5)){
    ///     println!("Graph AFTER modify: {:?}", Edge(h1, h3));
    ///     graph.print_graph();
    /// } else {
    ///     println!("Failed to modify {:?}", Edge(h1, h3));
    /// }
    /// ```
    fn modify_edge(
        &mut self,
        old_edge: Edge,
        left_node: Option<Handle>,
        right_node: Option<Handle>,
    ) -> Result<bool, GraphError>;
}

/// Trait encapsulating the mutable aspects of a handlegraph
/// WIP
pub trait MutableHandleGraph: HandleGraph {
    fn divide_handle(
        &mut self,
        handle: Handle,
        offsets: Vec<usize>,
    ) -> Vec<Handle>;

    fn split_handle(
        &mut self,
        handle: Handle,
        offset: usize,
    ) -> (Handle, Handle) {
        let handles = self.divide_handle(handle, vec![offset]);
        (handles[0], handles[1])
    }

    fn apply_orientation(&mut self, handle: Handle) -> Handle;
}

pub trait MutHandleGraphRef: HandleGraphRef {}

impl<'a, T> MutHandleGraphRef for &'a T
where
    T: HandleGraph,
    &'a T: HandleGraphRef,
{
}
