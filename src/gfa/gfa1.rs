/// This file provides the structure to create a GFA Object
use crate::gfa::orientation::*;
use crate::gfa::segment_id::*;

use bstr::{BString, ByteSlice};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Simple representation of a parsed GFA file, using a Vec<T> to
/// store each separate GFA line type.\
/// Returns a GFA object
///
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
///
/// ## Arguments
///
/// * `headers` - A [`vector`][vec] of [`Header`](struct.Header.html).
/// * `segments` - A [`vector`][vec] of [`Segment`](struct.Segment.html).
/// * `links` - A [`vector`][vec] of [`Link`](struct.Link.html).
/// * `containments` - A [`vector`][vec] of [`Containment`](struct.Containment.html).
/// * `paths` - A [`vector`][vec] of [`Path`](struct.Path.html).
///
/// ## Examples
/// ```ignore
/// let gfa: GFA = GFA {
///     headers: vec![
///         Header::new("VN:Z:1.0".into(), b""),
///     ],
///     segments: vec![
///         Segment::new(65, b"AAAAAAACGT", b""),
///     ],
///     links: vec![
///         Link::new(15, Orientation::Backward, 10, Orientation::Forward, b"4M", b""),
///     ],
///     containments: vec![
///         Containmnet::new(1, Orientation::Backward, 2, Orientation::Forward, b"110", b"100M", b""),
///     ],
///     paths: vec![
///         Path::new(b"14", b"11+,12-,13+", vec![b"4M", b"5M"], b""),
///     ],
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
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
    pub fn new() -> Self {
        Default::default()
    }
}

/// Returns an Header line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
///
/// ## Arguments
///
/// * `version` - A [`bstring`][bstring] slice.
/// * `optional field` - A [`bstring`][bstring] slice.
///
/// ## Examples
/// ```ignore
/// let header = "VN:Z:1.0";
/// let header_ = Header {
///     version: "VN:Z:1.0".into(),
///     optional: b"",
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Header {
    pub version: BString,
    pub optional: BString,
}

