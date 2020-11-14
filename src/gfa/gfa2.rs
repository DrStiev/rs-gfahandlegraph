use crate::gfa::orientation::*;
use crate::gfa::segment_id::*;

use bstr::{BStr, BString, ByteSlice};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Returns an Header line
///
/// # Examples
/// ```ignore
/// let header = "VN:Z:2.0";
/// let header_ = Header {
///     version: Some("VN:Z:2.0".into()),
///     tag: "".into(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Header {
    pub version: Option<BString>,
    pub tag: BString,
}

impl Header {
    pub fn new(version: Option<BString>) -> Self {
        Header {
            version,
            tag: Default::default(),
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Header {
            version: Some("2.0".into()),
            tag: Default::default(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = &self.version {
            write!(f, "H\t{}\t{}", v, self.tag.as_bstr().to_string(),)
        } else {
            write!(f, "H\t{}", self.tag.as_bstr().to_string(),)
        }
    }
}

/// Returns a Segment line
///
/// # Examples
/// let segment = "A\t10\tAAAAAAACGT";
/// let segment_: Segment<BString> = Segment {
///     id: "A".into(),
///     len: "10".into(),
///     sequence: "AAAAAAACGT".into(),
///     tag:"".into(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Segment<N> {
    pub id: N,
    pub len: BString,
    pub sequence: BString,
    pub tag: BString,
}

impl Segment<BString> {
    pub fn new(id: &[u8], len: &[u8], sequence: &[u8]) -> Self {
        Segment {
            id: BString::from(id),
            len: BString::from(len),
            sequence: BString::from(sequence),
            tag: Default::default(),
        }
    }
}

impl<N: SegmentId> fmt::Display for Segment<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "S\t{}\t{}\t{}\t{}",
            self.id,
            self.len.as_bstr(),
            self.sequence.as_bstr(),
            self.tag.as_bstr().to_string(),
        )
    }
}

/// Returns a Fragment line
///
/// # Examples
///
/// ```ignore
/// let fragment = "15\tr1-\t10\t10\t20\t20\t*";
/// let fragment_: Fragment<BString> = Fragment {
///     id: "15".into(),
///     ext_ref: "r1-".into(),
///     sbeg: "10".into(),
///     send: "10".into(),
///     fbeg: "20".into(),
///     fend: "20".into(),
///     alignment: "*".into(),
///     tag: "".into(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Fragment<N> {
    pub id: N,
    pub ext_ref: N, // orientation as final char (+-)
    pub sbeg: BString,
    pub send: BString, // dollar character as optional final char
    pub fbeg: BString,
    pub fend: BString,
    pub alignment: BString, // alignment field can be *, trace or CIGAR
    pub tag: BString,
}

impl Fragment<BString> {
    pub fn new(
        id: &[u8],
        ext_ref: &[u8],
        sbeg: &[u8],
        send: &[u8],
        fbeg: &[u8],
        fend: &[u8],
        alignment: &[u8],
    ) -> Self {
        Fragment {
            id: BString::from(id),
            ext_ref: BString::from(ext_ref),
            sbeg: sbeg.into(),
            send: send.into(),
            fbeg: fbeg.into(),
            fend: fend.into(),
            alignment: alignment.into(),
            tag: Default::default(),
        }
    }
}

impl<N: SegmentId> fmt::Display for Fragment<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "F\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            self.ext_ref,
            self.sbeg.as_bstr(),
            self.send.as_bstr(),
            self.fbeg.as_bstr(),
            self.fend.as_bstr(),
            self.alignment.as_bstr(),
            self.tag.as_bstr().to_string(),
        )
    }
}

