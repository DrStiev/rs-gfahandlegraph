pub mod graph;
pub mod node;
pub mod path;

pub use self::graph::HashGraph;
pub use self::node::Node;
pub use self::path::{Path, PathId, PathStep};

use bstr::BString;
use rayon::iter::*;

use crate::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::*,
    mutablehandlegraph::*,
    pathgraph::PathHandleGraph,
    util::dna,
};

impl<'a> AllHandles for &'a HashGraph {
    type Handles = NodeIdRefHandles<
        'a,
        std::collections::hash_map::Keys<'a, NodeId, Node>,
    >;

    #[inline]
    fn handles(self) -> Self::Handles {
        let keys = self.graph.keys();
        NodeIdRefHandles::new(keys)
    }

    #[inline]
    fn node_count(self) -> usize {
        self.graph.len()
    }

    #[inline]
    fn has_node<I: Into<NodeId>>(self, n_id: I) -> bool {
        self.graph.contains_key(&n_id.into())
    }
}

/// parallel version for AllHandles Iter
impl<'a> AllHandlesPar for &'a HashGraph {
    type HandlesPar = rayon::iter::IterBridge<
        NodeIdRefHandles<
            'a,
            std::collections::hash_map::Keys<'a, NodeId, Node>,
        >,
    >;

    fn handles_par(self) -> Self::HandlesPar {
        self.handles().par_bridge()
    }
}

impl<'a> AllEdges for &'a HashGraph {
    type Edges = EdgesIter<&'a HashGraph>;

    #[inline]
    fn edges(self) -> Self::Edges {
        EdgesIter::new(self)
    }
}

/// parallel version for AllEdges Iter
impl<'a> AllEdgesPar for &'a HashGraph {
    type EdgesPar = rayon::iter::IterBridge<EdgesIter<&'a HashGraph>>;

    fn edges_par(self) -> Self::EdgesPar {
        self.edges().par_bridge()
    }
}

impl<'a> HandleNeighbors for &'a HashGraph {
    type Neighbors = NeighborIter<'a, std::slice::Iter<'a, Handle>>;

    #[inline]
    fn neighbors(self, handle: Handle, dir: Direction) -> Self::Neighbors {
        let node = self.get_node_unchecked(&handle.id());

        let handles = match (dir, handle.is_reverse()) {
            (Direction::Left, true) => &node.right_edges,
            (Direction::Left, false) => &node.left_edges,
            (Direction::Right, true) => &node.left_edges,
            (Direction::Right, false) => &node.right_edges,
        };
        NeighborIter::new(handles.iter(), dir == Direction::Left)
    }

    #[inline]
    fn degree(self, handle: Handle, dir: Direction) -> usize {
        let n = self.get_node_unchecked(&handle.id());
        match dir {
            Direction::Right => n.right_edges.len(),
            Direction::Left => n.left_edges.len(),
        }
    }
}

/// parallel version for HandleNeighbors Iter
impl<'a> HandleNeighborsPar for &'a HashGraph {
    type NeighborsPar =
        rayon::iter::IterBridge<NeighborIter<'a, std::slice::Iter<'a, Handle>>>;

    fn neighbors_par(
        self,
        handle: Handle,
        dir: Direction,
    ) -> Self::NeighborsPar {
        self.neighbors(handle, dir).par_bridge()
    }
}

impl<'a> HandleSequences for &'a HashGraph {
    type Sequence = SequenceIter<std::iter::Copied<std::slice::Iter<'a, u8>>>;

    #[inline]
    fn sequence_iter(self, handle: Handle) -> Self::Sequence {
        let seq: &[u8] =
            &self.get_node_unchecked(&handle.id()).sequence.as_ref();
        SequenceIter::new(seq.iter().copied(), handle.is_reverse())
    }

    fn sequence(self, handle: Handle) -> Vec<u8> {
        let seq: &[u8] =
            &self.get_node_unchecked(&handle.id()).sequence.as_ref();
        if handle.is_reverse() {
            dna::rev_comp(seq)
        } else {
            seq.into()
        }
    }

    #[inline]
    fn node_len(self, handle: Handle) -> usize {
        self.get_node_unchecked(&handle.id()).sequence.len()
    }
}

/// parallel version for HandleSequence Iter
impl<'a> HandleSequencesPar for &'a HashGraph {
    type SequencePar = rayon::iter::IterBridge<
        SequenceIter<std::iter::Copied<std::slice::Iter<'a, u8>>>,
    >;

