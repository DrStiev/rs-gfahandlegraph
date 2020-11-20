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
    let parser: Parser = Parser::new();
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

    // add nodes, edges and paths
    for i in 1..1001 {
        match graph.create_handle(b"TEST_SEQUENCE", 5000 + i as u64) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        if i > 1 {
            let left = Handle::new(5000 + i - 1, Orientation::Forward);
            let right = Handle::new(5000 + i, Orientation::Forward);
            let edge = Edge(left, right);
            match graph.create_edge(edge) {
                Ok(_) => (),
                Err(why) => println!("Error: {}", why),
            };
        }
    }

    let path_id = b"default_path_id";
    let path = graph.create_path_handle(path_id, false);
    for i in 1..1001 {
        let handle = Handle::new(5000 + i, Orientation::Forward);
        match graph.append_step(&path, handle) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }
    true
}

fn create_graph_from_medium_gfa1() -> HashGraph {
    let mut graph = HashGraph::new();
    let parser: Parser = Parser::new();
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
    /*
    Benchmarking CREATE GRAPH FROM MID GFA: Collecting 100 samples in estimated 9.5548 s (100
                                CREATE GRAPH FROM MID GFA
                        time:   [95.612 ms 95.812 ms 96.030 ms]
                        change: [-13.742% -13.448% -13.183%] (p = 0.00 < 0.05)
                        Performance has improved.
    Found 16 outliers among 100 measurements (16.00%)
    4 (4.00%) high mild
    12 (12.00%) high severe
    */
    c.bench_function("CREATE GRAPH FROM MID GFA", |b| {
        b.iter(|| create_graph_from_medium_gfa1())
    });

    /*
    Benchmarking CREATE GRAPH FROM MID GFA2: Collecting 100 samples in estimated 11.821 s (10
                                CREATE GRAPH FROM MID GFA2
                        time:   [117.90 ms 118.05 ms 118.23 ms]
                        change: [-1.8760% -1.7066% -1.5191%] (p = 0.00 < 0.05)
                        Performance has improved.
    Found 4 outliers among 100 measurements (4.00%)
    3 (3.00%) high mild
    1 (1.00%) high severe
    */
    c.bench_function("CREATE GRAPH FROM MID GFA2", |b| {
        b.iter(|| create_graph_from_medium_gfa2())
    });

    /*
    Benchmarking MODIFY GRAPH FROM MID GFA2: Collecting 100 samples in estimated 92.361 s (10
                                MODIFY GRAPH FROM MID GFA2
                        time:   [908.10 ms 911.29 ms 914.78 ms]
                        change: [-2.0146% -1.6197% -1.1838%] (p = 0.00 < 0.05)
                        Performance has improved.
    Found 4 outliers among 100 measurements (4.00%)
    3 (3.00%) high mild
    1 (1.00%) high severe
    */
    c.bench_function("MODIFY GRAPH FROM MID GFA2", |b| {
        b.iter(|| mod_graph_from_medium_gfa2())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
