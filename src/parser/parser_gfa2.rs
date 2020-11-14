use crate::gfa::{gfa2::*, segment_id::SegmentId};
use crate::parser::error::*;

use bstr::BString;
use lazy_static::lazy_static;
use regex::bytes::Regex;

fn next_field<I, P>(mut input: I) -> ParserFieldResult<P>
where
    I: Iterator<Item = P>,
    P: AsRef<[u8]>,
{
    input.next().ok_or(ParseFieldError::MissingFields)
}

/// function that parses the version of the header tag
/// ```<header> <- {VN:Z:2.0}   {TS:i:<trace space>} <- ([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[+-]?[0-9]+\.?[0-9]+)?```
fn parse_header_tag<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?-u)([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[+-]?[0-9]+\.?[0-9]+)?").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Version"))
}

/// function that parses the tag element
/// ```<tag> <- [A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*```
fn parse_tag<I>(input: &mut I) -> Option<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?-u)([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*)*").unwrap();
    }

    RE.find(input.next()?.as_ref())
        .map(|s| BString::from(s.as_bytes()))
}

/// function that parses the HEADER field
/// ```H {VN:Z:2.0} {TS:i:<trace spacing>} <tag>*```
impl Header {
    #[inline]
    pub fn wrap<N: SegmentId>(self) -> Line<N> {
        Line::Header(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let version = Some(parse_header_tag(&mut input)?);
        let tag = parse_tag(&mut input).unwrap_or_else(|| BString::from(""));

        Ok(Header { version, tag })
    }
}

/// function that parses the sequence tag of the segment element
/// ```<sequence> <- * | [!-~]+```
fn parse_sequence<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\*|[!-~]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Sequence"))
}

/// function that parses the slen tag of the segment element
/// ```<int> <- {-}[0-9]+```
fn parse_slen<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\-?[0-9]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Lenght"))
}

/// function that parses the SEGMENT element
/// ```<segment> <- S <sid:id> <slen:int> <sequence> <tag>*```
impl<N: SegmentId> Segment<N> {
    #[inline]
    pub fn wrap(self) -> Line<N> {
        Line::Segment(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = N::parse_next(&mut input)?;
        let len = parse_slen(&mut input)?;
        let sequence = parse_sequence(&mut input)?;
        let tag = parse_tag(&mut input).unwrap_or_else(|| BString::from(""));
        Ok(Segment {
            id,
            len,
            sequence,
            tag,
        })
    }
}

/// function that parses the pos tag of the fragment element
/// ```<pos> <- {-}[0-9]+{$}```
fn parse_pos<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\-?[0-9]+\$?").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Position"))
}

/// function that parses the alignment tag
/// ```<alignment> <- * | <trace> <- {-}[0-9]+(,{-}[0-9]+)* | <CIGAR> <- ([0-9]+[MDIP])+```
fn parse_alignment<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?-u)\*|([0-9]+[MDIP])+|(\-?[0-9]+(,\-?[0-9]+)*)").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Alignment"))
}

/// function that parses the FRAGMENT element
/// ```<fragment> <- F <sid:id> <external:ref> <sbeg:pos> <send:pos> <fbeg:pos> <fend:pos> <alignment> <tag>*```
impl<N: SegmentId> Fragment<N> {
    #[inline]
    pub fn wrap(self) -> Line<N> {
        Line::Fragment(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = N::parse_next(&mut input)?;
        let ext_ref = N::parse_next_ref(&mut input)?;
        let sbeg = parse_pos(&mut input)?;
        let send = parse_pos(&mut input)?;
        let fbeg = parse_pos(&mut input)?;
        let fend = parse_pos(&mut input)?;
        let alignment = parse_alignment(&mut input)?;
        let tag = parse_tag(&mut input).unwrap_or_else(|| BString::from(""));
        Ok(Fragment {
            id,
            ext_ref,
            sbeg,
            send,
            fbeg,
            fend,
            alignment,
            tag,
        })
    }
}

/// function that parses the EDGE element
/// ```<edge> <- E <eid:opt_id> <sid1:ref> <sid2:ref> <beg1:pos> <end1:pos> <beg2:pos> <end2:pos> <alignment> <tag>*```
impl<N: SegmentId> Edge<N> {
    #[inline]
    pub fn wrap(self) -> Line<N> {
        Line::Edge(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = N::parse_next_opt(&mut input)?;
        let sid1 = N::parse_next_ref(&mut input)?;
        let sid2 = N::parse_next_ref(&mut input)?;
        let beg1 = parse_pos(&mut input)?;
        let end1 = parse_pos(&mut input)?;
        let beg2 = parse_pos(&mut input)?;
        let end2 = parse_pos(&mut input)?;
        let alignment = parse_alignment(&mut input)?;
        let tag = parse_tag(&mut input).unwrap_or_else(|| BString::from(""));
        Ok(Edge {
            id,
            sid1,
            sid2,
            beg1,
            end1,
            beg2,
            end2,
            alignment,
            tag,
        })
    }
}

/// function that parses the (dist)int tag of the gap element
/// ```<int> <- {-}[0-9]+```
fn parse_dist<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\-?[0-9]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Distance"))
}

/// function that parses the (var)int tag of the gap element
/// ```<int> <- {-}[0-9]+```
fn parse_var<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\*|\-?[0-9]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Variance"))
}