/// Returns an Edge line
///
/// # Examples
///
/// ```ignore
/// let edge = "*\t2+\t45+\t2531\t2591$\t0\t60\t60M";
/// let edge_: Edge<BString> = Edge {
///     id: "*".into(),
///     sid1: "2+".into(),
///     sid2: "45+".into(),
///     beg1: "2531".into(),
///     end1: "2591$".into(),
///     beg2: "0".into(),
///     end2: "60".into(),
///     alignment: "60M".into(),
///     tag: "".into(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Edge<N> {
    pub id: N,   // optional id, can be either * or id tag
    pub sid1: N, // orientation as final char (+-)
    pub sid2: N, // orientation as final char (+-)
    pub beg1: BString,
    pub end1: BString, // dollar character as optional final char
    pub beg2: BString,
    pub end2: BString,      // dollar character as optional final char
    pub alignment: BString, // alignment field can be *, trace or CIGAR
    pub tag: BString,
}

impl Edge<BString> {
    pub fn new(
        id: &[u8],
        sid1: &[u8],
        sid2: &[u8],
        beg1: &[u8],
        end1: &[u8],
        beg2: &[u8],
        end2: &[u8],
        alignment: &[u8],
    ) -> Self {
        Edge {
            id: BString::from(id),
            sid1: BString::from(sid1),
            sid2: BString::from(sid2),
            beg1: beg1.into(),
            end1: end1.into(),
            beg2: beg2.into(),
            end2: end2.into(),
            alignment: alignment.into(),
            tag: Default::default(),
        }
    }
}

impl<N: SegmentId> fmt::Display for Edge<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "E\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            self.sid1,
            self.sid2,
            self.beg1.as_bstr(),
            self.end1.as_bstr(),
            self.beg2.as_bstr(),
            self.end2.as_bstr(),
            self.alignment.as_bstr(),
            self.tag.as_bstr().to_string(),
        )
    }
}

/// Returns a Gap line
///
/// # Examples
///
/// ```ignore
/// let gap = "g1\t7+\t22+\t10\t*";
/// let gap_: Gap<BString> = Gap {
///     id: "g1".into(),
///     sid1: "7+".into(),
///     sid2: "22+".into(),
///     dist: "10".into(),
///     var: "*".into(),
///     tag: "".into(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Gap<N> {
    pub id: N,   // optional id, can be either * or id tag
    pub sid1: N, // orientation as final char (+-)
    pub sid2: N, // orientation as final char (+-)
    pub dist: BString,
    pub var: BString,
    pub tag: BString,
}

impl Gap<BString> {
    pub fn new(id: &[u8], sid1: &[u8], sid2: &[u8], dist: &[u8], var: &[u8]) -> Self {
        Gap {
            id: BString::from(id),
            sid1: BString::from(sid1),
            sid2: BString::from(sid2),
            dist: dist.into(),
            var: var.into(),
            tag: Default::default(),
        }
    }
}

impl<N: SegmentId> fmt::Display for Gap<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "G\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            self.sid1,
            self.sid2,
            self.dist.as_bstr(),
            self.var.as_bstr(),
            self.tag.as_bstr().to_string(),
        )
    }
}

/// Returns an O-Group line
///
/// # Examples
///
/// ```ignore
/// let ogroup = "P1\t36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-";
/// let ogroup_: GroupO<BString> = GroupO::new(
///     "P1".into(),
///     "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
///     "".into(),
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct GroupO<N> {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: BString,        // optional id, can be either * or id tag
    pub var_field: BString, // "array" of ref (from 1 to n)
    pub tag: BString,
    _segment_names: std::marker::PhantomData<N>,
}

impl<N: SegmentId> GroupO<N> {
    pub fn new(id: BString, var_field: BString, tag: BString) -> Self {
        GroupO {
            id,
            var_field,
            tag,
            _segment_names: std::marker::PhantomData,
        }
    }
}

impl<N: SegmentId> GroupO<N> {
    /// parses (and copies) a segment ID in the group segment list
    fn parse_segment_id(input: &[u8]) -> Option<(N, Orientation)> {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Group O segment did not include orientation"),
        };
        let seg = &input[..last];
        let id = N::parse_id(seg)?;
        Some((id, orient))
    }
}

