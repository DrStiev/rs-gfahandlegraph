use fnv::FnvHashMap;

use crate::gfa::{gfa1::GFA, gfa2::GFA2, orientation::Orientation};
use crate::{
    handle::{Edge as GraphEdge, Handle, NodeId},
    handlegraph::*,
    mutablehandlegraph::*,
    pathgraph::PathHandleGraph,
};

use super::{Node, Path, PathId};
use rayon::prelude::*;

/// New type
/// # Example
/// ```ignore
/// pub struct HashGraph {
///     pub max_id: NodeId,
///     pub min_id: NodeId,
///     pub graph: FnvHashMap<NodeId, Node>,
///     pub path_id: FnvHashMap<Vec<u8>, i64>,
///     pub paths: FnvHashMap<i64, Path>,
/// }
/// ```
#[derive(Clone, Debug)]
pub struct HashGraph {
    pub max_id: NodeId,
    pub min_id: NodeId,
    pub graph: FnvHashMap<NodeId, Node>,
    pub path_id: FnvHashMap<Vec<u8>, i64>,
    pub paths: FnvHashMap<i64, Path>,
}

impl Default for HashGraph {
    fn default() -> HashGraph {
        HashGraph {
            max_id: NodeId::from(0),
            min_id: NodeId::from(std::u64::MAX),
            graph: Default::default(),
            path_id: Default::default(),
            paths: Default::default(),
        }
    }
}

pub enum FileType {
    GFA(GFA),
    GFA2(GFA2),
}

impl HashGraph {
    pub fn new() -> HashGraph {
        Default::default()
    }

    /// Build an HashGraph from a GFA Object\
    /// The function will iterate only over the segments, edges (links) and ogroups (paths) fields
    ///
    /// [enum]: https://doc.rust-lang.org/std/keyword.enum.html
    /// [gfa]: https://github.com/GFA-spec/GFA-spec/blob/master/GFA1.md
    /// [gfa2]: https://github.com/GFA-spec/GFA-spec/blob/master/GFA2.md
    ///
    /// ## Arguments
    /// * ```GFA Object``` wrapped in an [`enum`][enum] type to define the kind of format used
    /// * ```GFA()``` [`enum`][enum] wrapper to specify the [`GFA`][gfa] version 1
    /// * ```GFA2()``` [`enum`][enum] wrapper to specify the [`GFA`][gfa2] version 2
    ///
    /// ## Examples
    /// ```ignore
    /// let graph = HashGraph::new();
    /// let mut file: GFA2<usize> = GFA2::new();
    /// match graph.create_graph(FileType::GFA2(file)) {
    ///     Ok(g) => Ok(g),
    ///     Err(why) => println!("{}", why),
    /// }
    /// ```
    pub fn create_graph(
        &mut self,
        file: FileType,
    ) -> Result<HashGraph, GraphError> {
        match file {
            FileType::GFA(x) => {
                x.segments.into_iter().for_each(|s| {
                    match self.create_handle(s.name, &s.sequence) {
                        Ok(_) => (),
                        Err(why) => println!("Error {}", why),
                    }
                });
                x.links.into_iter().for_each(|l| {
                    let left = Handle::new(l.from_segment, l.from_orient);
                    let right = Handle::new(l.to_segment, l.to_orient);
                    match self.create_edge(GraphEdge(left, right)) {
                        Ok(_) => (),
                        Err(why) => println!("Error {}", why),
                    }
                });
                x.paths.into_iter().for_each(|p| {
                    let path_id = self.create_path_handle(&p.path_name, false);
                    for (id, orient) in p.iter() {
                        match self
                            .append_step(&path_id, Handle::new(id, orient))
                        {
                            Ok(_) => (),
                            Err(why) => println!("Error: {}", why),
                        };
                    }
                });
                Ok(self.to_owned())
            }
            FileType::GFA2(x) => {
                x.segments.into_iter().for_each(|s| {
                    match self.create_handle(s.id, &s.sequence) {
                        Ok(_) => (),
                        Err(why) => println!("Error {}", why),
                    }
                });
                x.edges.into_iter().for_each(|e| {
                    let orient = |rev: &str| match rev {
                        "43" => Orientation::Forward,
                        "45" => Orientation::Backward,
                        _ => panic!("Error retrieving the orientation"),
                    };

                    let sid1 = e.sid1.to_string();
                    let len = sid1.len() - 2;
                    let l = sid1[..len].parse::<u64>().unwrap();
                    let l_orient = orient(&sid1[len..]);

                    let sid2 = e.sid2.to_string();
                    let len = sid2.len() - 2;
                    let r = sid2[..len].parse::<u64>().unwrap();
                    let r_orient = orient(&sid2[len..]);

                    let left = Handle::new(l, l_orient);
                    let right = Handle::new(r, r_orient);
                    match self.create_edge(GraphEdge(left, right)) {
                        Ok(_) => (),
                        Err(why) => println!("Error {}", why),
                    }
                });
                x.groups_o.into_iter().for_each(|o| {
                    let path_id = self.create_path_handle(&o.id, false);
                    for (id, orient) in o.iter() {
                        match self
                            .append_step(&path_id, Handle::new(id, orient))
                        {
                            Ok(_) => (),
                            Err(why) => println!("Error: {}", why),
                        };
                    }
                });
                Ok(self.to_owned())
            }
        }
    }

