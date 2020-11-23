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
    CREATE GRAPH FROM MID GFA
                        time:   [95.183 ms 95.269 ms 95.356 ms]
                        change: [-0.7161% -0.5547% -0.3996%] (p = 0.00 < 0.05)
                        Change within noise threshold.
    Found 6 outliers among 100 measurements (6.00%)
      2 (2.00%) low mild
      3 (3.00%) high mild
      1 (1.00%) high severe
    */
    c.bench_function("CREATE GRAPH FROM MID GFA", |b| {
        b.iter(|| create_graph_from_medium_gfa1())
    });

    /*
    CREATE GRAPH FROM MID GFA2
                        time:   [119.58 ms 119.71 ms 119.86 ms]
                        change: [-0.6557% -0.4369% -0.2326%] (p = 0.00 < 0.05)
                        Change within noise threshold.
    Found 6 outliers among 100 measurements (6.00%)
      4 (4.00%) high mild
      2 (2.00%) high severe
    */
    c.bench_function("CREATE GRAPH FROM MID GFA2", |b| {
        b.iter(|| create_graph_from_medium_gfa2())
    });

    /*
    MODIFY GRAPH FROM MID GFA2
                        time:   [908.09 ms 909.66 ms 911.30 ms]
                        change: [-2.8493% -1.6381% -0.9226%] (p = 0.00 < 0.05)
                        Change within noise threshold.
    */
    c.bench_function("MODIFY GRAPH FROM MID GFA2", |b| {
        b.iter(|| mod_graph_from_medium_gfa2())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
