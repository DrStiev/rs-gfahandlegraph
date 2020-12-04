pub mod error;
pub mod parser_gfa1;
pub mod parser_gfa2;

pub use self::error::*;
pub use self::parser_gfa1::*;
pub use self::parser_gfa2::*;

use crate::gfa::{gfa1::GFA, gfa2::GFA2};
use crate::hashgraph::HashGraph;

/// Function that given a
/// [`GFA`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa1/struct.GFA.html)
/// or
/// [`GFA2`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa2/struct.GFA2.html)
/// file as input, creates the
/// corresponding
/// [`HashGraph`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/hashgraph/graph/struct.HashGraph.html).
/// # Example
/// ```ignore
/// match parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
///     Ok(g) => g.print_graph(),
///     Err(why) => println!("Error {}", why),
/// }
///
/// /*
/// Graph: {
///     Nodes: {
///         13: CTTGATT
///         12: TCAAGG
///         11: ACCTT
///     }
///     Edges: {
///         12- -- 13+
///         11+ -- 12-
///         11+ -- 13+
///     }
///     Paths: {
///         14: ACCTT -> CTTGATT
///         15: ACCTT -> CCTTGA -> CTTGATT
///     }
/// }
/// */
/// ```
pub fn parse_file_to_graph<P: AsRef<std::path::Path>>(path: P) -> Result<HashGraph, ParseError> {
    use crate::hashgraph::graph::FileType;
    use std::ffi::OsStr;

    match path.as_ref().extension().and_then(OsStr::to_str).unwrap() {
        "gfa2" => {
            let mut graph = HashGraph::default();
            let parser = GFA2Parser::default();
            let gfa2: GFA2 = parser.parse_file(path)?;

            match graph.create_graph(FileType::GFA2(gfa2)) {
                Ok(g) => Ok(g),
                Err(why) => Err(ParseError::ConversionGFAToGraph(why.to_string())),
            }
        }
        "gfa" => {
            let mut graph = HashGraph::default();
            let parser = GFAParser::default();
            let gfa: GFA = parser.parse_file(path)?;

            match graph.create_graph(FileType::GFA(gfa)) {
                Ok(g) => Ok(g),
                Err(why) => Err(ParseError::ConversionGFAToGraph(why.to_string())),
            }
        }
        _ => Err(ParseError::ExtensionError()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hashgraph::graph::FileType;
    use crate::mutablehandlegraph::SubtractiveHandleGraph;
    use bstr::io::BufReadExt;
    use std::fs::File;
    use std::io::BufReader;
    use time::Instant;

    #[test]
    fn can_create_graph_from_gfa2_file() {
        match parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
            Ok(g) => println!("{}", g), //g.print_graph(),
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn ditto_test() {
        match parse_file_to_graph("./tests/gfa2_files/irl.gfa2") {
            Ok(g) => println!("{}", g), //g.print_graph(),
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn can_create_graph_from_gfa1_file() {
        match parse_file_to_graph("./tests/gfa1_files/lil.gfa") {
            Ok(g) => println!("{}", g),
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    #[ignore]
    fn big_file() {
        /*
        Create GFAObject from ./tests/big_files/ape-4-0.10b.gfa: Duration { seconds: 25, nanoseconds: 99623600 }
        Nodes: 715018	Edges: 985462	Paths: 0
        Create HashGraph: Duration { seconds: 2, nanoseconds: 605987000 }
        Create GFAObject from ./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa: Duration { seconds: 20, nanoseconds: 501353900 }
        Nodes: 241419	Edges: 355105	Paths: 0
        Create HashGraph: Duration { seconds: 1, nanoseconds: 741834000 }
        Create GFAObject from ./tests/big_files/GRCh38-20-0.10b.gfa: Duration { seconds: 19, nanoseconds: 233540200 }
        Nodes: 148618	Edges: 214995	Paths: 0
        Create HashGraph: Duration { seconds: 1, nanoseconds: 521270900 }
        Create GFAObject from ./tests/big_files/ape-4-0.10b.gfa2: Duration { seconds: 25, nanoseconds: 515242300 }
        Nodes: 715018	Edges: 985462	Paths: 0
        Create HashGraph: Duration { seconds: 2, nanoseconds: 966349800 }
        Create GFAObject from ./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa2: Duration { seconds: 22, nanoseconds: 522741600 }
        Nodes: 241419	Edges: 355105	Paths: 0
        Create HashGraph: Duration { seconds: 1, nanoseconds: 835280800 }
        Create GFAObject from ./tests/big_files/GRCh38-20-0.10b.gfa2: Duration { seconds: 20, nanoseconds: 638227800 }
        Nodes: 148618	Edges: 214995	Paths: 0
        Create HashGraph: Duration { seconds: 1, nanoseconds: 573665700 }
         */
        const FILES: [&str; 3] = [
            "./tests/big_files/ape-4-0.10b.gfa",
            "./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa",
            "./tests/big_files/GRCh38-20-0.10b.gfa",
        ];
        for i in 0..3 {
            let start = Instant::now();
            let parser = GFAParser::default();
            let gfa = parser.parse_file(FILES[i].to_string()).unwrap();
            println!(
                "Create GFAObject from {}: {:?}",
                FILES[i].to_string(),
                start.elapsed()
            );
            println!(
                "Nodes: {}\tEdges: {}\tPaths: {}",
                gfa.segments.len(),
                gfa.links.len(),
                gfa.paths.len()
            );

            let start = Instant::now();
            let mut graph = HashGraph::default();
            match graph.create_graph(FileType::GFA(gfa)) {
                Ok(_) => println!("Create HashGraph: {:?}", start.elapsed()),
                Err(why) => println!("Error: {}", why),
            };

            graph.clear_graph();
        }

        const FILES2: [&str; 3] = [
            "./tests/big_files/ape-4-0.10b.gfa2",
            "./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa2",
            "./tests/big_files/GRCh38-20-0.10b.gfa2",
        ];
        for i in 0..3 {
            let start = Instant::now();
            let parser = GFA2Parser::default();
            let gfa = parser.parse_file(FILES2[i].to_string()).unwrap();
            println!(
                "Create GFAObject from {}: {:?}",
                FILES2[i].to_string(),
                start.elapsed()
            );
            println!(
                "Nodes: {}\tEdges: {}\tPaths: {}",
                gfa.segments.len(),
                gfa.edges.len(),
                gfa.groups_o.len()
            );

            let start = Instant::now();
            let mut graph = HashGraph::default();
            match graph.create_graph(FileType::GFA2(gfa)) {
                Ok(_) => println!("Create HashGraph: {:?}", start.elapsed()),
                Err(why) => println!("Error: {}", why),
            };

            graph.clear_graph();
        }
    }

    #[test]
    #[ignore]
    fn read_big_file() {
        /*
        Read file ./tests/big_files/ape-4-0.10b.gfa (has 1700480 lines): Duration { seconds: 20, nanoseconds: 779014300 }
        Read file ./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa (has 596524 lines): Duration { seconds: 20, nanoseconds: 306792000 }
        Read file ./tests/big_files/GRCh38-20-0.10b.gfa (has 363613 lines): Duration { seconds: 18, nanoseconds: 951543200 }
        Read file ./tests/big_files/ape-4-0.10b.gfa2 (has 1700480 lines): Duration { seconds: 23, nanoseconds: 392768900 }
        Read file ./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa2 (has 596524 lines): Duration { seconds: 21, nanoseconds: 349775300 }
        Read file ./tests/big_files/GRCh38-20-0.10b.gfa2 (has 363613 lines): Duration { seconds: 19, nanoseconds: 231613100 }
         */
        const FILES: [&str; 6] = [
            "./tests/big_files/ape-4-0.10b.gfa",
            "./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa",
            "./tests/big_files/GRCh38-20-0.10b.gfa",
            "./tests/big_files/ape-4-0.10b.gfa2",
            "./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa2",
            "./tests/big_files/GRCh38-20-0.10b.gfa2",
        ];
        for i in 0..6 {
            let start = Instant::now();
            let lines = BufReader::new(File::open(FILES[i].to_string()).unwrap()).byte_lines();
            let mut count = 0;
            lines.for_each(|_l| count += 1);
            println!(
                "Read file {} (has {} lines): {:?}",
                FILES[i].to_string(),
                count,
                start.elapsed()
            );
        }
    }
}