impl GroupO<usize> {
    /// Produces an iterator over the usize segments of the given group
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, Orientation)> + 'a {
        self.var_field
            .split_str(b" ")
            .filter_map(Self::parse_segment_id)
    }
}

impl GroupO<BString> {
    /// Produces an iterator over the segments of the given group,
    /// parsing the orientation and producing a slice to each segment
    /// name
    pub fn iter(&self) -> impl Iterator<Item = (&'_ BStr, Orientation)> {
        self.var_field.split_str(b" ").map(Self::segment_id_ref)
    }

    fn segment_id_ref(input: &[u8]) -> (&'_ BStr, Orientation) {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Group O segment did not include orientation"),
        };
        let seg = &input[..last];
        (seg.as_ref(), orient)
    }
}

impl<N: SegmentId> fmt::Display for GroupO<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "O\t{}\t{}\t{}",
            self.id,
            self.var_field.as_bstr().to_string(),
            self.tag.as_bstr().to_string(),
        )
    }
}

/// Returns an U-Group line
///
/// # Examples
///
/// ```ignore
/// let ugroup = "SG1\t16 24 SG2 51_24 16_24";
/// let ugroup_: GroupU<BString> = GroupU::new(
///     "SG1".into(),
///     "16 24 SG2 51_24 16_24".into(),
///     "".into(),
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct GroupU<N> {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: BString,        // optional id, can be either * or id tag
    pub var_field: BString, // "array" of id (from 1 to n)
    pub tag: BString,
    _segment_names: std::marker::PhantomData<N>,
}

impl<N: SegmentId> GroupU<N> {
    pub fn new(id: BString, var_field: BString, tag: BString) -> Self {
        GroupU {
            id,
            var_field,
            tag,
            _segment_names: std::marker::PhantomData,
        }
    }
}

// U-Group do not have any orientations on the segment ids that they contained
// so I used as "deafult orientation" the Forward one ('+')
impl<N: SegmentId> GroupU<N> {
    /// parses (and copies) a segment ID in the group segment list
    fn parse_segment_id(input: &[u8]) -> Option<N> {
        let id = N::parse_opt_id(input)?;
        Some(id)
    }
}

impl GroupU<usize> {
    /// Produces an iterator over the usize segments of the given group
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.var_field
            .split_str(b" ")
            .filter_map(Self::parse_segment_id)
    }
}

// U-Group do not have any orientations on the segment ids that they contained
// so I used as "deafult orientation" the Forward one ('+')
impl GroupU<BString> {
    /// Produces an iterator over the segments of the given group,
    /// parsing the orientation and producing a slice to each segment
    /// name
    pub fn iter(&self) -> impl Iterator<Item = &'_ BStr> {
        self.var_field.split_str(b" ").map(Self::segment_id_ref)
    }

    fn segment_id_ref(input: &[u8]) -> &'_ BStr {
        input.as_ref()
    }
}

impl<N: SegmentId> fmt::Display for GroupU<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "U\t{}\t{}\t{}",
            self.id,
            self.var_field.as_bstr().to_string(),
            self.tag.as_bstr().to_string(),
        )
    }
}

/// Returns a GFA2 object
///
/// # Examples
///
/// ```ignore
/// let gfa2: GFA2<BString> = GFA2 {
///     headers: vec![
///         Header::new(Some("VN:Z:2.0".into())),
///     ],
///     segments: vec![
///         Segment::new(b"A", b"10", b"AAAAAAACGT"),
///     ],
///     fragments: vec![
///         Fragment::new(b"15", b"r1-", b"10", b"10", b"20", b"20", b"*"),
///     ],
///     edges: vec![
///         Edge::new(b"*", b"2+", b"45+", b"2531", b"2591$", b"0", b"60", b"60M"),
///     ],
///     gaps: vec![
///         Gap::new(b"g1", b"7+", b"22+", b"10", b"*"),
///     ],
///     groups_o: vec![
///         GroupO::new(b"P1", b"36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-", vec![]),
///     ],
///     groups_u: vec![
///         GroupU::new(b"SG1", b"16 24 SG2 51_24 16_24", vec![]),
///     ]
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GFA2<N> {
    // OptFields is used to encode the <tag>* item
    // struct to hold the results of parsing a file; not actually a graph
    pub headers: Vec<Header>,
    pub segments: Vec<Segment<N>>,
    pub fragments: Vec<Fragment<N>>,
    pub edges: Vec<Edge<N>>,
    pub gaps: Vec<Gap<N>>,
    pub groups_o: Vec<GroupO<N>>,
    pub groups_u: Vec<GroupU<N>>,
}

