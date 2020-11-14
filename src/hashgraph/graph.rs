use fnv::FnvHashMap;

use crate::gfa::{gfa1::GFA, gfa2::GFA2, orientation::Orientation};

use crate::{
    handle::{Edge as GraphEdge, Handle, NodeId},
    handlegraph::*,
    mutablehandlegraph::*,
    pathgraph::PathHandleGraph,
};

use super::{Node, Path, PathId};

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
    GFA(GFA<usize>),
    GFA2(GFA2<usize>),
}

impl HashGraph {
    pub fn new() -> HashGraph {
        Default::default()
    }

    fn add_segment<T: Into<NodeId>>(
        &mut self,
        node: T,
        sequence: &[u8],
    ) -> Result<bool, GraphError> {
        let node = node.into();
        match self.create_handle(sequence, node) {
            Ok(_) => Ok(true),
            Err(why) => Err(why),
        }
    }

    fn add_edge<T: Into<NodeId>>(
        &mut self,
        left: T,
        left_orient: Orientation,
        right: T,
        right_orient: Orientation,
    ) -> Result<bool, GraphError> {
        let left_handle = Handle::new(left, left_orient);
        let right_handle = Handle::new(right, right_orient);

        match self.create_edge(GraphEdge(left_handle, right_handle)) {
            Ok(_) => Ok(true),
            Err(why) => Err(why),
        }
    }

    fn add_path(
        &mut self,
        path_id: &[u8],
        sequence_id: impl Iterator<Item = (usize, Orientation)>,
    ) -> Result<bool, GraphError> {
        let path_id = self.create_path_handle(&path_id, false);
        for (id, orient) in sequence_id {
            self.append_step(&path_id, Handle::new(id as u64, orient));
        }
        Ok(true)
    }

    /// Build an HashGraph from a GFA(2)<usize> Object\
    /// the function will iterate only over the segments, edges and ogroups fields
    /// # Examples
    /// ```ignore
    /// let graph = HashGraph::new();
    /// let mut file: GFA2<usize> = GFA2::new();
    /// match graph.create_graph(FileType::GFA2(file)) {
    ///     Ok(g) => Ok(g),
    ///     Err(why) => println!("{}", why),
    /// }
    /// ```
    #[inline]
    pub fn create_graph(&self, file: FileType) -> Result<HashGraph, GraphError> {
        let mut new_self = self.clone();
        match file {
            FileType::GFA(x) => {
                //HashGraph::insert_gfa1_line(new_self, &x)
                x.segments
                    .iter()
                    .for_each(|s| match /*new_self.add_segment_from_gfa(s)*/ new_self.add_segment(s.name, &s.sequence) {
                        Ok(_) => (),
                        Err(why) => println!("Error {}", why),
                    });
                x.links
                    .iter()
                    .for_each(|l| match /*new_self.add_link_from_gfa(l)*/ new_self.add_edge(l.from_segment, l.from_orient, l.to_segment, l.to_orient) {
                        Ok(_) => (),
                        Err(why) => println!("Error {}", why),
                    });
                x.paths
                    .iter()
                    .for_each(|p| match /*new_self.add_path_from_gfa(p)*/ new_self.add_path(&p.path_name, p.iter()) {
                        Ok(_) => (),
                        Err(why) => println!("Error {}", why),
                    });
                Ok(new_self)
            }
            FileType::GFA2(x) => {
                //HashGraph::insert_gfa2_line(new_self, &x)
                x.segments
                    .iter()
                    .for_each(|s| match /*new_self.add_segment2_from_gfa(s)*/ new_self.add_segment(s.id, &s.sequence) {
                        Ok(_) => (),
                        Err(why) => println!("Error {}", why),
                    });
                x.edges
                    .iter()
                    .for_each(|e| /*new_self.add_edge_from_gfa(e)*/ {
                        let len = e.sid1.to_string().len() - 1;
                        let l = e.sid1.to_string()[..len].parse::<u64>().unwrap();
                        let l_orient = match &e.sid1.to_string()[len..] {
                            "0" => Orientation::Forward,
                            "1" => Orientation::Backward,
                            _ => panic!("Error! Edge did not include orientation"),
                        };

                        let len = e.sid2.to_string().len() - 1;
                        let r = e.sid2.to_string()[..len].parse::<u64>().unwrap();
                        let r_orient = match &e.sid2.to_string()[len..] {
                            "0" => Orientation::Forward,
                            "1" => Orientation::Backward,
                            _ => panic!("Error! Edge did not include orientation"),
                        };

                        match new_self.add_edge(l, l_orient, r, r_orient) {
                            Ok(_) => (),
                            Err(why) => println!("Error {}", why),
                        }
                    });
                x.groups_o
                    .iter()
                    .for_each(|o| match /*new_self.add_ogroup_from_gfa(o)*/ new_self.add_path(&o.id, o.iter()) {
                        Ok(_) => (),
                        Err(why) => println!("Error {}", why),
                    });
                Ok(new_self)
            }
        }
    }