impl Header {
    pub fn new(version: &[u8], optional: &[u8]) -> Self {
        Header {
            version: version.into(),
            optional: optional.into(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "H\t{}\t{}", self.version, self.optional,)
    }
}

/// Returns a Segment line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
///
/// ## Arguments
///
/// * `name` - An [`usize`][usize] identifier
/// * `sequence` - A [`bstring`][bstring] slice.
/// * `optional field` - A [`bstring`][bstring] slice.
///
/// ## Examples
/// ```ignore
/// let segment = "1\tAAAAAAACGT";
/// let segment_: Segment = Segment {
///     name: 1,
///     sequence: "AAAAAAACGT".into(),
///     optional: b"",
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Segment {
    pub name: usize,
    pub sequence: BString,
    pub optional: BString,
}

impl Segment {
    pub fn new(name: usize, sequence: &[u8], optional: &[u8]) -> Self {
        Segment {
            name,
            sequence: BString::from(sequence),
            optional: optional.into(),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "S\t{}\t{}\t{}",
            self.name,
            self.sequence,
            self.optional
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Link line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
///
/// ## Arguments
///
/// * `from_segment` - An [`usize`][usize] identifier
/// * `from_orient` - An orientation identifier, it can be Forward or Backward (+-)
/// * `to_segment` - An [`usize`][usize] identifier
/// * `to_orient` - An orientation identifier, it can be Forward or Backward (+-)
/// * `overlap` - A [`bstring`][bstring] slice encoding a [`CIGAR`][cigar] alignment
/// * `optional field` - A [`bstring`][bstring] slice.
///
/// ## Examples
/// ```ignore
/// let link = "15\t-\t10\t+\t20M";
/// let link_: Link = Link {
///     from_segment: 15,
///     from_orient: Orientation::Backward,
///     to_segment: 10,
///     to_orient: Orientation::Forward,
///     overlap: 20M
///     optional: b"",
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Link {
    pub from_segment: usize,
    pub from_orient: Orientation,
    pub to_segment: usize,
    pub to_orient: Orientation,
    pub overlap: BString,
    pub optional: BString,
}

impl Link {
    pub fn new(
        from_segment: usize,
        from_orient: Orientation,
        to_segment: usize,
        to_orient: Orientation,
        overlap: &[u8],
        optional: &[u8],
    ) -> Link {
        Link {
            from_segment,
            from_orient,
            to_segment,
            to_orient,
            overlap: overlap.into(),
            optional: optional.into(),
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "L\t{}\t{}\t{}\t{}\t{}\t{}",
            self.from_segment,
            self.from_orient,
            self.to_segment,
            self.to_orient,
            self.overlap,
            self.optional,
        )
    }
}

/// Returns a Containment line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
///
/// ## Arguments
///
/// * `container_name` - An [`usize`][usize] identifier
/// * `container_orient` - An orientation identifier, it can be Forward or Backward (+-)
/// * `contained_name` - An [`usize`][usize] identifier
/// * `contained_orient` - An orientation identifier, it can be Forward or Backward (+-)
/// * `pos` - An [`usize`][usize] identifier
/// * `overlap` - A [`bstring`][bstring] slice encoding a [`CIGAR`][cigar] alignment
/// * `optional field` - A [`bstring`][bstring] slice.
///
/// ## Examples
/// ```ignore
/// let containment = "15\t-\t10\t+\t4\t20M";
/// let containment_: Containment = Containment {
///     container_name: 15,
///     container_orient: Orientation::Backward,
///     contained_name: 10,
///     contained_orient: Orientation::Forward,
///     pos: 4
///     overlap: 20M
///     optional: b"",
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Containment {
    pub container_name: usize,
    pub container_orient: Orientation,
    pub contained_name: usize,
    pub contained_orient: Orientation,
    pub pos: usize,
    pub overlap: BString,
    pub optional: BString,
}

impl Containment {
    pub fn new(
        container_name: usize,
        container_orient: Orientation,
        contained_name: usize,
        contained_orient: Orientation,
        pos: usize,
        overlap: &[u8],
        optional: &[u8],
    ) -> Containment {
        Containment {
            container_name,
            container_orient,
            contained_name,
            contained_orient,
            pos,
            overlap: overlap.into(),
            optional: optional.into(),
        }
    }
}

impl fmt::Display for Containment {
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
            self.optional,
        )
    }
}

/// Returns a Path line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
///
/// ## Arguments
///
/// * `path_name` - A [`bstring`][bstring] identifier
/// * `segment_names` - A [`bstring`][bstring] identifier
/// * `overlap` - A [`bstring`][bstring] slice encoding a [`CIGAR`][cigar] alignment
/// * `optional field` - A [`bstring`][bstring] slice.
///
/// ## Examples
/// ```ignore
/// let path = "14\t11+,12-,13+\t4M,5M";
/// let path_: Path = Path {
///     path_name: "14".into(),
///     segment_names: "11+,12-,13+".into(),
///     overlaps: "4M,5M".into(),
///     optional: "".into(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Path {
    pub path_name: BString,
    pub segment_names: BString,
    pub overlaps: BString,
    pub optional: BString,
}

impl Path {
    pub fn new(
        path_name: BString,
        segment_names: BString,
        overlaps: BString,
        optional: &[u8],
    ) -> Self {
        Path {
            path_name,
            segment_names,
            overlaps,
            optional: optional.into(),
        }
    }

    /// Parses (and copies!) a segment ID in the path segment list
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
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, Orientation)> + 'a {
        self.segment_names
            .split_str(b",")
            .filter_map(Self::parse_segment_id)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "P\t{}\t{}\t{}\t{}",
            self.path_name,
            self.segment_names,
            self.overlaps,
            self.optional
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}