/// Enum containing the different kinds of GFA2 lines.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Line<N> {
    Header(Header),
    Segment(Segment<N>),
    Fragment(Fragment<N>),
    Edge(Edge<N>),
    Gap(Gap<N>),
    GroupO(GroupO<N>),
    GroupU(GroupU<N>),
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
some_line_fn!(some_fragment, Fragment<N>, Line::Fragment);
some_line_fn!(some_edge, Edge<N>, Line::Edge);
some_line_fn!(some_gap, Gap<N>, Line::Gap);
some_line_fn!(some_ogroup, GroupO<N>, Line::GroupO);
some_line_fn!(some_ugroup, GroupU<N>, Line::GroupU);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LineRef<'a, N> {
    Header(&'a Header),
    Segment(&'a Segment<N>),
    Fragment(&'a Fragment<N>),
    Edge(&'a Edge<N>),
    Gap(&'a Gap<N>),
    GroupO(&'a GroupO<N>),
    GroupU(&'a GroupU<N>),
}

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
some_line_ref_fn!(some_fragment, Fragment<N>, LineRef::Fragment);
some_line_ref_fn!(some_edge, Edge<N>, LineRef::Edge);
some_line_ref_fn!(some_gap, Gap<N>, LineRef::Gap);
some_line_ref_fn!(some_ogroup, GroupO<N>, LineRef::GroupO);
some_line_ref_fn!(some_ugroup, GroupU<N>, LineRef::GroupU);

/// Insert a GFA line (wrapped in the Line enum) into an existing
/// GFA. Simply pushes it into the corresponding Vec in the GFA,
/// or replaces the header, so there's no deduplication or sorting
/// taking place.
impl<N> GFA2<N> {
    /// Insert a GFA line (wrapped in the Line enum) into an existing
    /// GFA. Simply pushes it into the corresponding Vec in the GFA,
    /// or replaces the header, so there's no deduplication or sorting
    /// taking place.
    pub fn insert_line(&mut self, line: Line<N>) {
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
    pub fn lines_into_iter(self) -> impl Iterator<Item = Line<N>> {
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
    pub fn lines_iter(&'_ self) -> impl Iterator<Item = LineRef<'_, N>> {
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

impl<N: SegmentId> GFA2<N> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<N: SegmentId> fmt::Display for GFA2<N> {
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
        let ogroup_: GroupO<BString> = GroupO::new(
            "P1".into(),
            "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
            "".into(),
        );
        for (name, orientation) in ogroup_.iter() {
            println!("{}{}", name, orientation);
        }
    }

    #[test]
    fn u_group_iter() {
        let ugroup_: GroupU<BString> =
            GroupU::new("SG1".into(), "16 24 SG2 51_24 16_24".into(), "".into());
        for name in ugroup_.iter() {
            println!("{}", name);
        }
    }

    #[test]
    fn o_group_iter_usize() {
        let ogroup_: GroupO<usize> = GroupO::new(
            "P1".into(),
            "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
            "".into(),
        );
        for (name, orientation) in ogroup_.iter() {
            println!("{}{}", name, orientation);
        }
    }

    #[test]
    fn u_group_iter_usize() {
        let ugroup_: GroupU<usize> =
            GroupU::new("SG1".into(), "16 24 SG2 51_24 16_24".into(), "".into());
        for name in ugroup_.iter() {
            println!("{}", name);
        }
    }
}
