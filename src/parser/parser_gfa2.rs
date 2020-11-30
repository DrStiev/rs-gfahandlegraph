/// This file provides the function to parse all the fields of a GFA2 file
use crate::gfa::{gfa2::*, segment_id::*};
use crate::parser::error::ParserTolerance;
use crate::parser::error::*;

use bstr::{BStr, BString, ByteSlice};
use lazy_static::lazy_static;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::bytes::Regex;
use std::sync::Mutex;

/// Builder struct for GFAParsers
#[derive(Debug, Default, Clone, Copy)]
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

    pub fn segments(&mut self, include: bool) -> &mut Self {
        self.segments = include;
        self
    }

    pub fn edges(&mut self, include: bool) -> &mut Self {
        self.edges = include;
        self
    }

    pub fn groups_o(&mut self, include: bool) -> &mut Self {
        self.groups_o = include;
        self
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

    pub fn build(self) -> GFA2Parser {
        GFA2Parser {
            headers: self.headers,
            segments: self.segments,
            fragments: self.fragments,
            edges: self.edges,
            gaps: self.gaps,
            groups_o: self.groups_o,
            groups_u: self.groups_u,
            tolerance: self.tolerance,
        }
    }
}

#[derive(Clone)]
pub struct GFA2Parser {
    headers: bool,
    segments: bool,
    fragments: bool,
    edges: bool,
    gaps: bool,
    groups_o: bool,
    groups_u: bool,
    tolerance: ParserTolerance,
}

impl Default for GFA2Parser {
    fn default() -> Self {
        let config = ParserBuilder::all();
        config.build()
    }
}

impl GFA2Parser {
    /// Create a new GFAParser that will parse all four GFA line
    /// types, and use the optional fields parser and storage `T`.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    fn parse_gfa_line(&self, bytes: &[u8]) -> ParserResult<Line> {
        let line: &BStr = bytes.trim().as_ref();

        let mut fields = line.split_str(b"\t");
        let hdr = fields.next().ok_or(ParseError::EmptyLine)?;

        let invalid_line =
            |e: ParseFieldError| ParseError::invalid_line(e, bytes);

        let line = match hdr {
            // most common lines and more important ones
            b"S" => Segment::parse_line(fields).map(Segment::wrap),
            b"E" => Edge::parse_line(fields).map(Edge::wrap),
            b"O" => GroupO::parse_line(fields).map(GroupO::wrap),
            // less common lines and less important ones
            b"H" => Header::parse_line(fields).map(Header::wrap),
            b"F" => Fragment::parse_line(fields).map(Fragment::wrap),
            b"G" => Gap::parse_line(fields).map(Gap::wrap),
            b"U" => GroupU::parse_line(fields).map(GroupU::wrap),
            _ => return Err(ParseError::UnknownLineType),
        }
        .map_err(invalid_line)?;
        Ok(line)
    }

    /// Function that return a Result<
    /// [`GFA2`](/gfahandlegraph/gfa/gfa2/struct.GFA2.html),
    /// [`ParseError`](../error/enum.ParseError.html)> Object
    ///
    /// # Examples
    /// ```ignore
    /// let parser: GFA2Parser = GFA2Parser::new();
    /// let gfa2: GFA2 =
    ///     parser.parse_file(&"./tests/gfa2_files/data.gfa").unwrap();
    ///
    /// println!("{}", gfa2);
    ///
    /// /*
    /// H       aa:i:15
    /// H       VN:Z:2.0    TS:i:15
    /// S       3       21      TGCAACGTATAGACTTGTCAC   RC:i:4  KC:i:485841 LN:i:1329
    /// E       42       1+      2+      3       8$      0       5       0,2,4  TS:i:2  zz:Z:tag    vo:J:{"labels":false}
    /// */
    ///
    /// ```
    pub fn parse_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<GFA2, ParseError> {
        use {
            bstr::io::BufReadExt,
            std::{fs::File, io::BufReader},
        };

        let file = File::open(path.as_ref())?;
        let lines = BufReader::new(file).byte_lines();

        let gfa2 = Mutex::new(GFA2::default());

        lines.par_bridge().for_each(|line| {
            match self.parse_gfa_line(line.unwrap().as_ref()) {
                Ok(parsed) => gfa2.lock().unwrap().insert_line(parsed),
                Err(err) if err.can_safely_continue(&self.tolerance) => (),
                Err(err) => panic!("Error: {}", err), // this line should return the error not panic, but for now it's ok
            }
        });
        /*
        for line in lines {
            match self.parse_gfa_line(line?.as_ref()) {
                Ok(parsed) => gfa2.insert_line(parsed),
                Err(err) if err.can_safely_continue(&self.tolerance) => (),
                Err(err) => return Err(err),
            };
        }
         */
        Ok(gfa2.into_inner().unwrap())
    }
}