/// function that parses the GAP element
/// ```<gap> <- G <gid:opt_id> <sid1:ref> <sid2:ref> <dist:int> (* | <var:int>) <tag>*```
impl<N: SegmentId> Gap<N> {
    #[inline]
    pub fn wrap(self) -> Line<N> {
        Line::Gap(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = N::parse_next_opt(&mut input)?;
        let sid1 = N::parse_next_ref(&mut input)?;
        let sid2 = N::parse_next_ref(&mut input)?;
        let dist = parse_dist(&mut input)?;
        let var = parse_var(&mut input)?;
        let tag = parse_tag(&mut input).unwrap_or_else(|| BString::from(""));
        Ok(Gap {
            id,
            sid1,
            sid2,
            dist,
            var,
            tag,
        })
    }
}

/// function that parses the ref tag og the o group element
/// ```<ref> <- [!-~]+[+-]```
fn parse_group_ref<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)[!-~]+[+-]([ ][!-~]+[+-])*").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Reference Group Id"))
}

/// function that parses the id tag og the o group element
/// ```<id> <- [!-~]+```
fn parse_group_id<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)[!-~]+([ ][!-~]+)*").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Id Group Id"))
}

/// function that parses the optional id tag of the o group element
/// ```<id> <- *|[!-~]+```
fn parse_optional_id<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\*|[!-~]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Optional Id"))
}

/// function that parses the GROUPO element
/// ```<o_group> <- O <oid:opt_id> <ref>([ ]<ref>)* <tag>*```
impl<N: SegmentId> GroupO<N> {
    #[inline]
    pub fn wrap(self) -> Line<N> {
        Line::GroupO(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = parse_optional_id(&mut input)?;
        let var_field = parse_group_ref(&mut input)?;
        let tag = parse_tag(&mut input).unwrap_or_else(|| BString::from(""));
        Ok(GroupO::new(id, var_field, tag))
    }
}

/// function that parses the GROUPO element
/// ```<u_group> <- U <uid:opt_id>  <id>([ ]<id>)*  <tag>*```
impl<N: SegmentId> GroupU<N> {
    #[inline]
    pub fn wrap(self) -> Line<N> {
        Line::GroupU(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = parse_optional_id(&mut input)?;
        let var_field = parse_group_id(&mut input)?;
        let tag = parse_tag(&mut input).unwrap_or_else(|| BString::from(""));
        Ok(GroupU::new(id, var_field, tag))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_header() {
        let header = "";
        let header_ = Header {
            version: Some("".into()),
            tag: "".into(),
        };

        let result: ParserFieldResult<Header> = Header::parse_line([header].iter());

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(h) => assert_eq!(h, header_),
        }
    }

    #[test]
    fn can_parse_header() {
        let header = "VN:Z:2.0";
        let header_ = Header {
            version: Some("VN:Z:2.0".into()),
            tag: "".into(),
        };

        let result: ParserFieldResult<Header> = Header::parse_line([header].iter());

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(h) => assert_eq!(h, header_),
        }
    }

    #[test]
    fn can_parse_segment() {
        let segment = "A\t10\tAAAAAAACGT";
        let segment_: Segment<BString> = Segment {
            id: "A".into(),
            len: "10".into(),
            sequence: "AAAAAAACGT".into(),
            tag: "".into(),
        };

        let fields = segment.split_terminator('\t');
        let result = Segment::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(s) => assert_eq!(s, segment_),
        }
    }

