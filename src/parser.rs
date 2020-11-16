pub mod error;
pub mod parser_gfa1;
pub mod parser_gfa2;

pub use self::error::*;
pub use self::parser_gfa1::*;
pub use self::parser_gfa2::*;

use crate::gfa::{
    gfa1::Line as Line1, gfa1::GFA, gfa2::Line as Line2, gfa2::GFA2, segment_id::SegmentId,
};
use crate::hashgraph::HashGraph;
use crate::parser::error::ParserTolerance;

use bstr::{BStr, BString, ByteSlice};

/// Builder struct for GFAParsers
pub struct ParserBuilder {
    pub headers: bool,
    pub segments: bool,
    pub fragments: bool,
    pub edges: bool,
    pub links: bool,
    pub gaps: bool,
    pub containments: bool,
    pub groups_o: bool,
    pub groups_u: bool,
    pub paths: bool,
    pub tolerance: ParserTolerance,
}

impl ParserBuilder {
    /// Parse no GFA lines, useful if you only want to parse one line type.
    pub fn none() -> Self {
        ParserBuilder {
            headers: false,
            segments: false,
            fragments: false,
            edges: false,
            links: false,
            gaps: false,
            containments: false,
            groups_o: false,
            groups_u: false,
            paths: false,
            tolerance: Default::default(),
        }
    }

    /// Parse all GFA lines.
    pub fn all() -> Self {
        ParserBuilder {
            headers: true,
            segments: true,
            fragments: true,
            edges: true,
            links: true,
            gaps: true,
            containments: true,
            groups_o: true,
            groups_u: true,
            paths: true,
            tolerance: Default::default(),
        }
    }

    pub fn ignore_errors(mut self) -> Self {
        self.tolerance = ParserTolerance::IgnoreAll;
        self
    }

    pub fn ignore_safe_errors(mut self) -> Self {
        self.tolerance = ParserTolerance::Safe;
        self
    }

    pub fn pedantic_errors(mut self) -> Self {
        self.tolerance = ParserTolerance::Pedantic;
        self
    }

    pub fn build<N: SegmentId>(self) -> Parser<N> {
        Parser {
            headers: self.headers,
            segments: self.segments,
            fragments: self.fragments,
            edges: self.edges,
            links: self.links,
            gaps: self.gaps,
            containments: self.containments,
            groups_o: self.groups_o,
            groups_u: self.groups_u,
            paths: self.paths,
            tolerance: self.tolerance,
            _segment_names: std::marker::PhantomData,
        }
    }

    pub fn build_usize_id(self) -> Parser<usize> {
        self.build()
    }

    pub fn build_bstr_id(self) -> Parser<BString> {
        self.build()
    }
}

#[derive(Clone)]
pub struct Parser<N: SegmentId> {
    headers: bool,
    segments: bool,
    fragments: bool,
    edges: bool,
    links: bool,
    gaps: bool,
    containments: bool,
    groups_o: bool,
    groups_u: bool,
    paths: bool,
    tolerance: ParserTolerance,
    _segment_names: std::marker::PhantomData<N>,
}

impl<N: SegmentId> Default for Parser<N> {
    fn default() -> Self {
        let config = ParserBuilder::all();
        config.build()
    }
}

impl Parser<usize> {
    pub fn new() -> Self {
        Default::default()
    }