#[inline]
pub const fn type_header() -> u8 {
    b'H'
}

#[inline]
pub const fn type_segment() -> u8 {
    b'S'
}

#[inline]
pub const fn type_fragment() -> u8 {
    b'F'
}

#[inline]
pub const fn type_edge() -> u8 {
    b'E'
}

#[inline]
pub const fn type_gap() -> u8 {
    b'G'
}

#[inline]
pub const fn type_ogroup() -> u8 {
    b'O'
}

#[inline]
pub const fn type_ugroup() -> u8 {
    b'U'
}

#[inline]
fn next_field<I, P>(mut input: I) -> ParserFieldResult<P>
where
    I: Iterator<Item = P>,
    P: AsRef<[u8]>,
{
    input.next().ok_or(ParseFieldError::MissingFields)
}

#[inline]
fn parse_tag(input: &[u8]) -> Option<bool> {
    lazy_static! {
        static ref RE_TAG: Regex =
            Regex::new(r"(?-u)([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*)*")
                .unwrap();
    }
    Some(RE_TAG.is_match(input))
}

/// function that parses the version of the header tag
/// ```<header> <- {VN:Z:2.0}   {TS:i:<trace space>} <- ((VN:Z:2\.0)?\t?(TS:i:[+-]?[0-9]+)?)?```
#[inline]
fn parse_header_tag<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_HEADER: Regex =
            Regex::new(r"(?-u)((VN:Z:2\.0)?\t?(TS:i:[+-]?[0-9]+)?)?").unwrap();
    }
    let next = next_field(input)?;
    RE_HEADER
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Version"))
}

/// function that parses the HEADER field
/// ```H {VN:Z:2.0} {TS:i:<trace spacing>} <tag>*```
impl Header {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::Header(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let version = parse_header_tag(&mut input)?;
        /*
        let mut tag: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        tag.pop();
         */
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }
        Ok(Header { version, /*tag*/ })
    }
}

/// function that parses the sequence tag of the segment element
/// ```<sequence> <- * | [!-~]+```
#[inline]
fn parse_sequence<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_SEQUENCE: Regex = Regex::new(r"(?-u)\*|[!-~]+").unwrap();
    }
    let next = next_field(input)?;
    RE_SEQUENCE
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Sequence"))
}

/// function that parses the slen tag of the segment element
/// ```<int> <- {-}[0-9]+```
#[inline]
fn parse_slen<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_LEN: Regex = Regex::new(r"(?-u)\-?[0-9]+").unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_LEN
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Length"))
     */
    if RE_LEN.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Length"))
    }
}

/// function that parses the SEGMENT element
/// ```<segment> <- S <sid:id> <slen:int> <sequence> <tag>*```
impl Segment {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::Segment(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = usize::parse_next(&mut input, IdType::ID())?;
        parse_slen(&mut input)?;
        //let len = parse_slen(&mut input)?;
        let sequence = parse_sequence(&mut input)?;
        /*
        let mut tag: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        tag.pop();
         */
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }
        Ok(Segment {
            id,
            //len,
            sequence,
            //tag,
        })
    }
}

/// function that parses the pos tag of the fragment element
/// ```<pos> <- {-}[0-9]+{$}```
#[inline]
fn parse_pos<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_POS: Regex = Regex::new(r"(?-u)\-?[0-9]+\$?").unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_POS
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Position"))
     */
    if RE_POS.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Position"))
    }
}

/// function that parses the alignment tag
/// ```<alignment> <- * | <trace> <- {-}[0-9]+(,{-}[0-9]+)* | <CIGAR> <- ([0-9]+[MDIP])+```
#[inline]
fn parse_alignment<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_ALIGNMENT: Regex =
            Regex::new(r"(?-u)\*|([0-9]+[MDIP])+|(\-?[0-9]+(,\-?[0-9]+)*)")
                .unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_ALIGNMENT
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Alignment"))
     */
    if RE_ALIGNMENT.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Length"))
    }
}

