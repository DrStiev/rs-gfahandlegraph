pub mod error;
pub use self::error::*;

use crate::gfa::*;
use crate::gfa::{gfa1::Line as Line1, gfa2::Line as Line2};
use crate::hashgraph::HashGraph;
use crate::parser::error::ParserTolerance;

use bstr::{BStr, BString, ByteSlice};
use lazy_static::lazy_static;
use regex::bytes::Regex;

/// Builder struct for GFAParsers
pub struct ParserBuilder {
    pub headers: bool,
    pub segments: bool,
    pub fragments: bool,
    pub edges: bool,
    pub gaps: bool,
    pub groups_o: bool,
    pub groups_u: bool,
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
            gaps: false,
            groups_o: false,
            groups_u: false,
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
            gaps: true,
            groups_o: true,
            groups_u: true,
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
            gaps: self.gaps,
            groups_o: self.groups_o,
            groups_u: self.groups_u,
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
    gaps: bool,
    groups_o: bool,
    groups_u: bool,
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

    pub fn parse_file_to_graph<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<HashGraph, ParseError> {
        use std::ffi::OsStr;
        use {
            bstr::io::BufReadExt,
            std::{fs::File, io::BufReader},
        };

        let file = File::open(path.as_ref())?;
        match path.as_ref().extension().and_then(OsStr::to_str).unwrap() {
            "gfa2" => {
                let lines = BufReader::new(file).byte_lines();
                let mut graph = HashGraph::new();

                for line in lines {
                    match self.parse_gfa2_line(line?.as_ref()) {
                        Ok(parsed) => graph = HashGraph::insert_gfa2_line(graph, parsed),
                        Err(err) if err.can_safely_continue(&self.tolerance) => (),
                        Err(why) => return Err(why),
                    }
                }
                Ok(graph)
            }
            "gfa" => {
                let lines = BufReader::new(file).byte_lines();
                let mut graph = HashGraph::new();

                for line in lines {
                    match self.parse_gfa_line(line?.as_ref()) {
                        Ok(parsed) => graph = HashGraph::insert_gfa1_line(graph, parsed),
                        Err(err) if err.can_safely_continue(&self.tolerance) => (),
                        Err(why) => return Err(why),
                    }
                }
                Ok(graph)
            }
            _ => return Err(ParseError::ExtensionError()),
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
