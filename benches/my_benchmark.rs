use criterion::{criterion_group, criterion_main, Criterion};
use gfahandlegraph::{
    gfa::orientation::Orientation,
    handle::{Edge, Handle},
    hashgraph::HashGraph,
    mutablehandlegraph::*,
    parser::*,
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

    let random_node = 15_usize;
    match graph.remove_handle(random_node) {
        Err(why) => println!("Error: {}", why),
        _ => (),
    };

    let edge = Edge(
        Handle::new(4046, Orientation::Forward),
        Handle::new(2346, Orientation::Forward),
    );
    match graph.remove_edge(edge) {
        Err(why) => println!("Error: {}", why),
        _ => (),
    };
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
                            time:   [44.878 ms 45.034 ms 45.224 ms]
                            change: [-11.071% -8.9593% -7.1015%] (p = 0.00 < 0.05)
                            Performance has improved.
    Found 7 outliers among 100 measurements (7.00%)
      5 (5.00%) high mild
      2 (2.00%) high severe

    CREATE GRAPH FROM MID GFA2
                            time:   [14.021 ms 14.050 ms 14.083 ms]
                            change: [-1.4801% -1.0474% -0.6352%] (p = 0.00 < 0.05)
                            Change within noise threshold.
    Found 4 outliers among 100 measurements (4.00%)
      2 (2.00%) high mild
      2 (2.00%) high severe

    MODIFY GRAPH FROM MID GFA2
                            time:   [15.563 ms 15.586 ms 15.609 ms]
                            change: [-97.327% -97.320% -97.314%] (p = 0.00 < 0.05)
                            Performance has improved.
    Found 2 outliers among 100 measurements (2.00%)
      2 (2.00%) high mild
    */
    c.bench_function("CREATE GRAPH FROM MID GFA", |b| {
        b.iter(|| create_graph_from_medium_gfa1())
    });
    c.bench_function("CREATE GRAPH FROM MID GFA2", |b| {
        b.iter(|| create_graph_from_medium_gfa2())
    });
    c.bench_function("MODIFY GRAPH FROM MID GFA2", |b| {
        b.iter(|| mod_graph_from_medium_gfa2())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