    fn sequence_par_iter(self, handle: Handle) -> Self::SequencePar {
        self.sequence_iter(handle).par_bridge()
    }
}

impl HandleGraph for HashGraph {
    #[inline]
    fn min_node_id(&self) -> NodeId {
        self.min_id
    }

    #[inline]
    fn max_node_id(&self) -> NodeId {
        self.max_id
    }
}

impl<'a> HandleGraphRef for &'a HashGraph {
    fn total_length(self) -> usize {
        self.graph.values().map(|n| n.sequence.len()).sum()
    }
}

impl ModdableHandleGraph for HashGraph {
    fn modify_handle<T: Into<NodeId>>(
        &mut self,
        node_id: T,
        seq: &[u8],
    ) -> Result<bool, GraphError> {
        let node_id: NodeId = node_id.into();
        let seq: BString = BString::from(seq);
        if let Some(n) = self.graph.get_mut(&node_id) {
            if n.sequence == seq {
                // no need to update
                Ok(true)
            } else {
                // update the sequence value of node
                *self.graph.get_mut(&node_id).unwrap().sequence = seq.to_vec();
                Ok(true)
            }
        } else {
            Err(GraphError::NodeNotExist(node_id.to_string()))
        }
    }

    fn modify_edge(
        &mut self,
        old_edge: Edge,
        left_node: Option<Handle>,
        right_node: Option<Handle>,
    ) -> Result<bool, GraphError> {
        let Edge(left, right) = old_edge;
        let left_node = left_node.unwrap_or(left);
        let right_node = right_node.unwrap_or(right);
        if old_edge == Edge(left_node, right_node) {
            // no need to update
            Ok(true)
        } else {
            // update Edge
            self.remove_edge(old_edge)?;
            self.create_edge(Edge(left_node, right_node))?;
            Ok(true)
        }
    }
}

impl SubtractiveHandleGraph for HashGraph {
    fn remove_handle<T: Into<NodeId>>(
        &mut self,
        node: T,
    ) -> Result<bool, GraphError> {
        let node_id: NodeId = node.into();
        if let Some(node) = self.graph.remove(&node_id) {
            // delete all the occurrencies in the edge list of node.id()
            let l = node.left_edges;
            let r = node.right_edges;
            l.iter().for_each(|i| {
                if let Some(left) = self.graph.get_mut(&i.id()) {
                    if i.is_reverse() {
                        if let Some(ll) = left
                            .left_edges
                            .par_iter()
                            .position_any(|x| x.id() == node_id)
                        {
                            left.left_edges.remove(ll);
                        }
                    } else if let Some(lr) = left
                        .right_edges
                        .par_iter()
                        .position_any(|x| x.id() == node_id)
                    {
                        left.right_edges.remove(lr);
                    }
                }
            });
            r.iter().for_each(|i| {
                if let Some(right) = self.graph.get_mut(&i.id()) {
                    if i.is_reverse() {
                        if let Some(rr) = right
                            .right_edges
                            .par_iter()
                            .position_any(|x| x.id() == node_id)
                        {
                            right.right_edges.remove(rr);
                        }
                    } else if let Some(rl) = right
                        .left_edges
                        .par_iter()
                        .position_any(|x| x.id() == node_id)
                    {
                        right.left_edges.remove(rl);
                    }
                }
            });
            self.clone().paths().for_each(|path| {
                let nodes = &self.paths.get_mut(&path).unwrap().nodes;
                if nodes.par_iter().any(|x| x.id() == node_id) {
                    self.paths.remove(&path);
                }
            });
            Ok(true)
        } else {
            Err(GraphError::NodeNotExist(node_id.to_string()))
        }
    }