    /// Print an [`HashGraph`](struct.HashGraph.html) object in a simplified way
    /// # Example
    /// ```ignore
    /// graph.print_simple_graph();
    /// /*
    /// Graph: {
    ///     Nodes: {
    ///         13: CTTGATT
    ///         12: TCAAGG
    ///         11: ACCTT
    ///     }
    ///     Edges: {
    ///         12- -- 13+
    ///         11+ -- 12-
    ///         11+ -- 13+
    ///     }
    ///     Paths: {
    ///         14: ACCTT -> CTTGATT
    ///         15: ACCTT -> CCTTGA -> CTTGATT
    ///     }
    /// }
    /// */
    /// ```
    pub fn print_graph(&self) {
        println!("Graph: {{");
        // print all the segments
        println!("\tNodes: {{");
        self.print_segments();
        println!("\t}}");
        // print all the edges
        println!("\tEdges: {{");
        self.print_edges();
        println!("\t}}");
        // print all the paths
        println!("\tPaths: {{");
        self.print_paths();
        println!("\t}}");
        println!("}}");
    }

    /// Function that prints all the segments in a graph
    fn print_segments(&self) {
        use bstr::BString;
        // get all the nodeid and sequence associated with them
        self.handles_par().for_each(|handle| {
            let node_id: String = handle.id().to_string();
            let sequence: BString =
                self.sequence_iter(handle.forward()).collect();
            println!("\t\t{}: {}", node_id, sequence);
        });
    }

    /// Function that prints all the edges in a graph
    fn print_edges(&self) {
        // get all the link (edge) between nodes
        self.edges_par().for_each(|edge| {
            let GraphEdge(left, right) = edge;

            let orient = |rev: bool| {
                if rev {
                    "-".to_string()
                } else {
                    "+".to_string()
                }
            };

            let from_node: String = left.id().to_string();
            let to_node: String = right.id().to_string();

            let left_orient: String = orient(left.is_reverse());
            let right_orient: String = orient(right.is_reverse());

            println!(
                "\t\t{}{} -- {}{}",
                from_node, left_orient, to_node, right_orient
            );
        });
    }

    /// Function that prints all the paths in a graph
    fn print_paths(&self) {
        use crate::util::dna;
        use bstr::BString;

        // get all the path
        for path_id in self.paths() {
            let path = self.paths.get(&path_id).unwrap();
            //get the id or path name of a path
            let name = &path.name;
            let mut first: bool = true;

            for (ix, handle) in path.nodes.iter().enumerate() {
                let node = self.get_node(&handle.id()).unwrap();
                if first {
                    first = false;
                    print!("\t\t{}: ", name);
                }
                if ix != 0 {
                    print!(" -> ");
                }
                // print correct reverse and complement sequence to display the correct path
                if handle.is_reverse() {
                    let rev_sequence: BString =
                        dna::rev_comp(node.sequence.as_slice()).into();
                    print!("{}", rev_sequence);
                } else {
                    print!("{}", node.sequence);
                }
            }
            println!();
        }
    }

    pub fn print_occurrences(&self) {
        self.handles().for_each(|h| {
            let node = self.get_node(&h.id()).unwrap();
            println!("{} - {:?}", node.sequence, node.occurrences);
        });
    }

    /// Function that returns a reference to the value corresponding to the key.\
    /// The reference is a [`Node`](../node/struct.Node.html) object wrapped in Option
    /// # Examples
    /// ```ignore
    /// println!("{:?}", graph.get_node(&11));
    /// // Some(Node { sequence: "ACCTT", left_edges: [], right_edges: [], occurrences: {} })
    /// ```
    pub fn get_node(&self, node_id: &NodeId) -> Option<&Node> {
        self.graph.get(node_id)
    }

    pub fn get_node_unchecked(&self, node_id: &NodeId) -> &Node {
        self.graph.get(node_id).unwrap_or_else(|| {
            panic!("Tried getting a node that doesn't exist, ID: {:?}", node_id)
        })
    }

    pub fn get_node_mut(&mut self, node_id: &NodeId) -> Option<&mut Node> {
        self.graph.get_mut(node_id)
    }

    /// Function that returns a reference to the value corresponding to the key.\
    /// The reference is a [`Path`](../path/struct.Path.html) object wrapped in Option
    /// # Examples
    /// ```ignore
    /// println!("{:?}", graph.get_path(&0));
    /// // Some(Path { path_id: 0, name: "path-1", is_circular: false, nodes: [Handle(22), Handle(24)] })
    /// ```
    pub fn get_path(&self, path_id: &PathId) -> Option<&Path> {
        self.paths.get(path_id)
    }

    pub fn get_path_unchecked(&self, path_id: &PathId) -> &Path {
        self.paths
            .get(path_id)
            .unwrap_or_else(|| panic!("Tried to look up nonexistent path:"))
    }
}
