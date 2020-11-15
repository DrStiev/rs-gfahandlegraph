use criterion::{criterion_group, criterion_main, Criterion};
use gfahandlegraph::{
    gfa::orientation::Orientation,
    handle::{Edge, Handle},
    hashgraph::HashGraph,
    mutablehandlegraph::*,
    parser::*,
    pathgraph::PathHandleGraph,
};

/*
fn create_graph_from_big_gfa2() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn create_graph_from_big_gfa1() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}
*/

fn create_graph_from_medium_gfa2() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/big_files/test.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn mod_graph_from_medium_gfa2() -> bool {
    let mut graph = create_graph_from_medium_gfa2();

    for i in 1..1001 {
        match graph.remove_handle(i as u64) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }

    const PATHS: [&[u8]; 3] = [
        b"gi|568815592:32578768-32589835",
        b"gi|568815529:3998044-4011446",
        b"gi|568815551:3814534-3830133",
    ];
    for i in 1..PATHS.len() {
        let path_name: &[u8] = PATHS.get(i as usize).unwrap();
        match graph.remove_path(path_name) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }

    let left = Handle::new(2138, Orientation::Backward);
    let right = Handle::new(2137, Orientation::Backward);
    let edge = Edge(left, right);
    match graph.remove_edge(edge) {
        Ok(_) => (),
        Err(why) => println!("Error: {}", why),
    };

    let left = Handle::new(2139, Orientation::Forward);
    let right = Handle::new(2140, Orientation::Forward);
    let edge = Edge(left, right);
    match graph.remove_edge(edge) {
        Ok(_) => (),
        Err(why) => println!("Error: {}", why),
    };

    let left = Handle::new(2139, Orientation::Forward);
    let right = Handle::new(3090, Orientation::Forward);
    let edge = Edge(left, right);
    match graph.remove_edge(edge) {
        Ok(_) => (),
        Err(why) => println!("Error: {}", why),
    };

    let left = Handle::new(2139, Orientation::Backward);
    let right = Handle::new(2138, Orientation::Backward);
    let edge = Edge(left, right);
    match graph.remove_edge(edge) {
        Ok(_) => (),
        Err(why) => println!("Error: {}", why),
    };

    let left = Handle::new(2140, Orientation::Forward);
    let right = Handle::new(2141, Orientation::Forward);
    let edge = Edge(left, right);
    match graph.remove_edge(edge) {
        Ok(_) => (),
        Err(why) => println!("Error: {}", why),
    };

    // add nodes, edges and paths
    for i in 1..11 {
        match graph.create_handle(b"TEST_SEQUENCE", 5000 + i as u64) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        if i > 1 {
            let left = Handle::new(4000 + i - 1, Orientation::Forward);
            let right = Handle::new(4000 + i, Orientation::Forward);
            let edge = Edge(left, right);
            match graph.create_edge(edge) {
                Ok(_) => (),
                Err(why) => println!("Error: {}", why),
            };
        }
    }

    let paths: Vec<&[u8]> = vec![
        b"5001+", b"5002+", b"5003-", b"5004+", b"5005-", b"5006-", b"5007+", b"5008+", b"5009+",
        b"5010-",
    ];
    let path_id = b"default_path_id";
    // check if the path it's circular
    let last = paths.len() - 1;
    let is_circular: bool = paths[0] == paths[last];
    let path = graph.create_path_handle(path_id, is_circular);

    let handle = Handle::new(5001, Orientation::Forward);
    graph.append_step(&path, handle);
    let handle = Handle::new(5002, Orientation::Forward);
    graph.append_step(&path, handle);
    let handle = Handle::new(5003, Orientation::Backward);
    graph.append_step(&path, handle);
    let handle = Handle::new(5004, Orientation::Forward);
    graph.append_step(&path, handle);
    let handle = Handle::new(5005, Orientation::Backward);
    graph.append_step(&path, handle);
    let handle = Handle::new(5006, Orientation::Backward);
    graph.append_step(&path, handle);
    let handle = Handle::new(5007, Orientation::Forward);
    graph.append_step(&path, handle);
    let handle = Handle::new(5008, Orientation::Forward);
    graph.append_step(&path, handle);
    let handle = Handle::new(5009, Orientation::Forward);
    graph.append_step(&path, handle);
    let handle = Handle::new(5010, Orientation::Backward);
    graph.append_step(&path, handle);

    true
}

fn create_graph_from_medium_gfa1() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser<usize> = Parser::new();
    match parser.parse_file_to_graph("./tests/big_files/test.gfa") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

/*
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("big GFA", |b| b.iter(|| create_graph_from_big_gfa1()));
    c.bench_function("big GFA2", |b| b.iter(|| create_graph_from_big_gfa2()));
}
*/

fn criterion_benchmark(c: &mut Criterion) {
    // time:   [94.844 ms 95.228 ms 95.683 ms]
    // change: [-0.2046% +0.2172% +0.7086%] (p = 0.37 > 0.05)
    c.bench_function("CREATE GRAPH FROM MID GFA", |b| {
        b.iter(|| create_graph_from_medium_gfa1())
    });

    // time:   [109.98 ms 110.18 ms 110.40 ms]
    // change: [-0.5739% -0.3201% -0.0705%] (p = 0.01 < 0.05)
    c.bench_function("CREATE GRAPH FROM MID GFA2", |b| {
        b.iter(|| create_graph_from_medium_gfa2())
    });

    // time:   [1.0441 s 1.0461 s 1.0481 s]
    // change: [+1.4798% +1.7356% +1.9653%] (p = 0.00 < 0.05)
    c.bench_function("MOD MID GRAPH", |b| b.iter(|| mod_graph_from_medium_gfa2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
