use gfahandlegraph::{
    gfa::orientation::Orientation,
    handle::{Edge, Handle, NodeId},
    hashgraph::HashGraph,
    mutablehandlegraph::*,
    parser::*,
    pathgraph::PathHandleGraph,
};

fn read_small_gfa2() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn read_small_gfa1() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/gfa1_files/lil.gfa") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn read_medium_gfa2() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/big_files/test.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn read_medium_gfa1() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/big_files/test.gfa") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn read_big_gfa2() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn read_big_gfa1() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

#[test]
fn create_medium_graph() {
    // about 2 seconds total
    println!("Parse and Create graph from medium file [GFA]");
    let _g = read_medium_gfa1();
    /* nodes: 4058     edges: 9498     paths: 7
    let nodes = _g.all_handles().count();
    let edges = _g.all_edges().count();
    let paths = _g.paths_iter().count();
    println!("nodes: {}\tedges: {}\tpaths: {}", nodes, edges, paths);
    */
    println!("Graph complete!");

    println!("Parse and Create graph from medium file [GFA2]");
    let _g = read_medium_gfa2();
    /* nodes: 4058     edges: 9498     paths: 7
    let nodes = _g.all_handles().count();
    let edges = _g.all_edges().count();
    let paths = _g.paths_iter().count();
    println!("nodes: {}\tedges: {}\tpaths: {}", nodes, edges, paths);
    */
    println!("Graph complete!");
}

#[test]
#[ignore]
fn create_big_graph() {
    // actual configuration (7.31.58) using GFAObject
    // less than 8 minutes (7.33.28) using GFAObject
    // less than 8 minutes (7.39.23) not using GFAObject
    // The performance are very similar,
    // so I don't think not using an intermediate ogject could
    // improve the performance significantly
    println!("Parse and Create graph from big file [GFA]");
    let _g = read_big_gfa1();
    /* nodes: 715018   edges: 985445   paths: 0
    let nodes = _g.all_handles().count();
    let edges = _g.all_edges().count();
    let paths = _g.paths_iter().count();
    println!("nodes: {}\tedges: {}\tpaths: {}", nodes, edges, paths);
    */
    println!("Graph complete!");

    // actual configuration (8.31.58) using GFAObject
    // graph from gfa2 are generally 1 minute slower than from gfa
    println!("Parse and Create graph from big file [GFA2]");
    let _g = read_big_gfa2();
    /* nodes: 715018   edges: 985445   paths: 0
    let nodes = _g.all_handles().count();
    let edges = _g.all_edges().count();
    let paths = _g.paths_iter().count();
    println!("nodes: {}\tedges: {}\tpaths: {}", nodes, edges, paths);
    */
    println!("Graph complete!");
}

#[test]
fn add_node() {
    let mut graph = read_small_gfa2();
    let node: NodeId = 42.into();
    let sequence = b"TEST_SEQUENCE";

    match graph.create_handle(sequence, node) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}

#[test]
fn add_edge() {
    let mut graph = read_small_gfa2();

    let node: NodeId = 42.into();
    let sequence = b"TEST_SEQUENCE";
    match graph.create_handle(sequence, node) {
        Ok(_) => {
            let left: Handle = Handle::new(42 as u64, Orientation::Backward);
            let right: Handle = Handle::new(13 as u64, Orientation::Forward);
            let edge: Edge = Edge(left, right);
            match graph.create_edge(edge) {
                Ok(_) => graph.print_graph(),
                Err(why) => println!("Error {}", why),
            }
        }
        Err(why) => println!("Error {}", why),
    }
}

#[test]
fn add_path() {
    use bstr::ByteSlice;

    let mut graph = read_small_gfa2();

    let node: NodeId = 42.into();
    let sequence = b"TEST_SEQUENCE";
    match graph.create_handle(sequence, node) {
        Ok(_) => {
            let left: Handle = Handle::new(42 as u64, Orientation::Backward);
            let right: Handle = Handle::new(13 as u64, Orientation::Forward);
            let edge: Edge = Edge(left, right);
            match graph.create_edge(edge) {
                Ok(_) => {
                    let path = graph.create_path_handle(b"test_path", false);
                    let seq_ids = vec![b"11+", b"13-", b"42+"];
                    for seq in seq_ids.iter() {
                        let last = seq.len() - 1;
                        let seq_id = seq[..last].to_str().unwrap();

                        let sgn: &str = &seq[last..].to_str().unwrap();
                        let orient: Orientation = match sgn {
                            "+" => Orientation::Forward,
                            "-" => Orientation::Backward,
                            _ => panic!("AAAAAAAA"),
                        };

                        let handle = Handle::new(seq_id.parse::<u64>().unwrap(), orient);
                        graph.append_step(&path, handle);
                    }
                    graph.print_graph()
                }
                Err(why) => println!("Error {}", why),
            }
        }
        Err(why) => println!("Error {}", why),
    }
}

#[test]
fn remove_node() {
    let mut graph = read_small_gfa2();
    let node: NodeId = 12.into();

    match graph.remove_handle(node) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}

#[test]
fn remove_edge() {
    let mut graph = read_small_gfa2();

    let left: Handle = Handle::new(11 as u64, Orientation::Forward);
    let right: Handle = Handle::new(13 as u64, Orientation::Forward);
    let edge: Edge = Edge(left, right);
    match graph.remove_edge(edge) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}

#[test]
fn remove_path() {
    let mut graph = read_small_gfa2();

    let path = b"14";
    match graph.remove_path(path) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}

#[test]
fn modify_node() {
    let mut graph = read_small_gfa2();
    let node: NodeId = 12.into();
    let sequence = b"MODIFIED_SEQUENCE";

    match graph.modify_handle(node, sequence) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}

#[test]
fn modify_edge() {
    let mut graph = read_small_gfa2();

    let left: Handle = Handle::new(11 as u64, Orientation::Forward);
    let right: Handle = Handle::new(13 as u64, Orientation::Forward);
    let edge: Edge = Edge(left, right);
    let new_left: Handle = Handle::new(11 as u64, Orientation::Forward);
    let new_right: Handle = Handle::new(11 as u64, Orientation::Forward);
    match graph.modify_edge(edge, Some(new_left), Some(new_right)) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}

#[test]
fn modify_path() {
    let mut graph = read_small_gfa2();

    let path = b"14";
    let path_sequence = vec![
        Handle::new(11 as u64, Orientation::Forward),
        Handle::new(11 as u64, Orientation::Forward),
        Handle::new(11 as u64, Orientation::Forward),
    ];
    match graph.modify_path(path, path_sequence) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}
