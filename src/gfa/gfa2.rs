/// This file provides the structure to create a GFA2 Object
use crate::gfa::orientation::*;
use crate::gfa::segment_id::*;

use bstr::{BString, ByteSlice};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Returns an Header line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
///
/// ## Arguments
///
/// * `version` - A [`bstring`][bstring] slice.
/// * `tag` - A [`bstring`][bstring] slice.
///
/// ## Examples
/// ```ignore
/// let header = "VN:Z:2.0";
/// let header_ = Header {
///     version: "VN:Z:2.0".into(),
///     tag: b"",
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Header {
    pub version: BString,
    pub tag: BString,
}

impl Header {
    pub fn new(version: &[u8], tag: &[u8]) -> Self {
        Header {
            version: version.into(),
            tag: tag.into(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "H\t{}\t{}", self.version, self.tag,)
    }
}

/// Returns a Segment line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
///
/// ## Arguments
///
/// * `id` - An [`usize`][usize] identifier.
/// * `len` - A [`bstring`][bstring] slice.
/// * `sequence` - A [`bstring`][bstring] slice.
/// * `tag` - A [`bstring`][bstring] slice.
///
/// ## Examples
/// ```ignore
/// let segment = "A\t10\tAAAAAAACGT";
/// let segment_: Segment<BString> = Segment {
///     id: 65, // 'A' -> 65 ASCII CODE
///     len: "10".into(),
///     sequence: "AAAAAAACGT".into(),
///     tag: b"",
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Segment {
    pub id: usize,
    pub len: BString,
    pub sequence: BString,
    pub tag: BString,
}

impl Segment {
    pub fn new(id: usize, len: &[u8], sequence: &[u8], tag: &[u8]) -> Self {
        Segment {
            id,
            len: BString::from(len),
            sequence: BString::from(sequence),
            tag: BString::from(tag),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "S\t{}\t{}\t{}\t{}",
            self.id, self.len, self.sequence, self.tag,
        )
    }
}

/// Returns a Fragment line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
/// [trace]: https://dazzlerblog.wordpress.com/2015/11/05/trace-points/
///
/// ## Arguments
///
/// * `id` - An [`usize`][usize] identifier
/// * `ext_ref` - An [`usize`][usize] identifier followed by an Orientation character (43-45)
/// * `sbeg` - A [`bstring`][bstring] slice followed by an optional terminator ($)
/// * `send` - A [`bstring`][bstring] slice followed by an optional terminator ($)
/// * `fbeg` - A [`bstring`][bstring] slice followed by an optional terminator ($)
/// * `fend` - A [`bstring`][bstring] slice followed by an optional terminator ($)
/// * `alignment` - A [`bstring`][bstring] slice encoding a [`CIGAR`][cigar] or a [`trace`][trace] alignment
/// * `tag` - A [`bstring`][bstring] slice.
///
/// ## Examples
///
/// ```ignore
/// let fragment = "15\tr1-\t10\t10\t20\t20\t*";
/// let fragment_: Fragment = Fragment {
///     id: 15,
///     ext_ref: convert_to_usize(b"r1-").unwrap(),
///     sbeg: "10".into(),
///     send: "10".into(),
///     fbeg: "20".into(),
///     fend: "20".into(),
///     alignment: "*".into(),
///     tag: b"",
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Fragment {
    pub id: usize,
    pub ext_ref: usize, // orientation as final char (+-)
    pub sbeg: BString,
    pub send: BString, // dollar character as optional final char
    pub fbeg: BString,
    pub fend: BString,
    pub alignment: BString, // alignment field can be *, trace or CIGAR
    pub tag: BString,
}

impl Fragment {
    pub fn new(
        id: usize,
        ext_ref: usize,
        sbeg: &[u8],
        send: &[u8],
        fbeg: &[u8],
        fend: &[u8],
        alignment: &[u8],
        tag: &[u8],
    ) -> Self {
        Fragment {
            id,
            ext_ref,
            sbeg: sbeg.into(),
            send: send.into(),
            fbeg: fbeg.into(),
            fend: fend.into(),
            alignment: alignment.into(),
            tag: tag.into(),
        }
    }
}

impl fmt::Display for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.ext_ref.to_string().len() - 2;
        let ext_ref = self.ext_ref.to_string()[..len].to_string();
        let sgn = match self.ext_ref.to_string()[len..].to_string().as_str() {
            "43" => "+",
            "45" => "-",
            _ => panic!("Orientation not found!"),
        };

        write!(
            f,
            "F\t{}\t{}{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            ext_ref,
            sgn,
            self.sbeg,
            self.send,
            self.fbeg,
            self.fend,
            self.alignment,
            self.tag,
        )
    }
}