    fn remove_edge(&mut self, Edge(l, r): Edge) -> Result<bool, GraphError> {
        // delete all the occurrencies of edge found in graph
        if self.has_edge(l, r) {
            if let Some(left) = self.graph.get_mut(&l.id()) {
                if l.is_reverse() {
                    if let Some(ll) = left
                        .left_edges
                        .par_iter()
                        .position_any(|x| x.id() == r.id())
                    {
                        left.left_edges.remove(ll);
                    }
                } else if let Some(lr) = left
                    .right_edges
                    .par_iter()
                    .position_any(|x| x.id() == r.id())
                {
                    left.right_edges.remove(lr);
                }
            }
            if let Some(right) = self.graph.get_mut(&r.id()) {
                if r.is_reverse() {
                    if let Some(rr) = right
                        .right_edges
                        .par_iter()
                        .position_any(|x| x.id() == l.id())
                    {
                        right.right_edges.remove(rr);
                    }
                } else if let Some(rl) = right
                    .left_edges
                    .par_iter()
                    .position_any(|x| x.id() == l.id())
                {
                    right.left_edges.remove(rl);
                }
            }
            for path in self.clone().paths() {
                let nodes = &self.paths.get_mut(&path).unwrap().nodes;
                if let Some(l) =
                    nodes.par_iter().position_any(|x| x.id() == l.id())
                {
                    if let Some(r) =
                        nodes.par_iter().position_any(|x| x.id() == r.id())
                    {
                        if r == l + 1 {
                            self.paths.remove(&path);
                        }
                    }
                }
            }
            Ok(true)
        } else {
            Err(GraphError::EdgeNotExist(
                l.id().to_string(),
                r.id().to_string(),
            ))
        }
    }

    fn clear_graph(&mut self) {
        self.max_id = NodeId::from(0);
        self.min_id = NodeId::from(std::u64::MAX);
        // clears the map, removing all key-value pairs
        self.graph.clear();
        self.path_id.clear();
        self.paths.clear();

        self.graph.shrink_to_fit();
        self.path_id.shrink_to_fit();
        self.paths.shrink_to_fit();
    }
}

impl AdditiveHandleGraph for HashGraph {
    fn append_handle(&mut self, sequence: &[u8]) -> Result<Handle, GraphError> {
        Ok(self.create_handle(self.max_id + 1, sequence))?
    }

    fn create_handle<T: Into<NodeId>>(
        &mut self,
        node_id: T,
        seq: &[u8],
    ) -> Result<Handle, GraphError> {
        let id: NodeId = node_id.into();

        if seq.is_empty() {
            return Err(GraphError::EmptySequence);
        }
        if self.get_node(&id).is_some() {
            Err(GraphError::IdAlreadyExist(id.to_string()))
        } else {
            self.graph.insert(id, Node::new(seq));
            self.max_id = std::cmp::max(self.max_id, id);
            self.min_id = std::cmp::min(self.min_id, id);
            Ok(Handle::pack(id, false))
        }
    }

    fn create_edge(
        &mut self,
        Edge(left, right): Edge,
    ) -> Result<bool, GraphError> {
        let add_edge = {
            let left_node = self
                .graph
                .get(&left.id())
                .expect("Node doesn't exist for the given handle");

            None == left_node.right_edges.iter().find(|&&h| h == right)
        };

        if add_edge {
            let left_node = self
                .graph
                .get_mut(&left.id())
                .expect("Node doesn't exist for the given handle");
            if left.is_reverse() {
                left_node.left_edges.push(right);
            } else {
                left_node.right_edges.push(right);
            }
            if left != right.flip() {
                let right_node = self
                    .graph
                    .get_mut(&right.id())
                    .expect("Node doesn't exist for the given handle");
                if right.is_reverse() {
                    right_node.right_edges.push(left.flip());
                } else {
                    right_node.left_edges.push(left.flip());
                }
            }
        }
        Ok(true)
    }
}

