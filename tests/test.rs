use gfahandlegraph::{
    gfa::orientation::Orientation,
    handle::{Edge, Handle, NodeId},
    handlegraph::*,
    hashgraph::HashGraph,
    mutablehandlegraph::*,
    parser::parse_file_to_graph,
    pathgraph::PathHandleGraph,
};
use time::Instant;
//use log::{info, warn, error};

fn read_small_gfa2() -> HashGraph {
    let mut graph = HashGraph::new();
    match parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn read_ditto() -> HashGraph {
    let mut graph = HashGraph::new();
    match parse_file_to_graph("./tests/gfa2_files/ditto.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn read_medium_gfa2() -> HashGraph {
    let start = Instant::now();
    let mut graph = HashGraph::new();
    match parse_file_to_graph("./tests/big_files/test.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    println!("Create graph from file: {:?}", start.elapsed());
    graph
}

fn read_medium_gfa1() -> HashGraph {
    let start = Instant::now();
    let mut graph = HashGraph::new();
    match parse_file_to_graph("./tests/big_files/test.gfa") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    println!("Create graph from file: {:?}", start.elapsed());
    graph
}

fn read_big_gfa2() -> HashGraph {
    let start = Instant::now();
    let mut graph = HashGraph::new();
    match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    println!("Create graph from file: {:?}", start.elapsed());
    graph
}

fn read_big_gfa1() -> HashGraph {
    let start = Instant::now();
    let mut graph = HashGraph::new();
    match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    println!("Create graph from file: {:?}", start.elapsed());
    graph
}

#[test]
#[ignore]
fn clear_big_graph() {
    /*
    Create graph from file: Duration { seconds: 460, nanoseconds: 665865700 }
    Clear graph: Duration { seconds: 13, nanoseconds: 755750000 }
    */
    let mut graph = read_big_gfa1();
    let start = Instant::now();
    graph.clear_graph();
    println!("Clear graph: {:?}", start.elapsed());
}

#[test]
fn ditto() {
    let mut graph = read_ditto();
    //println!("{:#?}", graph);
    //graph.print_graph();
    let node: u64 = 1;
    let start = Instant::now();
    match graph.remove_handle(node) {
        Ok(_) => {
            println!("{:?}", start.elapsed());
            //println!("{:#?}", graph);
            graph.print_graph();
        }
        Err(why) => println!("Error: {}", why),
    }

    let left = Handle::new(7, Orientation::Forward);
    let right = Handle::new(8, Orientation::Forward);
    let edge = Edge(left, right);
    let start = Instant::now();
    match graph.remove_edge(edge) {
        Ok(_) => {
            //graph.print_graph();
            println!("{:?}", start.elapsed());
        }
        Err(why) => println!("Error: {}", why),
    }
}

#[test]
fn create_medium_graph() {
    // Create graph from file: Duration { seconds: 0, nanoseconds: 711420200 } (main PC)
    // Create graph from file: Duration { seconds: 1, nanoseconds: 814588300 } (Portable PC)
    // Create graph from file: Duration { seconds: 0, nanoseconds: 912034700 } (Portable PC with rayon)
    let g = read_medium_gfa1();
    // nodes: 4058     edges: 9498     paths: 7
    let nodes = g.handles().count();
    let edges = g.edges().count();
    let paths = g.paths().count();
    println!("nodes: {}\tedges: {}\tpaths: {}", nodes, edges, paths);
    //g.print_graph();

    // Create graph from file: Duration { seconds: 1, nanoseconds: 153754100 } (main PC)
    // Create graph from file: Duration { seconds: 2, nanoseconds: 300047700 } (Portable PC)
    // Create graph from file: Duration { seconds: 1, nanoseconds: 230163700 } (Portable PC with rayon)
    let g = read_medium_gfa2();
    // nodes: 4058     edges: 9498     paths: 7
    let nodes = g.handles().count();
    let edges = g.edges().count();
    let paths = g.paths().count();
    println!("nodes: {}\tedges: {}\tpaths: {}", nodes, edges, paths);
    //g.print_graph();
}

#[test]
fn mod_graph_from_medium_gfa() {
    /* MAIN PC
    Create graph from file: Duration { seconds: 0, nanoseconds: 712439400 }
    remove 1000 nodes from graph: Duration { seconds: 5, nanoseconds: 404610100 }
    remove 1000 small edges: Duration { seconds: 5, nanoseconds: 498107900 }
    remove 1 big edge (form of 1000 edges): Duration { seconds: 5, nanoseconds: 593331400 }
    */
    /* PORTABLE PC
    Create graph from file: Duration { seconds: 1, nanoseconds: 473749400 }
    remove 1000 nodes from graph: Duration { seconds: 10, nanoseconds: 552469200 }
    remove 1000 small edges: Duration { seconds: 10, nanoseconds: 147140900 }
    remove 1 big edge (form of 1000 edges): Duration { seconds: 10, nanoseconds: 7050100 }
     */
    //let mut graph = read_medium_gfa1();

    /* MAIN PC
    Create graph from file: Duration { seconds: 1, nanoseconds: 275933800 }
    remove 1000 nodes from graph: Duration { seconds: 5, nanoseconds: 442340100 }
    remove 1000 small edges: Duration { seconds: 5, nanoseconds: 434688800 }
    remove 1 big edge (form of 1000 edges): Duration { seconds: 5, nanoseconds: 524103000 }
    */
    let mut graph = read_medium_gfa2();

    let start = Instant::now();
    for i in 1..1001_u64 {
        match graph.remove_handle(i) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }
    println!("remove 1000 nodes from graph: {:?}", start.elapsed());

    // add nodes, edges and paths
    for i in 0..1_001_u64 {
        match graph.create_handle(5_000 + i, b"TEST_SEQUENCE") {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }

    for i in 1..1_001_u64 {
        let left = Handle::new(5_000 + i - 1, Orientation::Forward);
        let right = Handle::new(5_000 + i, Orientation::Forward);
        let edge = Edge(left, right);
        match graph.create_edge(edge) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }

    let start = Instant::now();
    for i in 1..1_001_u64 {
        let left = Handle::new(5_000 + i - 1, Orientation::Forward);
        let right = Handle::new(5_000 + i, Orientation::Forward);
        let edge = Edge(left, right);
        match graph.remove_edge(edge) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }
    println!("small edge: {:?}", start.elapsed());

    let left = Handle::new(5_000, Orientation::Forward);
    for i in 1..1_001_u64 {
        let right = Handle::new(5_000 + i, Orientation::Forward);
        let edge = Edge(left, right);
        match graph.create_edge(edge) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }

    let start = Instant::now();
    for i in 1..1_001_u64 {
        let right = Handle::new(5_000 + i, Orientation::Forward);
        let edge = Edge(left, right);
        match graph.remove_edge(edge) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }
    println!("big edge: {:?}", start.elapsed());
}

#[test]
#[ignore]
fn create_big_graph() {
    /*
    Create graph from file: Duration { seconds: 509, nanoseconds: 429796900 }
    */
    let _g = read_big_gfa1();
    // nodes: 715018   edges: 985445   paths: 0
    let nodes = _g.handles().count();
    let edges = _g.edges().count();
    let paths = _g.paths().count();
    println!("nodes: {}\tedges: {}\tpaths: {}", nodes, edges, paths);

    /*
    Create graph from file: Duration { seconds: 531, nanoseconds: 725273400 }
    */
    let _g = read_big_gfa2();
    // nodes: 715018   edges: 985445   paths: 0
    let nodes = _g.handles().count();
    let edges = _g.edges().count();
    let paths = _g.paths().count();
    println!("nodes: {}\tedges: {}\tpaths: {}", nodes, edges, paths);
}

#[test]
#[ignore]
fn mod_graph_from_big_gfa1() {
    /*
    Create graph from file: Duration { seconds: 499, nanoseconds: 440272800 }
    remove 100 nodes from graph: Duration { seconds: 390, nanoseconds: 430955800 }
    add 100 nodes and edges: Duration { seconds: 0, nanoseconds: 335200 }
    add 100 paths: Duration { seconds: 0, nanoseconds: 205700 }
    remove 100 edges with 2 occurrences [l - r]: Duration { seconds: 379, nanoseconds: 800566400 }
    remove 1 edge with 100 occurrences [1..100]: Duration { seconds: 379, nanoseconds: 139578300 }
    */
    let mut graph = read_big_gfa1();

    let start = Instant::now();
    for i in 1..101_u64 {
        let id: u64 = format!("{}{}", 115, i).parse::<u64>().unwrap();
        match graph.remove_handle(id) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }
    println!("remove 100 nodes from graph: {:?}", start.elapsed());

    let start = Instant::now();
    // add nodes, edges and paths
    for i in 0..101_u64 {
        match graph.create_handle(42_000 + i, b"TEST_SEQUENCE") {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        if i > 0 {
            let left = Handle::new(42_000 + i - 1, Orientation::Forward);
            let right = Handle::new(42_000 + i, Orientation::Forward);
            let edge = Edge(left, right);
            match graph.create_edge(edge) {
                Ok(_) => (),
                Err(why) => println!("Error: {}", why),
            };
        }
    }
    println!("add 100 nodes and edges: {:?}", start.elapsed());

    let start = Instant::now();
    let path_id = b"default_path_id";
    let path = graph.create_path_handle(path_id, false);
    for i in 0..101_u64 {
        let handle = Handle::new(42_000 + i, Orientation::Forward);
        match graph.append_step(&path, handle) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }
    println!("add 100 paths: {:?}", start.elapsed());

    let start = Instant::now();
    for i in 1..101_u64 {
        let left = Handle::new(42_000 + i - 1, Orientation::Forward);
        let right = Handle::new(42_000 + i, Orientation::Forward);
        let edge = Edge(left, right);
        match graph.remove_edge(edge) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }
    println!(
        "remove 100 edges with 2 occurrences [l - r]: {:?}",
        start.elapsed()
    );

    let left = Handle::new(42_000, Orientation::Forward);
    for i in 1..101_u64 {
        let right = Handle::new(42_000 + i, Orientation::Forward);
        let edge = Edge(left, right);
        match graph.create_edge(edge) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }

    let start = Instant::now();
    for i in 1..101_u64 {
        let right = Handle::new(42_000 + i, Orientation::Forward);
        let edge = Edge(left, right);
        match graph.remove_edge(edge) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }
    println!(
        "remove 1 edge with 100 occurrences [1..100]: {:?}",
        start.elapsed()
    );
}

#[test]
fn add_node() {
    let mut graph = read_small_gfa2();
    let node: NodeId = 42.into();
    let sequence = b"TEST_SEQUENCE";

    match graph.create_handle(node, sequence) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}

#[test]
fn add_edge() {
    let mut graph = read_small_gfa2();

    let node: NodeId = 42.into();
    let sequence = b"TEST_SEQUENCE";
    match graph.create_handle(node, sequence) {
        Ok(_) => {
            let left: Handle = Handle::new(42_u64, Orientation::Backward);
            let right: Handle = Handle::new(13_u64, Orientation::Forward);
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
    match graph.create_handle(node, sequence) {
        Ok(_) => {
            let left: Handle = Handle::new(42_u64, Orientation::Backward);
            let right: Handle = Handle::new(13_u64, Orientation::Forward);
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

                        let handle =
                            Handle::new(seq_id.parse::<u64>().unwrap(), orient);
                        match graph.append_step(&path, handle) {
                            Ok(_) => (),
                            Err(why) => println!("Error: {}", why),
                        };
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

    let left: Handle = Handle::new(11_u64, Orientation::Forward);
    let right: Handle = Handle::new(13_u64, Orientation::Forward);
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
    let path_id = graph
        .name_to_path_handle(path)
        .expect("Error, path did not exists");
    graph.destroy_path(&path_id);
    graph.print_graph();
}

#[test]
fn remove_node_from_path() {
    let mut graph = read_small_gfa2();

    let path = b"14";
    let node: u64 = 11_u64;

    match graph.remove_step(path, node) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error: {}", why),
    }
}

#[test]
fn modify_node_from_path() {
    let mut graph = read_small_gfa2();

    let path = b"14";
    let node: u64 = 11_u64;
    let nodea = Handle::new(13_u64, Orientation::Forward);

    match graph.modify_step(path, node, nodea) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error: {}", why),
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

    let left: Handle = Handle::new(11_u64, Orientation::Forward);
    let right: Handle = Handle::new(13_u64, Orientation::Forward);
    let edge: Edge = Edge(left, right);
    let new_left: Handle = Handle::new(11_u64, Orientation::Forward);
    let new_right: Handle = Handle::new(11_u64, Orientation::Forward);
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
        Handle::new(11_u64, Orientation::Forward),
        Handle::new(11_u64, Orientation::Forward),
        Handle::new(11_u64, Orientation::Forward),
    ];
    match graph.rewrite_path(path, path_sequence) {
        Ok(_) => graph.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}