/// Returns an Edge line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
/// [trace]: https://dazzlerblog.wordpress.com/2015/11/05/trace-points/
///
/// ## Arguments
///
/// * `id` - An [`usize`][usize] identifier
/// * `sid1` - An [`usize`][usize] identifier followed by an Orientation character (43-45)
/// * `sid2` - An [`usize`][usize] identifier followed by an Orientation character (43-45)
/// * `beg1` - A [`bstring`][bstring] slice followed by an optional terminator ($)
/// * `end1` - A [`bstring`][bstring] slice followed by an optional terminator ($)
/// * `beg2` - A [`bstring`][bstring] slice followed by an optional terminator ($)
/// * `end2` - A [`bstring`][bstring] slice followed by an optional terminator ($)
/// * `alignment` - A [`bstring`][bstring] slice encoding a [`CIGAR`][cigar] or a [`trace`][trace] alignment
/// * `tag` - A [`bstring`][bstring] slice.
///
/// ## Examples
///
/// ```ignore
/// let edge = "*\t2+\t45+\t2531\t2591$\t0\t60\t60M";
/// let edge_: Edge = Edge {
///     id: 42, // '*' -> 42 ASCII CODE
///     sid1: 243, // '2+' -> 243 ('+' = 43 ASCII CODE)
///     sid2: 4543, // '45+' -> 4543 ('+' = 43 ASCII CODE)
///     beg1: "2531".into(),
///     end1: "2591$".into(),
///     beg2: "0".into(),
///     end2: "60".into(),
///     alignment: "60M".into(),
///     tag: b"",
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Edge {
    pub id: usize,   // optional id, can be either * or id tag
    pub sid1: usize, // orientation as final char (+-)
    pub sid2: usize, // orientation as final char (+-)
    pub beg1: BString,
    pub end1: BString, // dollar character as optional final char
    pub beg2: BString,
    pub end2: BString,      // dollar character as optional final char
    pub alignment: BString, // alignment field can be *, trace or CIGAR
    pub tag: BString,
}

impl Edge {
    pub fn new(
        id: usize,
        sid1: usize,
        sid2: usize,
        beg1: &[u8],
        end1: &[u8],
        beg2: &[u8],
        end2: &[u8],
        alignment: &[u8],
        tag: &[u8],
    ) -> Self {
        Edge {
            id,
            sid1,
            sid2,
            beg1: beg1.into(),
            end1: end1.into(),
            beg2: beg2.into(),
            end2: end2.into(),
            alignment: alignment.into(),
            tag: tag.into(),
        }
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

        write!(
            f,
            "E\t{}\t{}{}\t{}{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            sid1,
            sgn1,
            sid2,
            sgn2,
            self.beg1,
            self.end1,
            self.beg2,
            self.end2,
            self.alignment,
            self.tag,
        )
    }
}

/// Returns a Gap line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
///
/// ## Arguments
///
/// * `id` - An [`usize`][usize] identifier
/// * `sid1` - An [`usize`][usize] identifier followed by an Orientation character (43-45)
/// * `sid2` - An [`usize`][usize] identifier followed by an Orientation character (43-45)
/// * `dist` - A [`bstring`][bstring] slice.
/// * `var` - A [`bstring`][bstring] slice.
/// * `tag` - A [`bstring`][bstring] slice.
///
/// ## Examples
///
/// ```ignore
/// let gap = "g1\t7+\t22+\t10\t*";
/// let gap_: Gap = Gap {
///     id: convert_to_usize(b"g1").unwrap(),
///     sid1: 743, // '7+' -> 743 ('+' = 43 ASCII CODE)
///     sid2: 2243, // '22+' -> 2243 ('+' = 43 ASCII CODE)
///     dist: "10".into(),
///     var: "*".into(),
///     tag: b"",
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Gap {
    pub id: usize,   // optional id, can be either * or id tag
    pub sid1: usize, // orientation as final char (+-)
    pub sid2: usize, // orientation as final char (+-)
    pub dist: BString,
    pub var: BString,
    pub tag: BString,
}

impl Gap {
    pub fn new(id: usize, sid1: usize, sid2: usize, dist: &[u8], var: &[u8], tag: &[u8]) -> Self {
        Gap {
            id,
            sid1,
            sid2,
            dist: dist.into(),
            var: var.into(),
            tag: tag.into(),
        }
    }
}

impl fmt::Display for Gap {
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

        write!(
            f,
            "G\t{}\t{}{}\t{}{}\t{}\t{}\t{}",
            self.id, sid1, sgn1, sid2, sgn2, self.dist, self.var, self.tag,
        )
    }
}

/// Returns an O-Group line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
///
/// ## Arguments
///
/// * `id` - A [`bstring`][bstring] slice.
/// * `var_field` - A [`bstring`][bstring] slice.
/// * `tag` - A [`bstring`][bstring] slice.
///
/// ## Examples
///
/// ```ignore
/// let ogroup = "P1\t36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-";
/// let ogroup_: GroupO = GroupO::new(
///     "P1".into(),
///     "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
///     b"",
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct GroupO {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: BString,        // optional id, can be either * or id tag
    pub var_field: BString, // "array" of ref (from 1 to n)
    pub tag: BString,
}