#[inline]
fn parse_id<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_ID: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_LEN
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Length"))
     */
    if RE_ID.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("ID"))
    }
}

#[inline]
fn parse_opt_id<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_OPTIONAL_ID: Regex =
            Regex::new(r"(?-u)[!-~]+|\*").unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_LEN
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Length"))
     */
    if RE_OPTIONAL_ID.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Optional ID"))
    }
}

#[inline]
fn parse_ref_id<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_REFERENCE_ID: Regex =
            Regex::new(r"(?-u)[!-~]+[+-]").unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_LEN
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Length"))
     */
    if RE_REFERENCE_ID.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Reference ID"))
    }
}

/// function that parses the FRAGMENT element
/// ```<fragment> <- F <sid:id> <external:ref> <sbeg:pos> <send:pos> <fbeg:pos> <fend:pos> <alignment> <tag>*```
impl Fragment {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::Fragment(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        /*
        let id = usize::parse_next(&mut input, IdType::ID())?;
        let ext_ref = usize::parse_next(&mut input, IdType::REFERENCEID())?;
        let sbeg = parse_pos(&mut input)?;
        let send = parse_pos(&mut input)?;
        let fbeg = parse_pos(&mut input)?;
        let fend = parse_pos(&mut input)?;
        let alignment = parse_alignment(&mut input)?;
        let mut tag: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        tag.pop();
         */
        parse_id(&mut input)?;
        parse_ref_id(&mut input)?;
        parse_pos(&mut input)?;
        parse_pos(&mut input)?;
        parse_pos(&mut input)?;
        parse_pos(&mut input)?;
        parse_alignment(&mut input)?;
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }

        Ok(Fragment {
            /*
            id,
            ext_ref,
            sbeg,
            send,
            fbeg,
            fend,
            alignment,
            tag,
             */
        })
    }
}

/// function that parses the EDGE element
/// ```<edge> <- E <eid:opt_id> <sid1:ref> <sid2:ref> <beg1:pos> <end1:pos> <beg2:pos> <end2:pos> <alignment> <tag>*```
impl Edge {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::Edge(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        parse_opt_id(&mut input)?;
        //let id = usize::parse_next(&mut input, IdType::OPTIONALID())?;
        let sid1 = usize::parse_next(&mut input, IdType::OPTIONALID())?;
        let sid2 = usize::parse_next(&mut input, IdType::OPTIONALID())?;
        /*
        let beg1 = parse_pos(&mut input)?;
        let end1 = parse_pos(&mut input)?;
        let beg2 = parse_pos(&mut input)?;
        let end2 = parse_pos(&mut input)?;
        let alignment = parse_alignment(&mut input)?;
        let mut tag: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        tag.pop();
         */
        parse_pos(&mut input)?;
        parse_pos(&mut input)?;
        parse_pos(&mut input)?;
        parse_pos(&mut input)?;
        parse_alignment(&mut input)?;
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }

        Ok(Edge {
            //id,
            sid1,
            sid2,
            /*
            beg1,
            end1,
            beg2,
            end2,
            alignment,
            tag,
             */
        })
    }
}

/// function that parses the (var)int tag of the gap element
/// ```<int> <- * | {-}[0-9]+```
#[inline]
fn parse_var<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_VAR: Regex = Regex::new(r"(?-u)\*|\-?[0-9]+").unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_VAR
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Variance"))
     */
    if RE_VAR.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Variance"))
    }
}

/// function that parses the GAP element
/// ```<gap> <- G <gid:opt_id> <sid1:ref> <sid2:ref> <dist:int> (* | <var:int>) <tag>*```
impl Gap {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::Gap(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        /*
        let id = usize::parse_next(&mut input, IdType::OPTIONALID())?;
        let sid1 = usize::parse_next(&mut input, IdType::REFERENCEID())?;
        let sid2 = usize::parse_next(&mut input, IdType::REFERENCEID())?;
        let dist = parse_slen(&mut input)?;
        let var = parse_var(&mut input)?;
        let mut tag: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        tag.pop();
         */
        parse_opt_id(&mut input)?;
        parse_ref_id(&mut input)?;
        parse_ref_id(&mut input)?;
        parse_slen(&mut input)?;
        parse_var(&mut input)?;
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }

        Ok(Gap {
            /*
            id,
            sid1,
            sid2,
            dist,
            var,
            tag,
             */
        })
    }
}

