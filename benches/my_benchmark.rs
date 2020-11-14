use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gfahandlegraph::{hashgraph::HashGraph, parser::*};

/*
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
*/
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
    // time:   [95.027 ms 95.118 ms 95.216 ms]
    c.bench_function("mid GFA", |b| b.iter(|| create_graph_from_medium_gfa1()));
    // time:   [116.37 ms 118.10 ms 121.48 ms]
    c.bench_function("mid GFA2", |b| b.iter(|| create_graph_from_medium_gfa2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
