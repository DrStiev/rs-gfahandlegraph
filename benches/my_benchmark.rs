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
    match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}

fn create_graph_from_big_gfa1() -> HashGraph {
    let mut graph = HashGraph::new();
    match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa") {
        Ok(g) => graph = g,
        Err(why) => println!("Error {}", why),
    }
    graph
}
*/

fn create_graph_from_medium_gfa2() -> HashGraph {
    let mut graph = HashGraph::new();
    match parse_file_to_graph("./tests/big_files/test.gfa2") {
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
        match graph.create_handle(5_000 + i as u64, b"TEST_SEQUENCE") {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        if i > 1 {
            let left = Handle::new(5_000 + i - 1, Orientation::Forward);
            let right = Handle::new(5_000 + i, Orientation::Forward);
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
        let handle = Handle::new(5_000 + i, Orientation::Forward);
        match graph.append_step(&path, handle) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
    }
    true
}

fn create_graph_from_medium_gfa1() -> HashGraph {
    let mut graph = HashGraph::new();
    match parse_file_to_graph("./tests/big_files/test.gfa") {
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
    Benchmarking CREATE GRAPH FROM MID GFA: Warming up for 3.0000 s
    Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.1s, or reduce sample count to 90.
    CREATE GRAPH FROM MID GFA
                            time:   [47.304 ms 47.565 ms 47.906 ms]
                            change: [-24.942% -24.520% -23.874%] (p = 0.00 < 0.05)
                            Performance has improved.
    Found 4 outliers among 100 measurements (4.00%)
      2 (2.00%) high mild
      2 (2.00%) high severe
        */
    c.bench_function("CREATE GRAPH FROM MID GFA", |b| {
        b.iter(|| create_graph_from_medium_gfa1())
    });

    /*
    Benchmarking CREATE GRAPH FROM MID GFA2: Warming up for 3.0000 s
    Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.7s, or reduce sample count to 80.
    CREATE GRAPH FROM MID GFA2
                            time:   [56.428 ms 58.076 ms 60.217 ms]
                            change: [-52.468% -51.155% -49.404%] (p = 0.00 < 0.05)
                            Performance has improved.
    Found 11 outliers among 100 measurements (11.00%)
      11 (11.00%) high severe
        */
    c.bench_function("CREATE GRAPH FROM MID GFA2", |b| {
        b.iter(|| create_graph_from_medium_gfa2())
    });

    /*
    Benchmarking MODIFY GRAPH FROM MID GFA2: Warming up for 3.0000 s
    Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 90.4s, or reduce sample count to 10.
    MODIFY GRAPH FROM MID GFA2
                            time:   [911.72 ms 924.17 ms 937.83 ms]
                            change: [+0.5052% +1.8391% +3.3773%] (p = 0.01 < 0.05)
                            Change within noise threshold.
    Found 7 outliers among 100 measurements (7.00%)
      5 (5.00%) high mild
      2 (2.00%) high severe
        */
    c.bench_function("MODIFY GRAPH FROM MID GFA2", |b| {
        b.iter(|| mod_graph_from_medium_gfa2())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