impl MutableHandleGraph for HashGraph {
    fn divide_handle(
        &mut self,
        handle: Handle,
        mut offsets: Vec<usize>,
    ) -> Vec<Handle> {
        let mut result = vec![handle];
        let node_len = self.node_len(handle);
        let sequence = self.sequence(handle);

        let fwd_handle = handle.forward();

        // Push the node length as a last offset to make constructing
        // the ranges nicer
        offsets.push(node_len);

        let fwd_offsets: Vec<usize> = if handle.is_reverse() {
            offsets.iter().map(|o| node_len - o).collect()
        } else {
            offsets
        };

        // staggered zip of the offsets with themselves to make the ranges
        let ranges: Vec<_> = fwd_offsets
            .iter()
            .zip(fwd_offsets.iter().skip(1))
            .map(|(&p, &n)| p..n)
            .collect();

        // TODO it should be possible to do this without creating new
        // strings and collecting into a vec

        let subseqs: Vec<BString> =
            ranges.into_iter().map(|r| sequence[r].into()).collect();

        for seq in subseqs {
            match self.append_handle(&seq) {
                Ok(h) => result.push(h),
                Err(why) => println!("Error: {}", why),
            };
        }

        // move the outgoing edges to the last new segment
        // empty the existing right edges of the original node
        let mut orig_rights = std::mem::take(
            &mut self.get_node_mut(&handle.id()).unwrap().right_edges,
        );

        let new_rights = &mut self
            .get_node_mut(&result.last().unwrap().id())
            .unwrap()
            .right_edges;
        // and swap with the new right edges
        std::mem::swap(&mut orig_rights, new_rights);

        // shrink the sequence of the starting handle
        let orig_node = &mut self.get_node_mut(&handle.id()).unwrap();
        orig_node.sequence = orig_node.sequence[0..fwd_offsets[0]].into();

        // update backwards references
        // first collect all the handles whose nodes we need to update
        let last_neighbors: Vec<_> = self
            .neighbors(*result.last().unwrap(), Direction::Right)
            .collect();

        // And perform the update
        for h in last_neighbors {
            let node = &mut self.get_node_mut(&h.id()).unwrap();
            let neighbors = if h.is_reverse() {
                &mut node.right_edges
            } else {
                &mut node.left_edges
            };

            for bwd in neighbors.iter_mut() {
                if *bwd == fwd_handle.flip() {
                    *bwd = result.last().unwrap().flip();
                }
            }
        }

        // create edges between the new segments
        for (this, next) in result.iter().zip(result.iter().skip(1)) {
            match self.create_edge(Edge(*this, *next)) {
                Ok(_) => (),
                Err(why) => println!("Error: {}", why),
            }
        }

        // update paths and path occurrences

        // TODO this is probably not correct, and it's silly to clone
        // the results all the time
        let affected_paths: Vec<(i64, usize)> = self
            .get_node_unchecked(&handle.id())
            .occurrences
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect();

        for (path_id, ix) in affected_paths.into_iter() {
            let step = PathStep::Step(path_id, ix);
            self.rewrite_segment(&step, &step, result.clone());
        }

        result
    }

    fn apply_orientation(&mut self, handle: Handle) -> Handle {
        if !handle.is_reverse() {
            return handle;
        }

        let node = self.get_node_mut(&handle.id()).unwrap();
        node.sequence = dna::rev_comp(node.sequence.as_slice()).into();

        let edges = {
            let node = self.get_node(&handle.id()).unwrap();
            node.left_edges
                .iter()
                .chain(node.right_edges.iter())
                .copied()
                .collect::<Vec<_>>()
        };

        for target in edges {
            let other = self.get_node_mut(&target.id()).unwrap();
            let backward_edges = if target.is_reverse() {
                other.right_edges.iter_mut()
            } else {
                other.left_edges.iter_mut()
            };

            for backward_handle in backward_edges {
                if backward_handle.id() == handle.id() {
                    *backward_handle = backward_handle.flip();
                    break;
                }
            }
        }

        let node = self.get_node_mut(&handle.id()).unwrap();
        std::mem::swap(&mut node.left_edges, &mut node.right_edges);

        let occurrences = &self.graph.get(&handle.id()).unwrap().occurrences;
        let paths = &mut self.paths;

        for (path_id, index) in occurrences.iter() {
            let path = paths.get_mut(&path_id).unwrap();
            let step = path.nodes.get_mut(*index).unwrap();
            *step = step.flip();
        }

        handle.flip()
    }
}

impl PathHandleGraph for HashGraph {
    type PathHandle = PathId;
    type StepHandle = PathStep;

    fn path_count(&self) -> usize {
        self.path_id.len()
    }

    fn has_path(&self, name: &[u8]) -> bool {
        self.path_id.contains_key(name)
    }

    fn name_to_path_handle(&self, name: &[u8]) -> Option<Self::PathHandle> {
        self.path_id.get(name).copied()
    }

    fn path_handle_to_name(&self, path_id: &Self::PathHandle) -> &[u8] {
        self.get_path_unchecked(path_id).name.as_slice()
    }

    fn is_circular(&self, path_id: &Self::PathHandle) -> bool {
        self.get_path_unchecked(path_id).is_circular
    }

    fn step_count(&self, path_id: &Self::PathHandle) -> usize {
        self.get_path_unchecked(path_id).nodes.len()
    }

    fn handle_of_step(&self, step: &Self::StepHandle) -> Option<Handle> {
        self.get_path_unchecked(&step.path_id())
            .lookup_step_handle(step)
    }

    fn path_handle_of_step(&self, step: &Self::StepHandle) -> Self::PathHandle {
        step.path_id()
    }

    fn path_begin(&self, path: &Self::PathHandle) -> Self::StepHandle {
        PathStep::Step(*path, 0)
    }

    fn path_end(&self, path: &Self::PathHandle) -> Self::StepHandle {
        PathStep::End(*path)
    }