    #[test]
    fn can_parse_fragment() {
        let fragment = "15\tr1-\t10\t10\t20\t20\t*";
        let fragment_: Fragment<BString> = Fragment {
            id: "15".into(),
            ext_ref: "r1-".into(),
            sbeg: "10".into(),
            send: "10".into(),
            fbeg: "20".into(),
            fend: "20".into(),
            alignment: "*".into(),
            tag: "".into(),
        };

        let fields = fragment.split_terminator('\t');
        let result = Fragment::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(f) => assert_eq!(f, fragment_),
        }
    }

    #[test]
    fn can_parse_edge() {
        let edge = "*\t2+\t45+\t2531\t2591$\t0\t60\t60M";
        let edge_: Edge<BString> = Edge {
            id: "*".into(),
            sid1: "2+".into(),
            sid2: "45+".into(),
            beg1: "2531".into(),
            end1: "2591$".into(),
            beg2: "0".into(),
            end2: "60".into(),
            alignment: "60M".into(),
            tag: "".into(),
        };

        let fields = edge.split_terminator('\t');
        let result = Edge::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(e) => assert_eq!(e, edge_),
        }
    }

    #[test]
    fn can_parse_gap() {
        let gap = "g1\t7+\t22+\t10\t*";
        let gap_: Gap<BString> = Gap {
            id: "g1".into(),
            sid1: "7+".into(),
            sid2: "22+".into(),
            dist: "10".into(),
            var: "*".into(),
            tag: "".into(),
        };

        let fields = gap.split_terminator('\t');
        let result = Gap::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(g) => assert_eq!(g, gap_),
        }
    }

    #[test]
    fn can_parse_ogroup() {
        let ogroup = "P1\t36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-";
        let ogroup_: GroupO<BString> = GroupO::new(
            "P1".into(),
            "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
            "".into(),
        );

        let fields = ogroup.split_terminator('\t');
        let result = GroupO::parse_line(fields);

        match result {
            Err(why) => println!("Error {}", why),
            Ok(o) => {
                println!("{}", o);
                assert_eq!(o, ogroup_)
            }
        }
    }

    #[test]
    fn can_parse_ugroup() {
        let ugroup = "SG1\t16 24 SG2 51_24 16_24";
        let ugroup_: GroupU<BString> =
            GroupU::new("SG1".into(), "16 24 SG2 51_24 16_24".into(), "".into());

        let fields = ugroup.split_terminator('\t');
        let result = GroupU::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(u) => {
                println!("{}", u);
                assert_eq!(u, ugroup_)
            }
        }
    }

    #[test]
    fn can_parse_alignment_cigar() {
        let cigar = vec!["1M1I1M1I2M"];
        let result = parse_alignment(&mut cigar.iter());

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(u) => {
                assert_eq!(
                    cigar
                        .iter()
                        .fold(String::new(), |acc, str| acc + &str.to_string()),
                    u
                );
                println!("{}", u);
            }
        }
    }

    #[test]
    fn can_parse_alignment_trace() {
        let trace = vec!["0,2,4"];
        let result = parse_alignment(&mut trace.iter());

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(u) => {
                assert_eq!(
                    trace
                        .iter()
                        .fold(String::new(), |acc, str| acc + &str.to_string()),
                    u
                );
                println!("{}", u);
            }
        }
    }

    #[test]
    fn can_parse_no_alignment() {
        let no_aligment = vec!["*"];
        let result = parse_alignment(&mut no_aligment.iter());

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(u) => {
                assert_eq!(
                    no_aligment
                        .iter()
                        .fold(String::new(), |acc, str| acc + &str.to_string()),
                    u
                );
                println!("{}", u);
            }
        }
    }

    #[test]
    fn can_parse_error_alignment() {
        // this should return an error message (and it does)
        let error = vec!["ERROR"];
        let result = parse_alignment(&mut error.iter());

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(u) => {
                assert_eq!(
                    error
                        .iter()
                        .fold(String::new(), |acc, str| acc + &str.to_string()),
                    u
                );
                println!("{}", u);
            }
        }
    }

    #[test]
    fn can_parse_single_tag() {
        let tag = vec!["aa:Z:test"];
        let result = parse_tag(&mut tag.iter());

        match result {
            None => (),
            Some(t) => {
                assert_eq!(
                    tag.iter()
                        .fold(String::new(), |acc, str| acc + &str.to_string()),
                    t
                );
                println!("{}", t);
            }
        }
    }

    #[test]
    fn can_parse_multiple_tag() {
        let tag = vec!["aa:Z:test   hr:i:2020"];
        let result = parse_tag(&mut tag.iter());

        match result {
            None => (),
            Some(t) => {
                assert_eq!(
                    tag.iter()
                        .fold(String::new(), |acc, str| acc + &str.to_string()),
                    t
                );
                println!("{}", t);
            }
        }
    }
}
