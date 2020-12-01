/// This file provides the structure to create a GFA Object
use crate::gfa::orientation::*;
use crate::gfa::segment_id::*;

use bstr::{BString, ByteSlice};
use serde::{Deserialize, Serialize};
use std::fmt;

// see: https://github.com/GFA-spec/GFA-spec/blob/master/GFA1.md
#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct GFA {
    pub headers: Vec<Header>,
    pub segments: Vec<Segment>,
    pub links: Vec<Link>,
    pub containments: Vec<Containment>,
    pub paths: Vec<Path>,
}

impl fmt::Display for GFA {
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
            self.links
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.paths
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
        )
    }
}

/// Enum containing the different kinds of GFA lines.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub enum Line {
    Header(Header),
    Segment(Segment),
    Link(Link),
    Containment(Containment),
    Path(Path),
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
some_line_fn!(some_link, Link, Line::Link);
some_line_fn!(some_containment, Containment, Line::Containment);
some_line_fn!(some_path, Path, Line::Path);

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
some_line_ref_fn!(some_link, Link, LineRef::Link);
some_line_ref_fn!(some_containment, Containment, LineRef::Containment);
some_line_ref_fn!(some_path, Path, LineRef::Path);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LineRef<'a> {
    Header(&'a Header),
    Segment(&'a Segment),
    Link(&'a Link),
    Containment(&'a Containment),
    Path(&'a Path),
}

impl GFA {
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
            Link(s) => self.links.push(s),
            Containment(s) => self.containments.push(s),
            Path(s) => self.paths.push(s),
        }
    }

    /// Consume a GFA object to produce an iterator over all the lines
    /// contained within. The iterator first produces all segments, then
    /// links, then containments, and finally paths.
    pub fn lines_into_iter(self) -> impl Iterator<Item = Line> {
        use Line::*;
        let heads = self.headers.into_iter().map(Header);
        let segs = self.segments.into_iter().map(Segment);
        let links = self.links.into_iter().map(Link);
        let conts = self.containments.into_iter().map(Containment);
        let paths = self.paths.into_iter().map(Path);

        heads.chain(segs).chain(links).chain(conts).chain(paths)
    }

    /// Return an iterator over references to the lines in the GFA
    pub fn lines_iter(&'_ self) -> impl Iterator<Item = LineRef<'_>> {
        use LineRef::*;
        let heads = self.headers.iter().map(Header);
        let segs = self.segments.iter().map(Segment);
        let links = self.links.iter().map(Link);
        let conts = self.containments.iter().map(Containment);
        let paths = self.paths.iter().map(Path);

        heads.chain(segs).chain(links).chain(conts).chain(paths)
    }
}

impl GFA {
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
    pub name: usize,
    pub sequence: BString,
}

impl Segment {
    #[inline]
    pub fn new(name: usize, sequence: &[u8]) -> Self {
        Segment {
            name,
            sequence: BString::from(sequence),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S\t{}\t{}", self.name, self.sequence)
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct Link {
    pub from_segment: usize,
    pub from_orient: Orientation,
    pub to_segment: usize,
    pub to_orient: Orientation,
}

impl Link {
    #[inline]
    pub fn new(
        from_segment: usize,
        from_orient: Orientation,
        to_segment: usize,
        to_orient: Orientation,
    ) -> Link {
        Link {
            from_segment,
            from_orient,
            to_segment,
            to_orient,
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "L\t{}\t{}\t{}\t{}",
            self.from_segment,
            self.from_orient,
            self.to_segment,
            self.to_orient,
        )
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct Containment {}

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash,
)]
pub struct Path {
    pub path_name: BString,
    pub segment_names: BString,
}

impl Path {
    #[inline]
    pub fn new(path_name: BString, segment_names: BString) -> Self {
        Path {
            path_name,
            segment_names,
        }
    }

    /// Parses (and copies!) a segment ID in the path segment list
    #[inline]
    fn parse_segment_id(input: &[u8]) -> Option<(usize, Orientation)> {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Path segment did not include orientation"),
        };
        let seg = &input[..last];
        let id = usize::parse_id(IdType::ID(), seg)?;
        Some((id, orient))
    }

    /// Produces an iterator over the usize segments of the given
    /// path.
    #[inline]
    pub fn iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, Orientation)> + 'a {
        self.segment_names
            .split_str(b",")
            .filter_map(Self::parse_segment_id)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P\t{}\t{}", self.path_name, self.segment_names)
    }
}
