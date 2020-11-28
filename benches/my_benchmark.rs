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
        CREATE GRAPH FROM MID GFA
                            time:   [62.412 ms 62.555 ms 62.698 ms]
                            change: [-18.929% -18.644% -18.364%] (p = 0.00 < 0.05)
                            Performance has improved.
    Found 5 outliers among 100 measurements (5.00%)
      4 (4.00%) low mild
      1 (1.00%) high mild

        */
    c.bench_function("CREATE GRAPH FROM MID GFA", |b| {
        b.iter(|| create_graph_from_medium_gfa1())
    });

    /*
            CREATE GRAPH FROM MID GFA2
                            time:   [119.17 ms 119.33 ms 119.55 ms]
                            change: [+22.426% +22.751% +23.097%] (p = 0.00 < 0.05)
                            Performance has regressed.
    Found 7 outliers among 100 measurements (7.00%)
      6 (6.00%) high mild
      1 (1.00%) high severe

        */
    c.bench_function("CREATE GRAPH FROM MID GFA2", |b| {
        b.iter(|| create_graph_from_medium_gfa2())
    });

    /*
        MODIFY GRAPH FROM MID GFA2
                            time:   [900.07 ms 903.22 ms 907.38 ms]
                            change: [-0.6081% +0.9582% +2.1199%] (p = 0.18 > 0.05)
                            No change in performance detected.
    Found 9 outliers among 100 measurements (9.00%)
      1 (1.00%) high mild
      8 (8.00%) high severe

        */
    c.bench_function("MODIFY GRAPH FROM MID GFA2", |b| {
        b.iter(|| mod_graph_from_medium_gfa2())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