/// function that parses the ref tag og the o group element
/// ```<ref> <- [!-~]+[+-]```
#[inline]
fn parse_group_ref<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_GROUP_REF: Regex =
            Regex::new(r"(?-u)[!-~]+[+-]([ ][!-~]+[+-])*").unwrap();
    }
    let next = next_field(input)?;
    RE_GROUP_REF
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Reference Group ID"))
}

/// function that parses the id tag og the o group element
/// ```<id> <- [!-~]+```
#[inline]
fn parse_group_id<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_GROUP_ID: Regex =
            Regex::new(r"(?-u)[!-~]+([ ][!-~]+)*").unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_GROUP_ID
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Id Group Id"))
     */
    if RE_GROUP_ID.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Group ID"))
    }
}

/// function that parses the GROUPO element
/// ```<o_group> <- O <oid:opt_id> <ref>([ ]<ref>)* <tag>*```
impl GroupO {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::GroupO(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = BString::parse_next(&mut input, IdType::OPTIONALID())?;
        let var_field = parse_group_ref(&mut input)?;
        /*
        let mut tag: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        tag.pop();
         */
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }
        Ok(GroupO {
            id,
            var_field, /*tag*/
        })
    }
}

/// function that parses the GROUPO element
/// ```<u_group> <- U <uid:opt_id>  <id>([ ]<id>)*  <tag>*```
impl GroupU {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::GroupU(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        /*
        let id = BString::parse_next(&mut input, IdType::OPTIONALID())?;
        let var_field = parse_group_id(&mut input)?;
        let mut tag: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        tag.pop();
         */
        parse_opt_id(&mut input)?;
        parse_group_id(&mut input)?;
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }
        Ok(GroupU { /*id, var_field, tag*/ })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Instant;

    #[test]
    #[ignore]
    fn parse_big_file() {
        // Create gfa from file: Duration { seconds: 418, nanoseconds: 278731700 } (with find)
        // Create gfa from file: Duration { seconds: 398, nanoseconds: 194378000 } (with is_match)
        let parser = GFA2Parser::default();
        let start = Instant::now();
        let _gfa2: GFA2 = parser
            .parse_file("./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa2")
            .unwrap();
        println!("Create gfa from file: {:?}", start.elapsed());
    }

    #[test]
    fn parse_med_file() {
        // Create gfa from file: Duration { seconds: 0, nanoseconds: 729470500 } (with is_match) (MAIN PC)
        // Create gfa from file: Duration { seconds: 1, nanoseconds: 331284300 } (with is_match) (PORTABLE PC)
        // Create gfa from file: Duration { seconds: 0, nanoseconds: 487090400 } (with rayon) (with is_match) (PORTABLE PC)
        let parser = GFA2Parser::default();
        let start = Instant::now();
        let _gfa2: GFA2 =
            parser.parse_file("./tests/big_files/test.gfa2").unwrap();
        println!("Create gfa from file: {:?}", start.elapsed());
    }

    #[test]
    #[ignore]
    fn parse_big_file1() {
        // Create gfa from file: Duration { seconds: 462, nanoseconds: 820784800 } (with is_match)
        let parser = GFA2Parser::default();
        let start = Instant::now();
        let _gfa2: GFA2 = parser
            .parse_file("./tests/big_files/ape-4-0.10b.gfa2")
            .unwrap();
        println!("Create gfa from file: {:?}", start.elapsed());
    }