impl GroupO {
    pub fn new(id: BString, var_field: BString, tag: &[u8]) -> Self {
        GroupO {
            id,
            var_field,
            tag: tag.into(),
        }
    }

    /// parses (and copies) a segment ID in the group segment list
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
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, Orientation)> + 'a {
        self.var_field
            .split_str(b" ")
            .filter_map(Self::parse_segment_id)
    }
}

impl fmt::Display for GroupO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "O\t{}\t{}\t{}", self.id, self.var_field, self.tag,)
    }
}

/// Returns an U-Group line
///
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
///
/// ## Arguments
///
/// * `id` - A [`bstring`][bstring] slice.
/// * `var_field` - A [`bstring`][bstring] slice.
/// * `tag` - A [`bstring`][bstring] slice.
///
/// ## Examples
///
/// ```ignore
/// let ugroup = "SG1\t16 24 SG2 51_24 16_24";
/// let ugroup_: GroupU = GroupU::new(
///     "SG1".into(),
///     "16 24 SG2 51_24 16_24".into(),
///     b"",
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct GroupU {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: BString,        // optional id, can be either * or id tag
    pub var_field: BString, // "array" of id (from 1 to n)
    pub tag: BString,
}

impl GroupU {
    pub fn new(id: BString, var_field: BString, tag: &[u8]) -> Self {
        GroupU {
            id,
            var_field,
            tag: tag.into(),
        }
    }

    /// parses (and copies) a segment ID in the group segment list
    fn parse_segment_id(input: &[u8]) -> Option<usize> {
        let id = usize::parse_id(IdType::OPTIONALID(), input)?;
        Some(id)
    }

    /// Produces an iterator over the usize segments of the given group
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.var_field
            .split_str(b" ")
            .filter_map(Self::parse_segment_id)
    }
}

impl fmt::Display for GroupU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U\t{}\t{}\t{}", self.id, self.var_field, self.tag,)
    }
}

/// Returns a GFA2 object
///
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [bstring]: https://docs.rs/bstr/0.2.14/bstr/struct.BString.html
/// [usize]: https://doc.rust-lang.org/std/primitive.usize.html
///
/// ## Arguments
///
/// * `headers` - A [`vector`][vec] of Header.
/// * `segments` - A [`vector`][vec] of Segment.
/// * `fragments` - A [`vector`][vec] of Fragment.
/// * `edges` - A [`vector`][vec] of Edge.
/// * `gaps` - A [`vector`][vec] of Gap.
/// * `o groups` - A [`vector`][vec] of OGroup.
/// * `u groups` - A [`vector`][vec] of UGroup.
///
/// ## Examples
///
/// ```ignore
/// let gfa2: GFA2 = GFA2 {
///     headers: vec![
///         Header::new("VN:Z:2.0".into(), b""),
///     ],
///     segments: vec![
///         Segment::new(65, b"10", b"AAAAAAACGT", b""),
///     ],
///     fragments: vec![
///         Fragment::new(15, convert_to_usize(b"r1-").unwrap(), b"10", b"10", b"20", b"20", b"*", b""),
///     ],
///     edges: vec![
///         Edge::new(42, 243, 4543, b"2531", b"2591$", b"0", b"60", b"60M", b""),
///     ],
///     gaps: vec![
///         Gap::new(convert_to_usize(b"g1").unwrap(), 743, 2243, b"10", b"*", b""),
///     ],
///     groups_o: vec![
///         GroupO::new(b"P1", b"36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-", b""),
///     ],
///     groups_u: vec![
///         GroupU::new(b"SG1", b"16 24 SG2 51_24 16_24", b""),
///     ]
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct GFA2 {
    // OptFields is used to encode the <tag>* item
    // struct to hold the results of parsing a file; not actually a graph
    pub headers: Vec<Header>,
    pub segments: Vec<Segment>,
    pub fragments: Vec<Fragment>,
    pub edges: Vec<Edge>,
    pub gaps: Vec<Gap>,
    pub groups_o: Vec<GroupO>,
    pub groups_u: Vec<GroupU>,
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
    pub fn new() -> Self {
        Default::default()
    }
}

impl fmt::Display for GFA2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}",
            self.headers
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.segments
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.fragments
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.edges
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.gaps
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.groups_o
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.groups_u
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn o_group_iter() {
        let ogroup_: GroupO =
            GroupO::new("P1".into(), "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(), b"");
        for (name, orientation) in ogroup_.iter() {
            println!("{}{}", name, orientation);
        }
    }

    #[test]
    fn u_group_iter() {
        let ugroup_: GroupU = GroupU::new("SG1".into(), "16 24 SG2 51_24 16_24".into(), b"");
        for name in ugroup_.iter() {
            println!("{}", name);
        }
    }
}
