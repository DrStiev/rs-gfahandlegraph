use gfahandlegraph::{
    gfa::orientation::Orientation,
    handle::{Edge, Handle, NodeId},
    hashgraph::HashGraph,
    mutablehandlegraph::*,
    parser::parse_file_to_graph,
    pathgraph::PathHandleGraph,
};
use time::Instant;

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
    let node: u64 = 1;
    println!("{:#?}", graph);
    let start = Instant::now();
    match graph.remove_handle(node) {
        Ok(_) => {
            println!("{:?}", start.elapsed());
            println!("{}", graph);
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
    // Create graph from file: Duration { seconds: 0, nanoseconds: 59582400 }
    // nodes: 4058     edges: 9498     paths: 7
    read_medium_gfa1();

    // Create graph from file: Duration { seconds: 0, nanoseconds: 15734400 }
    // nodes: 4058     edges: 9498     paths: 7
    read_medium_gfa2();
}

#[test]
fn mod_graph_from_medium_gfa() {
    /*
    Create graph from file: Duration { seconds: 0, nanoseconds: 17247300 }
    remove node 15 from graph: Duration { seconds: 0, nanoseconds: 1111400 }
    remove edge Edge(Handle(8092), Handle(4692)) from graph: Duration { seconds: 0, nanoseconds: 1195000 }
    nodes: 4058     edges: 9498     paths: 7
     */
    //let mut graph = read_medium_gfa1();
    let mut graph = read_medium_gfa2();
    let random_node = 15_usize;
    let start = Instant::now();
    match graph.remove_handle(random_node) {
        Err(why) => println!("Error: {}", why),
        _ => (),
    };
    println!(
        "remove node {} from graph: {:?}",
        random_node,
        start.elapsed()
    );

    let edge = Edge(
        Handle::new(4046, Orientation::Forward),
        Handle::new(2346, Orientation::Forward),
    );
    let start = Instant::now();
    match graph.remove_edge(edge) {
        Err(why) => println!("Error: {}", why),
        _ => (),
    };
    println!("remove edge {:?} from graph: {:?}", edge, start.elapsed());
}

#[test]
#[ignore]
fn create_big_graph() {
    /*
    Create graph from file: Duration { seconds: 509, nanoseconds: 429796900 } (vanilla)
    Create graph from file: Duration { seconds: 83, nanoseconds: 32517500 } (rayon) (no optimization)
    Create graph from file: Duration { seconds: 39, nanoseconds: 585733300 } (rayon) (with optimization)
    nodes: 715018	edges: 985452	paths: 0
    */
    read_big_gfa1();

    /*
    Create graph from file: Duration { seconds: 531, nanoseconds: 725273400 } (vanilla)
    Create graph from file: Duration { seconds: 96, nanoseconds: 576846600 } (rayon) (no optimization)
    Create graph from file: Duration { seconds: 42, nanoseconds: 224143600 } (rayon) (with optimization)
    nodes: 715018	edges: 985454	paths: 0
    */
    read_big_gfa2();
}

#[test]
#[ignore]
fn mod_graph_from_big_gfa2() {
    /*
    Create graph from file: Duration { seconds: 32, nanoseconds: 822503100 }
    remove node 15 from graph: Duration { seconds: 3, nanoseconds: 822455400 }
    remove edge Edge(714952+, 440456-) from graph: Duration { seconds: 3, nanoseconds: 631893300 }
     */
    let mut graph = read_big_gfa2();
    let random_node = 11515_usize;
    let start = Instant::now();
    match graph.remove_handle(random_node) {
        Err(why) => println!("Error: {}", why),
        _ => (),
    };
    println!("remove node from graph: {:?}", start.elapsed());

    let edge = Edge(
        Handle::new(115_714_952, Orientation::Forward),
        Handle::new(115_440_456, Orientation::Backward),
    );
    let start = Instant::now();
    match graph.remove_edge(edge) {
        Err(why) => println!("Error: {}", why),
        _ => (),
    };
    println!("remove edge {:?} from graph: {:?}", edge, start.elapsed());
}

#[test]
fn add_node() {
    let mut graph = read_small_gfa2();
    let node: NodeId = 42.into();
    let sequence = b"TEST_SEQUENCE";

    match graph.create_handle(node, sequence) {
        Ok(_) => println!("{}", graph),
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
                Ok(_) => println!("{}", graph),
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

                        let handle = Handle::new(seq_id.parse::<u64>().unwrap(), orient);
                        match graph.append_step(&path, handle) {
                            Ok(_) => (),
                            Err(why) => println!("Error: {}", why),
                        };
                    }
                    println!("{}", graph)
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
        Ok(_) => println!("{}", graph),
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
        Ok(_) => println!("{}", graph),
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
    println!("{}", graph);
}

#[test]
fn remove_node_from_path() {
    let mut graph = read_small_gfa2();

    let path = b"14";
    let node: u64 = 11_u64;

    match graph.remove_step(path, node) {
        Ok(_) => println!("{}", graph),
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
        Ok(_) => println!("{}", graph),
        Err(why) => println!("Error: {}", why),
    }
}

#[test]
fn modify_node() {
    let mut graph = read_small_gfa2();
    let node: NodeId = 12.into();
    let sequence = b"MODIFIED_SEQUENCE";

    match graph.modify_handle(node, sequence) {
        Ok(_) => println!("{}", graph),
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
        Ok(_) => println!("{}", graph),
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
        Ok(_) => println!("{}", graph),
        Err(why) => println!("Error {}", why),
    }
}
