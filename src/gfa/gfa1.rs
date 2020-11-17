use crate::gfa::orientation::*;
use crate::gfa::segment_id::*;

use bstr::{BStr, BString, ByteSlice};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Simple representation of a parsed GFA file, using a Vec<T> to
/// store each separate GFA line type.\
/// Returns a GFA object
///
/// # Examples
/// ```ignore
/// let gfa: GFA<BString> = GFA {
///     headers: vec![
///         Header::new(Some("VN:Z:1.0".into())),
///     ],
///     segments: vec![
///         Segment::new(b"A", b"AAAAAAACGT"),
///     ],
///     links: vec![
///         Link::new(b"15", Orientation::Backward, b"10", Orientation::Forward, b"4M"),
///     ],
///     containments: vec![
///         Containmnet::new(b"1", Orientation::Backward, b"2", Orientation::Forward, b"110", b"100M"),
///     ],
///     paths: vec![
///         Path::new(b"14", b"11+,12-,13+", vec![b"4M", b"5M"]),
///     ],
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct GFA<N> {
    pub headers: Vec<Header>,
    pub segments: Vec<Segment<N>>,
    pub links: Vec<Link<N>>,
    pub containments: Vec<Containment<N>>,
    pub paths: Vec<Path<N>>,
}

impl<N: SegmentId> fmt::Display for GFA<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.headers
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.segments
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.links
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.containments
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
pub enum Line<N> {
    Header(Header),
    Segment(Segment<N>),
    Link(Link<N>),
    Containment(Containment<N>),
    Path(Path<N>),
}

macro_rules! some_line_fn {
    ($name:ident, $tgt:ty, $variant:path) => {
        impl<N> Line<N> {
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
some_line_fn!(some_segment, Segment<N>, Line::Segment);
some_line_fn!(some_link, Link<N>, Line::Link);
some_line_fn!(some_containment, Containment<N>, Line::Containment);
some_line_fn!(some_path, Path<N>, Line::Path);

macro_rules! some_line_ref_fn {
    ($name:ident, $tgt:ty, $variant:path) => {
        impl<'a, N> LineRef<'a, N> {
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
some_line_ref_fn!(some_segment, Segment<N>, LineRef::Segment);
some_line_ref_fn!(some_link, Link<N>, LineRef::Link);
some_line_ref_fn!(some_containment, Containment<N>, LineRef::Containment);
some_line_ref_fn!(some_path, Path<N>, LineRef::Path);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LineRef<'a, N> {
    Header(&'a Header),
    Segment(&'a Segment<N>),
    Link(&'a Link<N>),
    Containment(&'a Containment<N>),
    Path(&'a Path<N>),
}

impl<N> GFA<N> {
    /// Insert a GFA line (wrapped in the Line enum) into an existing
    /// GFA. Simply pushes it into the corresponding Vec in the GFA,
    /// or replaces the header, so there's no deduplication or sorting
    /// taking place.
    pub fn insert_line(&mut self, line: Line<N>) {
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
    pub fn lines_into_iter(self) -> impl Iterator<Item = Line<N>> {
        use Line::*;
        let heads = self.headers.into_iter().map(Header);
        let segs = self.segments.into_iter().map(Segment);
        let links = self.links.into_iter().map(Link);
        let conts = self.containments.into_iter().map(Containment);
        let paths = self.paths.into_iter().map(Path);

        heads.chain(segs).chain(links).chain(conts).chain(paths)
    }

    /// Return an iterator over references to the lines in the GFA
    pub fn lines_iter(&'_ self) -> impl Iterator<Item = LineRef<'_, N>> {
        use LineRef::*;
        let heads = self.headers.iter().map(Header);
        let segs = self.segments.iter().map(Segment);
        let links = self.links.iter().map(Link);
        let conts = self.containments.iter().map(Containment);
        let paths = self.paths.iter().map(Path);

        heads.chain(segs).chain(links).chain(conts).chain(paths)
    }
}

impl<N: SegmentId> GFA<N> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The header line of a GFA graph\
/// Returns an Header line
///
/// # Examples
/// ```ignore
/// let header = "VN:Z:1.0";
/// let header_ = Header {
///     version: Some("VN:Z:1.0".into()),
///     optional: "".into(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Header {
    pub version: Option<BString>,
    pub optional: BString,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            version: Some("1.0".into()),
            optional: Default::default(),
        }
    }
}

impl Header {
    pub fn new(version: Option<BString>) -> Self {
        Header {
            version,
            optional: Default::default(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = &self.version {
            write!(f, "H\t{}\t{}", v, self.optional.as_bstr().to_string(),)
        } else {
            write!(f, "H\t{}", self.optional.as_bstr().to_string(),)
        }
    }
}

/// A segment in a GFA graph.\
/// Returns a Segment line
///
/// # Examples
/// ```ignore
/// let segment = "1\tAAAAAAACGT";
/// let segment_: Segment<BString> = Segment {
///     name: "1".into(),
///     sequence: "AAAAAAACGT".into(),
///     optional: "".into(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Segment<N> {
    pub name: N,
    pub sequence: BString,
    pub optional: BString,
}

impl Segment<BString> {
    pub fn new(name: &[u8], sequence: &[u8]) -> Self {
        Segment {
            name: BString::from(name),
            sequence: BString::from(sequence),
            optional: Default::default(),
        }
    }
}

impl<N: SegmentId> fmt::Display for Segment<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "S\t{}\t{}\t{}",
            self.name,
            self.sequence.as_bstr(),
            self.optional.as_bstr().to_string(),
        )
    }
}

/// Returns a Link line
///
/// # Examples
/// ```ignore
/// let link = "15\t-\t10\t+\t20M";
/// let link_: Link<BString> = Link {
///     from_segment: "15".into(),
///     from_orient: Orientation::Backward,
///     to_segment: "10".into(),
///     to_orient: Orientation::Forward,
///     overlap: 20M
///     optional:"".into(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Link<N> {
    pub from_segment: N,
    pub from_orient: Orientation,
    pub to_segment: N,
    pub to_orient: Orientation,
    pub overlap: BString,
    pub optional: BString,
}

impl Link<BString> {
    pub fn new(
        from_segment: &[u8],
        from_orient: Orientation,
        to_segment: &[u8],
        to_orient: Orientation,
        overlap: &[u8],
    ) -> Link<BString> {
        Link {
            from_segment: from_segment.into(),
            from_orient,
            to_segment: to_segment.into(),
            to_orient,
            overlap: overlap.into(),
            optional: Default::default(),
        }
    }
}

impl<N: SegmentId> fmt::Display for Link<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "L\t{}\t{}\t{}\t{}\t{}\t{}",
            self.from_segment,
            self.from_orient,
            self.to_segment,
            self.to_orient,
            self.overlap,
            self.optional.as_bstr().to_string(),
        )
    }
}

/// Returns a Containment line
///
/// # Examples
/// ```ignore
/// let containment = "15\t-\t10\t+\t4\t20M";
/// let containment_: Containment<BString> = Containment {
///     container_name: "15".into(),
///     container_orient: Orientation::Backward,
///     contained_name: "10".into(),
///     contained_orient: Orientation::Forward,
///     pos: 4
///     overlap: 20M
///     optional: "".into(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Containment<N> {
    pub container_name: N,
    pub container_orient: Orientation,
    pub contained_name: N,
    pub contained_orient: Orientation,
    pub pos: usize,
    pub overlap: BString,
    pub optional: BString,
}

impl<N: SegmentId> fmt::Display for Containment<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "C\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.container_name,
            self.container_orient,
            self.contained_name,
            self.contained_orient,
            self.pos,
            self.overlap,
            self.optional.as_bstr().to_string(),
        )
    }
}

