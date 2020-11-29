pub mod error;
pub mod parse_tag;
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
pub fn parse_file_to_graph<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<HashGraph, ParseError> {
    use crate::hashgraph::graph::FileType;
    use std::ffi::OsStr;

    match path.as_ref().extension().and_then(OsStr::to_str).unwrap() {
        "gfa2" => {
            let mut graph = HashGraph::default();
            let parser = GFA2Parser::default();
            let gfa2: GFA2 = parser.parse_file(path)?;

            match graph.create_graph(FileType::GFA2(gfa2)) {
                Ok(g) => Ok(g),
                Err(why) => {
                    Err(ParseError::ConversionGFAToGraph(why.to_string()))
                }
            }
        }
        "gfa" => {
            let mut graph = HashGraph::default();
            let parser = GFAParser::default();
            let gfa: GFA = parser.parse_file(path)?;

            match graph.create_graph(FileType::GFA(gfa)) {
                Ok(g) => Ok(g),
                Err(why) => {
                    Err(ParseError::ConversionGFAToGraph(why.to_string()))
                }
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
            Ok(g) => g.print_graph(),
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn ditto_test() {
        match parse_file_to_graph("./tests/gfa2_files/irl.gfa2") {
            Ok(g) => g.print_graph(),
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn can_create_graph_from_gfa1_file() {
        match parse_file_to_graph("./tests/gfa1_files/lil.gfa") {
            Ok(g) => g.print_graph(),
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    #[ignore]
    fn big_file() {
        /* The real bottleneck is the creation of the GFAObject, for now IDK if it's the parsing part
        so the regex and the checking or if it's the building of the struct GFA inserting each line in
        the right field
        Create GFAObject from ./tests/big_files/ape-4-0.10b.gfa: Duration { seconds: 568, nanoseconds: 359199900 }
        Create HashGraph: Duration { seconds: 6, nanoseconds: 28818400 }

        Create GFAObject from ./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa: Duration { seconds: 433, nanoseconds: 506246400 }
        Create HashGraph: Duration { seconds: 2, nanoseconds: 980813800 }

        Create GFAObject from ./tests/big_files/GRCh38-20-0.10b.gfa: Duration { seconds: 393, nanoseconds: 569971300 }
        Create HashGraph: Duration { seconds: 2, nanoseconds: 233447000 }

        Create GFAObject from ./tests/big_files/ape-4-0.10b.gfa2: Duration { seconds: 508, nanoseconds: 68815000 }
        Create HashGraph: Duration { seconds: 7, nanoseconds: 691810000 }

        Create GFAObject from ./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa2: Duration { seconds: 414, nanoseconds: 719255900 }
        Create HashGraph: Duration { seconds: 3, nanoseconds: 738306100 }

        Create GFAObject from ./tests/big_files/GRCh38-20-0.10b.gfa2: Duration { seconds: 381, nanoseconds: 183538000 }
        Create HashGraph: Duration { seconds: 2, nanoseconds: 744088900 }
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
        Create gfa from file: Duration { seconds: 22, nanoseconds: 21791200 }
        File ./tests/big_files/ape-4-0.10b.gfa, number of lines: 1700480

        Create gfa from file: Duration { seconds: 20, nanoseconds: 230613000 }
        File ./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa, number of lines: 596524

        Create gfa from file: Duration { seconds: 19, nanoseconds: 95918000 }
        File ./tests/big_files/GRCh38-20-0.10b.gfa, number of lines: 363613

        Create gfa from file: Duration { seconds: 22, nanoseconds: 561371700 }
        File ./tests/big_files/ape-4-0.10b.gfa2, number of lines: 1700480

        Create gfa from file: Duration { seconds: 20, nanoseconds: 357006500 }
        File ./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa2, number of lines: 596524

        Create gfa from file: Duration { seconds: 20, nanoseconds: 185023100 }
        File ./tests/big_files/GRCh38-20-0.10b.gfa2, number of lines: 363613

         */
        const FILES: [&str; 3] = [
            "./tests/big_files/ape-4-0.10b.gfa",
            "./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa",
            "./tests/big_files/GRCh38-20-0.10b.gfa",
        ];
        for i in 0..3 {
            let start = Instant::now();
            let file = File::open(FILES[i].to_string()).unwrap();
            let lines = BufReader::new(file).byte_lines();
            let mut count: usize = 0;
            lines.for_each(|line| match line {
                Ok(_) => count += 1,
                Err(why) => println!("Error: {}", why),
            });
            println!("Create gfa from file: {:?}", start.elapsed());
            println!(
                "File {}, number of lines: {}",
                FILES[i].to_string(),
                count
            );
        }

        const FILES2: [&str; 3] = [
            "./tests/big_files/ape-4-0.10b.gfa2",
            "./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa2",
            "./tests/big_files/GRCh38-20-0.10b.gfa2",
        ];
        for i in 0..3 {
            let start = Instant::now();
            let file = File::open(FILES2[i].to_string()).unwrap();
            let lines = BufReader::new(file).byte_lines();
            let mut count: usize = 0;
            lines.for_each(|line| match line {
                Ok(_) => count += 1,
                Err(why) => println!("Error: {}", why),
            });
            println!("Create gfa from file: {:?}", start.elapsed());
            println!(
                "File {}, number of lines: {}",
                FILES2[i].to_string(),
                count
            );
        }
    }
}
