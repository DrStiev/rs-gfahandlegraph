/// This file provides the structure to create a GFA2 Object
use crate::gfa::orientation::*;
use crate::gfa::segment_id::*;

use bstr::{BString, ByteSlice};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct GFA2 {
    pub headers: Vec<Header>,
    pub segments: Vec<Segment>,
    pub fragments: Vec<Fragment>,
    pub edges: Vec<Edge>,
    pub gaps: Vec<Gap>,
    pub groups_o: Vec<GroupO>,
    pub groups_u: Vec<GroupU>,
}

impl fmt::Display for GFA2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.headers
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.segments
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.edges
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.groups_o
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
        )
    }
}

/// Enum containing the different kinds of GFA2 lines.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub enum Line {
    Header(Header),
    Segment(Segment),
    Fragment(Fragment),
    Edge(Edge),
    Gap(Gap),
    GroupO(GroupO),
    GroupU(GroupU),
}

macro_rules! some_line_fn {
    ($name:ident, $tgt:ty, $variant:path) => {
        impl Line {
            pub fn $name(self) -> Option<$tgt> {
                if let $variant(x) = self {
                    Some(x)
                } else {
                    None
                }
            }
        }
    };
}

some_line_fn!(some_header, Header, Line::Header);
some_line_fn!(some_segment, Segment, Line::Segment);
some_line_fn!(some_fragment, Fragment, Line::Fragment);
some_line_fn!(some_edge, Edge, Line::Edge);
some_line_fn!(some_gap, Gap, Line::Gap);
some_line_fn!(some_ogroup, GroupO, Line::GroupO);
some_line_fn!(some_ugroup, GroupU, Line::GroupU);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LineRef<'a> {
    Header(&'a Header),
    Segment(&'a Segment),
    Fragment(&'a Fragment),
    Edge(&'a Edge),
    Gap(&'a Gap),
    GroupO(&'a GroupO),
    GroupU(&'a GroupU),
}

macro_rules! some_line_ref_fn {
    ($name:ident, $tgt:ty, $variant:path) => {
        impl<'a> LineRef<'a> {
            pub fn $name(self) -> Option<&'a $tgt> {
                if let $variant(x) = self {
                    Some(x)
                } else {
                    None
                }
            }
        }
    };
}

some_line_ref_fn!(some_header, Header, LineRef::Header);
some_line_ref_fn!(some_segment, Segment, LineRef::Segment);
some_line_ref_fn!(some_fragment, Fragment, LineRef::Fragment);
some_line_ref_fn!(some_edge, Edge, LineRef::Edge);
some_line_ref_fn!(some_gap, Gap, LineRef::Gap);
some_line_ref_fn!(some_ogroup, GroupO, LineRef::GroupO);
some_line_ref_fn!(some_ugroup, GroupU, LineRef::GroupU);

/// Insert a GFA line (wrapped in the Line enum) into an existing
/// GFA. Simply pushes it into the corresponding Vec in the GFA,
/// or replaces the header, so there's no deduplication or sorting
/// taking place.
impl GFA2 {
    /// Insert a GFA line (wrapped in the Line enum) into an existing
    /// GFA. Simply pushes it into the corresponding Vec in the GFA,
    /// or replaces the header, so there's no deduplication or sorting
    /// taking place.
    #[inline]
    pub fn insert_line(&mut self, line: Line) {
        use Line::*;
        match line {
            Header(h) => self.headers.push(h),
            Segment(s) => self.segments.push(s),
            Fragment(f) => self.fragments.push(f),
            Edge(e) => self.edges.push(e),
            Gap(g) => self.gaps.push(g),
            GroupO(o) => self.groups_o.push(o),
            GroupU(u) => self.groups_u.push(u),
        }
    }

    /// Consume a GFA2 object to produce an iterator over all the lines
    /// contained within. The iterator first produces all headers then segments,
    /// fragments, edges, gaps, groups, comments and finally custom records
    pub fn lines_into_iter(self) -> impl Iterator<Item = Line> {
        use Line::*;
        let heads = self.headers.into_iter().map(Header);
        let segs = self.segments.into_iter().map(Segment);
        let frags = self.fragments.into_iter().map(Fragment);
        let edges = self.edges.into_iter().map(Edge);
        let gaps = self.gaps.into_iter().map(Gap);
        let ogroups = self.groups_o.into_iter().map(GroupO);
        let ugroups = self.groups_u.into_iter().map(GroupU);

        heads
            .chain(segs)
            .chain(frags)
            .chain(edges)
            .chain(gaps)
            .chain(ogroups)
            .chain(ugroups)
    }

    /// Return an iterator over references to the lines in the GFA2
    pub fn lines_iter(&'_ self) -> impl Iterator<Item = LineRef<'_>> {
        use LineRef::*;
        let heads = self.headers.iter().map(Header);
        let segs = self.segments.iter().map(Segment);
        let frags = self.fragments.iter().map(Fragment);
        let edges = self.edges.iter().map(Edge);
        let gaps = self.gaps.iter().map(Gap);
        let ogroups = self.groups_o.iter().map(GroupO);
        let ugroups = self.groups_u.iter().map(GroupU);

        heads
            .chain(segs)
            .chain(frags)
            .chain(edges)
            .chain(gaps)
            .chain(ogroups)
            .chain(ugroups)
    }
}

impl GFA2 {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Header {
    pub version: BString,
}

impl Header {
    pub fn new(version: &[u8]) -> Self {
        Header {
            version: version.into(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "H\t{}", self.version)
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct Segment {
    pub id: usize,
    pub sequence: BString,
}

impl Segment {
    #[inline]
    pub fn new(id: usize, sequence: &[u8]) -> Self {
        Segment {
            id,
            sequence: BString::from(sequence),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S\t{}\t{}", self.id, self.sequence)
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct Fragment {}

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct Edge {
    pub sid1: usize, // orientation as final char (+-)
    pub sid2: usize, // orientation as final char (+-)
}

impl Edge {
    #[inline]
    pub fn new(sid1: usize, sid2: usize) -> Self {
        Edge { sid1, sid2 }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.sid1.to_string().len() - 2;
        let sid1 = self.sid1.to_string()[..len].to_string();
        let sgn1 = match self.sid1.to_string()[len..].to_string().as_str() {
            "43" => "+",
            "45" => "-",
            _ => panic!("Orientation not found!"),
        };

        let len = self.sid2.to_string().len() - 2;
        let sid2 = self.sid2.to_string()[..len].to_string();
        let sgn2 = match self.sid2.to_string()[len..].to_string().as_str() {
            "43" => "+",
            "45" => "-",
            _ => panic!("Orientation not found!"),
        };

        write!(f, "E\t{}{}\t{}{}", sid1, sgn1, sid2, sgn2)
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct Gap {}

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct GroupO {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: BString, // optional id, can be either * or id tag
    pub var_field: BString, // "array" of ref (from 1 to n)
}

impl GroupO {
    #[inline]
    pub fn new(id: BString, var_field: BString) -> Self {
        GroupO { id, var_field }
    }

    /// parses (and copies) a segment ID in the group segment list
    #[inline]
    fn parse_segment_id(input: &[u8]) -> Option<(usize, Orientation)> {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Group O segment did not include orientation"),
        };
        let seg = &input[..last];
        let id = usize::parse_id(IdType::ID(), seg)?;
        Some((id, orient))
    }

    /// Produces an iterator over the usize segments of the given group
    #[inline]
    pub fn iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, Orientation)> + 'a {
        self.var_field
            .split_str(b" ")
            .filter_map(Self::parse_segment_id)
    }
}

impl fmt::Display for GroupO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "O\t{}\t{}", self.id, self.var_field)
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct GroupU {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn o_group_iter() {
        let ogroup_: GroupO = GroupO::new(
            "P1".into(),
            "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
        );
        for (name, orientation) in ogroup_.iter() {
            println!("{}{}", name, orientation);
        }
    }
}