    fn path_back(&self, path: &Self::PathHandle) -> Self::StepHandle {
        PathStep::Step(*path, self.step_count(path) - 1)
    }

    fn path_front_end(&self, path: &Self::PathHandle) -> Self::StepHandle {
        PathStep::Front(*path)
    }

    fn has_next_step(&self, step: &Self::StepHandle) -> bool {
        matches!(step, PathStep::End(_))
    }

    fn has_previous_step(&self, step: &Self::StepHandle) -> bool {
        matches!(step, PathStep::Front(_))
    }

    fn path_bases_len(&self, path_handle: &Self::PathHandle) -> Option<usize> {
        let path = self.paths.get(path_handle)?;
        Some(path.bases_len(&self.graph))
    }

    fn position_of_step(&self, step: &Self::StepHandle) -> Option<usize> {
        let path = self.paths.get(&step.path_id())?;
        path.position_of_step(&self.graph, step)
    }

    fn step_at_position(
        &self,
        path_handle: &Self::PathHandle,
        pos: usize,
    ) -> Option<Self::StepHandle> {
        let path = self.paths.get(path_handle)?;
        Some(path.step_at_position(&self.graph, pos))
    }

    fn destroy_path(&mut self, path: &Self::PathHandle) {
        if let Some(p) = self.paths.get(&path) {
            for handle in p.nodes.iter() {
                let node: &mut Node = self.graph.get_mut(&handle.id()).unwrap();
                node.occurrences.remove(path);
            }
            self.paths.remove(&path);
        }
    }

    fn next_step(&self, step: &Self::StepHandle) -> Self::StepHandle {
        match step {
            PathStep::Front(pid) => self.path_begin(pid),
            PathStep::End(pid) => self.path_end(pid),
            PathStep::Step(pid, ix) => {
                if *ix < self.step_count(pid) - 1 {
                    PathStep::Step(*pid, ix + 1)
                } else {
                    self.path_end(pid)
                }
            }
        }
    }

    fn previous_step(&self, step: &Self::StepHandle) -> Self::StepHandle {
        match step {
            PathStep::Front(pid) => self.path_front_end(pid),
            PathStep::End(pid) => self.path_back(pid),
            PathStep::Step(pid, ix) => {
                if *ix > 0 {
                    PathStep::Step(*pid, ix - 1)
                } else {
                    self.path_end(pid)
                }
            }
        }
    }

    fn create_path_handle(
        &mut self,
        name: &[u8],
        is_circular: bool,
    ) -> Self::PathHandle {
        let path_id = self.paths.len() as i64;
        let path = Path::new(name, path_id, is_circular);
        self.path_id.insert(name.into(), path_id);
        self.paths.insert(path_id, path);
        path_id
    }

    fn append_step(
        &mut self,
        path_id: &Self::PathHandle,
        to_append: Handle,
    ) -> Result<Self::StepHandle, GraphError> {
        let path: &mut Path = match self.paths.get_mut(path_id) {
            Some(path) => path,
            None => return Err(GraphError::PathNotExist(path_id.to_string())),
        };
        path.nodes.push(to_append);
        let step = (*path_id, path.nodes.len() - 1);
        let node: &mut Node = self.graph.get_mut(&to_append.id()).unwrap();
        node.occurrences.insert(step.0, step.1);
        Ok(PathStep::Step(*path_id, path.nodes.len() - 1))
    }

    fn prepend_step(
        &mut self,
        path_id: &Self::PathHandle,
        to_prepend: Handle,
    ) -> Self::StepHandle {
        let path: &mut Path = self.paths.get_mut(path_id).unwrap();
        // update occurrences in nodes already in the graph
        for h in path.nodes.iter() {
            let node: &mut Node = self.graph.get_mut(&h.id()).unwrap();
            *node.occurrences.get_mut(path_id).unwrap() += 1;
        }
        path.nodes.insert(0, to_prepend);
        let node: &mut Node = self.graph.get_mut(&to_prepend.id()).unwrap();
        node.occurrences.insert(*path_id, 0);
        PathStep::Step(*path_id, 0)
    }

