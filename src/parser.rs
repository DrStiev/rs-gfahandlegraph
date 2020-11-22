pub mod error;
pub mod parse_tag;
pub mod parser_gfa1;
pub mod parser_gfa2;

pub use self::error::*;
pub use self::parser_gfa1::*;
pub use self::parser_gfa2::*;

use crate::gfa::{gfa1::GFA, gfa2::GFA2};
use crate::hashgraph::HashGraph;

/// Function that given a ```GFA``` or ```GFA2``` file as input, creates the
/// corresponding HashGraph.
/// # Example
/// ```ignore
/// let parser: Parser = Parser::new();
/// match parser.parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
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
///         15: ACCTT -> CCTTGA -(TCAAGG) -> CTTGATT
///     }
/// }
/// */
/// ```
pub fn parse_file_to_graph<P: AsRef<std::path::Path>>(path: P) -> Result<HashGraph, ParseError> {
    use crate::hashgraph::graph::FileType;
    use std::ffi::OsStr;

    match path.as_ref().extension().and_then(OsStr::to_str).unwrap() {
        "gfa2" => {
            let graph = HashGraph::default();
            let parser = GFA2Parser::default();
            let gfa2: GFA2 = parser.parse_file(path)?;

            match graph.create_graph(FileType::GFA2(gfa2)) {
                Ok(g) => Ok(g),
                Err(why) => Err(ParseError::ConversionGFAToGraph(why.to_string())),
            }
        }
        "gfa" => {
            let graph = HashGraph::default();
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
}