/// The step list that the path actually consists of is an unparsed
/// BString to keep memory down; use path.iter() to get an iterator
/// over the parsed path segments and orientations.\
/// Returns a Path line
/// # Examples
/// ```ignore
/// let path = "14\t11+,12-,13+\t4M,5M";
/// let path_: Path<BString> = Path::new(
///     "14".into(),
///     "11+,12-,13+".into(),
///     "4M,5M".into(),
///     "".into(),
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Path<N> {
    pub path_name: BString,
    pub segment_names: BString,
    pub overlaps: BString,
    pub optional: BString,
    _segment_names: std::marker::PhantomData<N>,
}

impl<N: SegmentId> Path<N> {
    pub fn new(
        path_name: BString,
        segment_names: BString,
        overlaps: BString,
        optional: BString,
    ) -> Self {
        Path {
            path_name,
            segment_names,
            overlaps,
            optional,
            _segment_names: std::marker::PhantomData,
        }
    }
}

impl<N: SegmentId> Path<N> {
    /// Parses (and copies!) a segment ID in the path segment list
    fn parse_segment_id(input: &[u8]) -> Option<(N, Orientation)> {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Path segment did not include orientation"),
        };
        let seg = &input[..last];
        let id = N::parse_id(IdType::ID(), seg)?;
        Some((id, orient))
    }
}

impl Path<BString> {
    /// Produces an iterator over the segments of the given path,
    /// parsing the orientation and producing a slice to each segment
    /// name
    pub fn iter(&self) -> impl Iterator<Item = (&'_ BStr, Orientation)> {
        self.segment_names.split_str(b",").map(Self::segment_id_ref)
    }

    fn segment_id_ref(input: &[u8]) -> (&'_ BStr, Orientation) {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Path segment did not include orientation"),
        };
        let seg = &input[..last];
        (seg.as_ref(), orient)
    }
}

impl Path<usize> {
    /// Produces an iterator over the usize segments of the given
    /// path.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, Orientation)> + 'a {
        self.segment_names
            .split_str(b",")
            .filter_map(Self::parse_segment_id)
    }
}

impl<N: SegmentId> fmt::Display for Path<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "P\t{}\t{}\t{}\t{}",
            self.path_name,
            self.segment_names.as_bstr().to_string(),
            self.overlaps.as_bstr().to_string(),
            self.optional.as_bstr().to_string(),
        )
    }
}