    /// Print an HashGraph object in a simplified way
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
    ///         15: ACCTT -> CCTTGA -(TCAAGG) -> CTTGATT
    ///     }
    /// }
    /// */
    /// ```
    pub fn print_graph(&self) {
        println!("Graph: {{");
        // print all the segments
        self.print_segments();
        // print all the edges
        self.print_edges();
        // print all the paths
        self.print_paths();
        println!("}}");
    }

    /// Function that prints all the segments in a graph
    fn print_segments(&self) {
        use bstr::BString;

        println!("\tNodes: {{");
        // get all the nodeid and sequence associated with them
        for handle in self.all_handles() {
            let node_id: String = handle.id().to_string();
            let sequence: BString = self.sequence_iter(handle.forward()).collect();

            println!("\t\t{}: {}", node_id, sequence);
        }
        println!("\t}}");
    }

    /// Function that prints all the edges in a graph
    fn print_edges(&self) {
        println!("\tEdges: {{");
        // get all the link (edge) between nodes
        for edge in self.all_edges() {
            let GraphEdge(left, right) = edge;

            let from_node: String = if !left.id().to_string().is_empty() {
                left.id().to_string()
            } else {
                "NUL".to_string()
            };
            let to_node: String = if !right.id().to_string().is_empty() {
                right.id().to_string()
            } else {
                "NUL".to_string()
            };

            let mut left_orient: String = "".to_string();
            if from_node != "NUL" {
                if left.is_reverse() {
                    left_orient = "-".to_string();
                } else {
                    left_orient = "+".to_string();
                }
            }

            let mut right_orient: String = "".to_string();
            if to_node != "NUL" {
                if right.is_reverse() {
                    right_orient = "-".to_string();
                } else {
                    right_orient = "+".to_string();
                }
            }

            println!(
                "\t\t{}{} -- {}{}",
                from_node, left_orient, to_node, right_orient
            );
        }
        println!("\t}}");
    }

    /// Function that prints all the paths in a graph
    fn print_paths(&self) {
        use bio::alphabets::dna;
        use bstr::BString;

        println!("\tPaths: {{");
        // get all the path
        for path_id in self.paths_iter() {
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
                    let rev_sequence: BString = dna::revcomp(node.sequence.as_slice()).into();
                    print!("{} -({})", rev_sequence, node.sequence);
                } else {
                    print!("{}", node.sequence);
                }
            }
            println!();
        }
        println!("\t}}");
    }

    pub fn print_occurrences(&self) {
        self.all_handles().for_each(|h| {
            let node = self.get_node(&h.id()).unwrap();
            println!("{} - {:?}", node.sequence, node.occurrences);
        });
    }

    /// Function that returns a reference to the value corresponding to the key.\
    /// The reference is a Node object wrapped in Option
    /// # Examples
    /// ```ignore
    /// println!("{:?}", graph.get_node(&11));
    /// // Some(Node { sequence: "ACCTT", left_edges: [], right_edges: [], occurrences: {} })
    /// ```
    pub fn get_node(&self, node_id: &NodeId) -> Option<&Node> {
        self.graph.get(node_id)
    }

    pub fn get_node_unchecked(&self, node_id: &NodeId) -> &Node {
        self.graph
            .get(node_id)
            .unwrap_or_else(|| panic!("Tried getting a node that doesn't exist, ID: {:?}", node_id))
    }

    pub fn get_node_mut(&mut self, node_id: &NodeId) -> Option<&mut Node> {
        self.graph.get_mut(node_id)
    }

    /// Function that returns a reference to the value corresponding to the key.\
    /// The reference is a Path object wrapped in Option
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