    /// Function that given a gfa or gfa2 file as input, creates the
    /// corresponding graph.
    /// # Example
    /// ```ignore
    /// let parser: Parser<usize> = Parser::new();
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
    pub fn parse_file_to_graph<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<HashGraph, ParseError> {
        use crate::hashgraph::graph::FileType;
        use std::ffi::OsStr;
        use {
            bstr::io::BufReadExt,
            std::{fs::File, io::BufReader},
        };

        let file = File::open(path.as_ref())?;
        match path.as_ref().extension().and_then(OsStr::to_str).unwrap() {
            "gfa2" => {
                let lines = BufReader::new(file).byte_lines();

                let graph = HashGraph::default();
                let mut gfa2: GFA2<usize> = GFA2::default();

                for line in lines {
                    match self.parse_gfa2_line(line?.as_ref()) {
                        Ok(parsed) => gfa2.insert_line(parsed),
                        Err(err) if err.can_safely_continue(&self.tolerance) => (),
                        Err(why) => return Err(why),
                    }
                }

                match graph.create_graph(FileType::GFA2(gfa2)) {
                    Ok(g) => Ok(g),
                    Err(why) => Err(ParseError::ConversionGFAToGraph(why.to_string())),
                }
            }
            "gfa" => {
                let lines = BufReader::new(file).byte_lines();

                let graph = HashGraph::default();
                let mut gfa: GFA<usize> = GFA::default();

                for line in lines {
                    match self.parse_gfa_line(line?.as_ref()) {
                        Ok(parsed) => gfa.insert_line(parsed),
                        Err(err) if err.can_safely_continue(&self.tolerance) => (),
                        Err(why) => return Err(why),
                    }
                }

                match graph.create_graph(FileType::GFA(gfa)) {
                    Ok(g) => Ok(g),
                    Err(why) => Err(ParseError::ConversionGFAToGraph(why.to_string())),
                }
            }
            _ => Err(ParseError::ExtensionError()),
        }
    }

    fn parse_gfa2_line(&self, bytes: &[u8]) -> Result<Line2<usize>, ParseError> {
        use crate::gfa::gfa2::*;

        let line: &BStr = bytes.trim().as_ref();
        let mut fields = line.split_str(b"\t");
        let hdr = fields.next().ok_or(ParseError::EmptyLine)?;
        let invalid_line = |e: ParseFieldError| ParseError::invalid_line(e, bytes);

        let line = match hdr {
            b"H" if self.headers => Header::parse_line(fields).map(Header::wrap),
            b"S" if self.segments => Segment::parse_line(fields).map(Segment::wrap),
            b"F" if self.fragments => Fragment::parse_line(fields).map(Fragment::wrap),
            b"E" if self.edges => Edge::parse_line(fields).map(Edge::wrap),
            b"G" if self.gaps => Gap::parse_line(fields).map(Gap::wrap),
            b"O" if self.groups_o => GroupO::parse_line(fields).map(GroupO::wrap),
            b"U" if self.groups_u => GroupU::parse_line(fields).map(GroupU::wrap),
            _ => return Err(ParseError::UnknownLineType),
        }
        .map_err(invalid_line)?;
        Ok(line)
    }

    fn parse_gfa_line(&self, bytes: &[u8]) -> Result<Line1<usize>, ParseError> {
        use crate::gfa::gfa1::*;

        let line: &BStr = bytes.trim().as_ref();
        let mut fields = line.split_str(b"\t");
        let hdr = fields.next().ok_or(ParseError::EmptyLine)?;
        let invalid_line = |e: ParseFieldError| ParseError::invalid_line(e, bytes);

        let line = match hdr {
            b"H" if self.headers => Header::parse_line(fields).map(Header::wrap),
            b"S" if self.segments => Segment::parse_line(fields).map(Segment::wrap),
            b"L" if self.links => Link::parse_line(fields).map(Link::wrap),
            b"C" if self.containments => Containment::parse_line(fields).map(Containment::wrap),
            b"P" if self.paths => Path::parse_line(fields).map(Path::wrap),
            _ => return Err(ParseError::UnknownLineType),
        }
        .map_err(invalid_line)?;
        Ok(line)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_graph_from_gfa2_file() {
        let parser: Parser<usize> = Parser::new();
        match parser.parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
            Ok(g) => g.print_graph(),
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn ditto_test() {
        let parser: Parser<usize> = Parser::new();
        match parser.parse_file_to_graph("./tests/gfa2_files/irl.gfa2") {
            Ok(g) => g.print_graph(),
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn can_create_graph_from_gfa1_file() {
        let parser: Parser<usize> = Parser::new();
        match parser.parse_file_to_graph("./tests/gfa1_files/lil.gfa") {
            Ok(g) => g.print_graph(),
            Err(why) => println!("Error {}", why),
        }
    }
}
