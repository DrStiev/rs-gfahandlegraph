use crate::handle::{Handle, NodeId};
use crate::handlegraph::error::*;

pub trait EmbeddedPaths {
    /// A handle to a path in the graph, can also be viewed as a path identifier
    type PathHandle;
    /// A handle to a specific step on a specific path in the graph
    type StepHandle;
}

// pub trait Paths {
// }

/// Trait encapsulating the immutable path-related aspects of a handlegraph
pub trait PathHandleGraph {
    // These associated types may be removed in the future if it turns
    // out it's better to use a single set of types for all
    // handlegraph implementations
    /// A handle to a path in the graph, can also be viewed as a path identifier
    type PathHandle;
    /// A handle to a specific step on a specific path in the graph
    type StepHandle;

    fn path_count(&self) -> usize;

    fn has_path(&self, name: &[u8]) -> bool;

    /// Paths have string names as well as handles
    fn name_to_path_handle(&self, name: &[u8]) -> Option<Self::PathHandle>;

    fn path_handle_to_name(&self, handle: &Self::PathHandle) -> &[u8];

    fn is_circular(&self, handle: &Self::PathHandle) -> bool;

    fn step_count(&self, handle: &Self::PathHandle) -> usize;

    /// Get the (node) handle that a step handle points to
    fn handle_of_step(&self, step_handle: &Self::StepHandle) -> Option<Handle>;

    fn path_handle_of_step(&self, step_handle: &Self::StepHandle) -> Self::PathHandle;

    /// Get the first step of the path
    fn path_begin(&self, path_handle: &Self::PathHandle) -> Self::StepHandle;

    /// Get the last step of the path
    fn path_end(&self, path_handle: &Self::PathHandle) -> Self::StepHandle;

    /// Get a step *beyond* the end of the path
    fn path_back(&self, path_handle: &Self::PathHandle) -> Self::StepHandle;

    /// Get a step *before* the end of the path
    fn path_front_end(&self, path_handle: &Self::PathHandle) -> Self::StepHandle;

    fn has_next_step(&self, step_handle: &Self::StepHandle) -> bool;

    fn has_previous_step(&self, step_handle: &Self::StepHandle) -> bool;

    fn path_bases_len(&self, path_handle: &Self::PathHandle) -> Option<usize>;

    fn position_of_step(&self, step_handle: &Self::StepHandle) -> Option<usize>;

    fn step_at_position(
        &self,
        path_handle: &Self::PathHandle,
        pos: usize,
    ) -> Option<Self::StepHandle>;

    fn destroy_path(&mut self, path: &Self::PathHandle);

    fn next_step(&self, step_handle: &Self::StepHandle) -> Self::StepHandle;

    fn previous_step(&self, step_handle: &Self::StepHandle) -> Self::StepHandle;

    fn create_path_handle(&mut self, name: &[u8], is_circular: bool) -> Self::PathHandle;

    fn append_step(
        &mut self,
        path: &Self::PathHandle,
        to_append: Handle,
    ) -> Result<Self::StepHandle, GraphError>;

    fn prepend_step(&mut self, path: &Self::PathHandle, to_prepend: Handle) -> Self::StepHandle;

    fn rewrite_segment(
        &mut self,
        begin: &Self::StepHandle,
        end: &Self::StepHandle,
        new_segment: Vec<Handle>,
    ) -> (Self::StepHandle, Self::StepHandle);

    /// Returns an iterator over all path identifiers in the graph
    fn paths<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::PathHandle> + 'a>;

    /// Returns an iterator over all the steps that
    /// cross through the given node handle, across all the paths in
    /// the graph
    fn occurrences<'a>(&'a self, handle: Handle)
        -> Box<dyn Iterator<Item = Self::StepHandle> + 'a>;

    /// Returns an iterator over all the steps in a path
    fn steps<'a>(
        &'a self,
        path: &'a Self::PathHandle,
    ) -> Box<dyn Iterator<Item = Self::StepHandle> + 'a>;

    /// Function that removes a
    /// [`Node`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/hashgraph/node/struct.Node.html)
    /// (and all it's occurrencies) from a
    /// [`Path`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/hashgraph/path/struct.Path.html)
    ///
    /// # Example
    /// ```ignore
    /// let path = b"14";
    /// let node = 11 as u64;
    ///
    /// match graph.remove_step(path, node) {
    ///     Ok(_) => graph.print_graph(),
    ///     Err(why) => println!("Error: {}", why),
    /// }
    /// ```
    fn remove_step<T: Into<NodeId>>(&mut self, name: &[u8], node: T) -> Result<bool, GraphError>;

    /// Function that modifies a
    /// [`Node`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/hashgraph/node/struct.Node.html)
    /// (and all it's occurrencies) from a
    /// [`Path`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/hashgraph/path/struct.Path.html)
    /// # Example
    /// ```ignore
    /// let path = b"14";
    /// let node = 11 as u64;
    /// let nodea = Handle::new(13 as u64, Orientation::Forward);
    ///
    /// match graph.modify_step(path, node, nodea) {
    ///     Ok(_) => graph.print_graph(),
    ///     Err(why) => println!("Error: {}", why),
    /// }
    /// ```
    fn modify_step<T: Into<NodeId>>(
        &mut self,
        name: &[u8],
        old_node: T,
        new_node: Handle,
    ) -> Result<bool, GraphError>;

    /// given a
    /// [`PathName`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/hashgraph/path/struct.Path.html),
    /// this function will replace the sequence of ids that compose the path
    /// # Example
    /// ```ignore
    /// let h1 = graph.create_handle(b"1", 1);
    /// let h3 = graph.create_handle(b"3", 3);
    ///
    /// if graph.rewrite_path(b"14", vec![h1, h3]){
    ///     println!("Graph AFTER modify path");
    ///     graph.print_graph();
    /// } else {
    ///     println!("Failed to modify path");
    /// }
    /// ```
    fn rewrite_path(
        &mut self,
        path_name: &[u8],
        sequence_of_id: Vec<Handle>,
    ) -> Result<bool, GraphError>;
}