    #[test]
    fn can_parse_header() {
        let header = "VN:Z:2.0\tHD:Z:20.20\tuR:i:AAAAAAAA";
        let header_ = Header {
            version: "VN:Z:2.0".into(),
        };

        let fields = header.split_terminator('\t');
        match Header::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(h) => assert_eq!(h, header_),
        }
    }

    #[test]
    fn can_parse_segment() {
        let segment = "A\t10\tAAAAAAACGT";
        let segment_ = Segment {
            id: convert_to_usize(b"A").unwrap(),
            sequence: "AAAAAAACGT".into(),
        };

        let fields = segment.split_terminator('\t');
        match Segment::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(s) => assert_eq!(s, segment_),
        }
    }

    #[test]
    fn can_parse_fragment() {
        let fragment = "15\tr1-\t10\t10\t20\t20\t*";
        let fragment_: Fragment = Fragment {};

        let fields = fragment.split_terminator('\t');
        match Fragment::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(f) => assert_eq!(f, fragment_),
        }
    }

    #[test]
    fn can_parse_edge() {
        let edge = "*\t2+\t45+\t2531\t2591$\t0\t60\t60M";
        let edge_: Edge = Edge {
            sid1: convert_to_usize(b"2+").unwrap(),
            sid2: convert_to_usize(b"45+").unwrap(),
        };

        let fields = edge.split_terminator('\t');
        match Edge::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(e) => assert_eq!(e, edge_),
        }
    }

    #[test]
    fn can_parse_gap() {
        let gap = "g1\t7+\t22+\t10\t*";
        let gap_: Gap = Gap {};

        let fields = gap.split_terminator('\t');
        match Gap::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(g) => assert_eq!(g, gap_),
        }
    }

    #[test]
    fn can_parse_ogroup() {
        let ogroup =
            "P1\t36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-\tAZ:i:87905\tHH:f:BAR";
        let ogroup_: GroupO = GroupO {
            id: "P1".into(),
            var_field: "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
        };

        let fields = ogroup.split_terminator('\t');
        match GroupO::parse_line(fields) {
            Err(why) => println!("Error {}", why),
            Ok(o) => assert_eq!(o, ogroup_),
        }
    }

    #[test]
    fn can_parse_ugroup() {
        let ugroup = "SG1\t16 24 SG2 51_24 16_24";
        let ugroup_: GroupU = GroupU {};

        let fields = ugroup.split_terminator('\t');
        match GroupU::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(u) => assert_eq!(u, ugroup_),
        }
    }

    /*
    #[test]
    fn blank_header() {
        let header = "";
        let header_ = Header {
            version: "".into(),
            tag: BString::from(""),
        };

        let fields = header.split_terminator('\t');
        let result = Header::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(h) => {
                assert_eq!(h, header_);
                println!("{} {}", h, header_)
            }
        }
    }

    #[test]
    fn can_parse_header() {
        let header = "VN:Z:2.0\tHD:Z:20.20\tuR:i:AAAAAAAA";
        let header_ = Header {
            version: "VN:Z:2.0".into(),
            tag: BString::from("HD:Z:20.20\tuR:i:AAAAAAAA"),
        };

        let fields = header.split_terminator('\t');
        let result = Header::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(h) => {
                assert_eq!(h, header_);
                println!("{}\n{}", h, header_)
            }
        }
    }

    #[test]
    fn can_parse_segment() {
        let segment = "A\t10\tAAAAAAACGT";
        let segment_ = Segment {
            id: convert_to_usize(b"A").unwrap(),
            len: "10".into(),
            sequence: "AAAAAAACGT".into(),
            tag: BString::from(""),
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
        let fragment_: Fragment = Fragment {
            id: 15,
            ext_ref: convert_to_usize(b"r1-").unwrap(),
            sbeg: "10".into(),
            send: "10".into(),
            fbeg: "20".into(),
            fend: "20".into(),
            alignment: "*".into(),
            tag: BString::from(""),
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
        let edge_: Edge = Edge {
            id: convert_to_usize(b"*").unwrap(),
            sid1: convert_to_usize(b"2+").unwrap(),
            sid2: convert_to_usize(b"45+").unwrap(),
            beg1: "2531".into(),
            end1: "2591$".into(),
            beg2: "0".into(),
            end2: "60".into(),
            alignment: "60M".into(),
            tag: BString::from(""),
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
        let gap_: Gap = Gap {
            id: convert_to_usize(b"g1").unwrap(),
            sid1: convert_to_usize(b"7+").unwrap(),
            sid2: convert_to_usize(b"22+").unwrap(),
            dist: "10".into(),
            var: "*".into(),
            tag: BString::from(""),
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
        let ogroup =
            "P1\t36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-\tAZ:i:87905\tHH:f:BAR";
        let ogroup_: GroupO = GroupO::new(
            "P1".into(),
            "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
            b"AZ:i:87905\tHH:f:BAR",
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
        let ugroup_: GroupU =
            GroupU::new("SG1".into(), "16 24 SG2 51_24 16_24".into(), b"");

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
    fn can_print_human_readable_file() {
        let parser = GFA2Parser::default();
        let gfa2 = parser
            .parse_file("./tests/gfa2_files/spec_q7.gfa2")
            .unwrap();
        println!("{}", gfa2);
    }
     */
}