    fn rewrite_segment(
        &mut self,
        begin: &Self::StepHandle,
        end: &Self::StepHandle,
        new_segment: Vec<Handle>,
    ) -> (Self::StepHandle, Self::StepHandle) {
        // extract the index range from the begin and end handles

        if begin.path_id() != end.path_id() {
            panic!("Tried to rewrite path segment between two different paths");
        }

        let path_id = begin.path_id();
        let path_len = self.paths.get(&path_id).unwrap().nodes.len();

        let step_index = |s: &Self::StepHandle| match s {
            PathStep::Front(_) => 0,
            PathStep::End(_) => path_len - 1,
            PathStep::Step(_, i) => *i,
        };

        let l = step_index(begin);
        let r = step_index(end);

        let range = l..=r;

        // first delete the occurrences of the nodes in the range
        for handle in self
            .paths
            .get(&path_id)
            .unwrap()
            .nodes
            .iter()
            .skip(l)
            .take(r - l + 1)
        {
            let node: &mut Node = self.graph.get_mut(&handle.id()).unwrap();
            node.occurrences.remove(&path_id);
        }

        // get a &mut to the path's vector of handles
        let handles: &mut Vec<Handle> =
            &mut self.paths.get_mut(&path_id).unwrap().nodes;

        let r = l + new_segment.len();
        // replace the range of the path's handle vector with the new segment
        handles.splice(range, new_segment);

        // update occurrences
        for (ix, handle) in
            self.paths.get(&path_id).unwrap().nodes.iter().enumerate()
        {
            let node: &mut Node = self.graph.get_mut(&handle.id()).unwrap();
            node.occurrences.insert(path_id, ix);
        }

        // return the new beginning and end step handles: even if the
        // input steps were Front and/or End, the output steps exist
        // on the path
        (PathStep::Step(path_id, l), PathStep::Step(path_id, r))
    }

    fn paths<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::PathHandle> + 'a> {
        Box::new(self.paths.keys())
    }

    fn occurrences<'a>(
        &'a self,
        handle: Handle,
    ) -> Box<dyn Iterator<Item = Self::StepHandle> + 'a> {
        let node: &Node = self.get_node_unchecked(&handle.id());
        Box::new(node.occurrences.iter().map(|(k, v)| PathStep::Step(*k, *v)))
    }

    fn steps<'a>(
        &'a self,
        path_handle: &'a Self::PathHandle,
    ) -> Box<dyn Iterator<Item = Self::StepHandle> + 'a> {
        let path = self.get_path_unchecked(path_handle);
        Box::new(
            path.nodes
                .iter()
                .enumerate()
                .map(move |(i, _)| PathStep::Step(*path_handle, i)),
        )
    }

    fn remove_step<T: Into<NodeId>>(
        &mut self,
        name: &[u8],
        node: T,
    ) -> Result<bool, GraphError> {
        use bstr::ByteSlice;

        if self.has_path(name) {
            let path_handle = self.name_to_path_handle(name).unwrap();
            let node = node.into();
            if let Some(p) = self.paths.get_mut(&path_handle) {
                p.nodes.retain(|x| x.id() != node);
            }
            Ok(true)
        } else {
            Err(GraphError::PathNotExist(name.to_str().unwrap().to_string()))
        }
    }

    fn modify_step<T: Into<NodeId>>(
        &mut self,
        name: &[u8],
        old_node: T,
        new_node: Handle,
    ) -> Result<bool, GraphError> {
        use bstr::ByteSlice;

        if self.has_path(name) {
            let path_handle = self.name_to_path_handle(name).unwrap();
            let old_node = old_node.into();
            if let Some(p) = self.paths.get_mut(&path_handle) {
                let path = p.nodes.clone();
                for (id, &handle) in path.iter().enumerate() {
                    if handle.id() == old_node {
                        p.nodes.remove(id);
                        p.nodes.insert(id, new_node);
                    }
                }
            }
            Ok(true)
        } else {
            Err(GraphError::PathNotExist(name.to_str().unwrap().to_string()))
        }
    }

    fn rewrite_path(
        &mut self,
        path_name: &[u8],
        sequence_of_id: Vec<Handle>,
    ) -> Result<bool, GraphError> {
        use bstr::ByteSlice;

        // update occurrencies in path
        if let Some(path_handle) = self.name_to_path_handle(path_name) {
            self.destroy_path(&path_handle);
            let len: usize = sequence_of_id.len();
            let mut x: usize = 0;
            let path = self.create_path_handle(path_name, false);
            while x < len {
                match self.append_step(&path, sequence_of_id[x]) {
                    Ok(_) => (),
                    Err(why) => return Err(why),
                };
                x += 1;
            }
            Ok(true)
        } else {
            Err(GraphError::PathNotExist(
                path_name.to_str().unwrap().to_string(),
            ))
        }
    }
}
