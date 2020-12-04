/// This file provides the function to parse all the fields of a GFA file
use crate::gfa::{gfa1::*, orientation::Orientation, segment_id::*};
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
    pub links: bool,
    pub containments: bool,
    pub paths: bool,
    pub tolerance: ParserTolerance,
}

impl ParserBuilder {
    /// Parse no GFA lines, useful if you only want to parse one line type.
    pub fn none() -> Self {
        ParserBuilder {
            headers: false,
            segments: false,
            links: false,
            containments: false,
            paths: false,
            tolerance: Default::default(),
        }
    }

    /// Parse all GFA lines.
    pub fn all() -> Self {
        ParserBuilder {
            headers: true,
            segments: true,
            links: true,
            containments: true,
            paths: true,
            tolerance: Default::default(),
        }
    }

    pub fn segments(&mut self, include: bool) -> &mut Self {
        self.segments = include;
        self
    }

    pub fn links(&mut self, include: bool) -> &mut Self {
        self.links = include;
        self
    }

    pub fn paths(&mut self, include: bool) -> &mut Self {
        self.paths = include;
        self
    }

    pub fn error_tolerance(&mut self, tol: ParserTolerance) -> &mut Self {
        self.tolerance = tol;
        self
    }

    pub fn ignore_errors(&mut self) -> &mut Self {
        self.tolerance = ParserTolerance::IgnoreAll;
        self
    }

    pub fn ignore_safe_errors(&mut self) -> &mut Self {
        self.tolerance = ParserTolerance::Safe;
        self
    }

    pub fn pedantic_errors(&mut self) -> &mut Self {
        self.tolerance = ParserTolerance::Pedantic;
        self
    }

    pub fn build(self) -> GFAParser {
        GFAParser {
            headers: self.headers,
            segments: self.segments,
            links: self.links,
            containments: self.containments,
            paths: self.paths,
            tolerance: self.tolerance,
        }
    }
}

/// Return a GFAParser object
/// # Examples
/// ```ignore
/// // create a parser
/// let parser: GFAParser<bstr::BString, ()> = GFAParser::new();
/// // create a gfa object to store the result of the parsing
/// let gfa: GFA<BString, ()> = parser.parse_file(&"./test/gfa1_files/lil.gfa"). unwrap();
/// ```
#[derive(Clone)]
pub struct GFAParser {
    headers: bool,
    segments: bool,
    links: bool,
    containments: bool,
    paths: bool,
    tolerance: ParserTolerance,
}

impl Default for GFAParser {
    fn default() -> Self {
        let config = ParserBuilder::all();
        config.build()
    }
}

impl GFAParser {
    /// Create a new GFAParser that will parse all four GFA line
    /// types, and use the optional fields parser and storage `T`.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn parse_gfa_line(&self, bytes: &[u8]) -> ParserResult<Line> {
        let line: &BStr = bytes.trim().as_ref();

        let mut fields = line.split_str(b"\t");
        let hdr = fields.next().ok_or(ParseError::EmptyLine)?;

        let invalid_line = |e: ParseFieldError| ParseError::invalid_line(e, bytes);

        let line = match hdr {
            // most common lines and more important ones
            b"S" => Segment::parse_line(fields).map(Segment::wrap),
            b"L" => Link::parse_line(fields).map(Link::wrap),
            b"P" => Path::parse_line(fields).map(Path::wrap),
            // less common lines and less important ones
            b"H" => Header::parse_line(fields).map(Header::wrap),
            b"C" => Containment::parse_line(fields).map(Containment::wrap),
            _ => return Err(ParseError::UnknownLineType),
        }
        .map_err(invalid_line)?;
        Ok(line)
    }

    /// Function that return a Result<
    /// [`GFA`](/gfahandlegraph/gfa/gfa1/struct.GFA.html),
    /// [`ParseError`](../error/enum.ParseError.html)> Object
    ///
    /// # Examples
    /// ```ignore
    /// let parser: GFAParser = GFAParser::new();
    /// let gfa: GFA =
    ///     parser.parse_file(&"./tests/gfa_files/data.gfa").unwrap();
    /// ```
    pub fn parse_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<GFA, ParseError> {
        use {
            bstr::io::BufReadExt,
            std::{fs::File, io::BufReader},
        };

        let file = File::open(path.as_ref())?;
        let lines = BufReader::new(file).byte_lines();
        let gfa = Mutex::new(GFA::default());
        lines.par_bridge().for_each(|line| {
            match self.parse_gfa_line(line.unwrap().as_ref()) {
                Ok(parsed) => gfa.lock().unwrap().insert_line(parsed),
                Err(err) if err.can_safely_continue(&self.tolerance) => (),
                // this line should return the error not panic, but for now it's ok
                Err(err) => panic!("{}", err),
            }
        });
        Ok(gfa.into_inner().unwrap())
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
pub const fn type_link() -> u8 {
    b'L'
}

#[inline]
pub const fn type_path() -> u8 {
    b'P'
}

#[inline]
pub const fn type_containment() -> u8 {
    b'C'
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
fn parse_orientation<I>(mut input: I) -> ParserFieldResult<Orientation>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    let next = next_field(&mut input)?;
    let parsed = Orientation::from_bytes_plus_minus(next.as_ref());
    Orientation::parse_error(parsed)
}

#[inline]
fn parse_tag(input: &[u8]) -> Option<bool> {
    lazy_static! {
        static ref RE_TAG: Regex =
            Regex::new(r"(?-u)([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*)*").unwrap();
    }
    Some(RE_TAG.is_match(input))
}

/// function that parses the version of the header tag
/// ```<header> <- {VN:Z:1.0}  <- (VN:Z:1\.0)?```
#[inline]
fn parse_header_tag<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_HEADER: Regex = Regex::new(r"(?-u)(VN:Z:1\.0)?").unwrap();
    }
    let next = next_field(input)?;
    RE_HEADER
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Version"))
}

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
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }
        Ok(Header { version })
    }
}

/// function that parses the overlap tag
/// ```<overlap> <- * | <CIGAR> <- ([0-9]+[MIDNSHPX=])+```
#[inline]
fn parse_overlap<I>(input: &mut I) -> ParserFieldResult<bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_OVERLAP: Regex = Regex::new(r"(?-u)\*|([0-9]+[MIDNSHPX=])+").unwrap();
    }
    let next = next_field(input)?;
    if RE_OVERLAP.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Overlap"))
    }
}

/// function that parses the sequence tag of the segment element
/// ```<sequence> <- * | [A-Za-z=.]+```
#[inline]
fn parse_sequence<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_SEQUENCE: Regex = Regex::new(r"(?-u)\*|[A-Za-z=.]+").unwrap();
    }
    let next = next_field(input)?;
    RE_SEQUENCE
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Sequence"))
}

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
        let name = usize::parse_next(&mut input, IdType::ID())?;
        let sequence = parse_sequence(&mut input)?;
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }
        Ok(Segment { name, sequence })
    }
}

impl Link {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::Link(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let from_segment = usize::parse_next(&mut input, IdType::ID())?;
        let from_orient = parse_orientation(&mut input)?;
        let to_segment = usize::parse_next(&mut input, IdType::ID())?;
        let to_orient = parse_orientation(&mut input)?;
        parse_overlap(&mut input)?;
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }
        Ok(Link {
            from_segment,
            from_orient,
            to_segment,
            to_orient,
        })
    }
}

#[inline]
fn parse_id<I>(input: &mut I) -> ParserFieldResult<bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_ID: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
    }
    let next = next_field(input)?;
    if RE_ID.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("ID"))
    }
}

#[inline]
fn parse_orient<I>(input: &mut I) -> ParserFieldResult<bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_ORIENTATION: Regex = Regex::new(r"(?-u)[+-]").unwrap();
    }
    let next = next_field(input)?;
    if RE_ORIENTATION.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Orientation"))
    }
}

#[inline]
fn parse_pos<I>(input: &mut I) -> ParserFieldResult<bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_POS: Regex = Regex::new(r"(?-u)[0-9]*").unwrap();
    }
    let next = next_field(input)?;
    if RE_POS.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Position"))
    }
}

impl Containment {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::Containment(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        parse_id(&mut input)?;
        parse_orient(&mut input)?;
        parse_id(&mut input)?;
        parse_orient(&mut input)?;
        parse_pos(&mut input)?;
        parse_overlap(&mut input)?;
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }

        Ok(Containment {})
    }
}

/// function that parses the overlap tag
/// ```<overlap> <- * | <CIGAR> <- [0-9]+[MIDNSHPX=](,[0-9]+[MIDNSHPX=])*```
#[inline]
fn parse_path_overlap<I>(input: &mut I) -> ParserFieldResult<bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_PATH_OVERLAP: Regex =
            Regex::new(r"(?-u)\*|[0-9]+[MIDNSHPX=](,[0-9]+[MIDNSHPX=])*").unwrap();
    }
    let next = next_field(input)?;
    if RE_PATH_OVERLAP.is_match(next.as_ref()) {
        Ok(true)
    } else {
        Err(ParseFieldError::InvalidField("Overlap"))
    }
}

/// function that parses the segment names tag
/// ```<overlap> <- * | [!-~]+(,[!-~]+)*```
#[inline]
fn parse_segment_names<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_SEGMENT_NAMES: Regex = Regex::new(r"(?-u)[!-~]+(,[!-~]+)*").unwrap();
    }
    let next = next_field(input)?;
    RE_SEGMENT_NAMES
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Segment names"))
}

impl Path {
    #[inline]
    pub fn wrap(self) -> Line {
        Line::Path(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let path_name = BString::parse_next(&mut input, IdType::ID())?;
        let segment_names = parse_segment_names(&mut input)?;

        parse_path_overlap(&mut input)?;
        for f in input.into_iter() {
            parse_tag(f.as_ref());
        }
        Ok(Path {
            path_name,
            segment_names,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Instant;

    #[test]
    #[should_panic]
    fn parse_err_file() {
        let parser = GFAParser::default();
        let _gfa = parser.parse_file("./tests/gfa2_files/big.gfa2").unwrap();
    }

    #[test]
    #[ignore]
    fn parse_big_file() {
        // Create gfa from file: Duration { seconds: 432, nanoseconds: 428425000 } (with find)
        // Create gfa from file: Duration { seconds: 423, nanoseconds: 311465600 } (with is_match)
        // Create gfa from file: Duration { seconds: 48, nanoseconds: 646661800 }(with rayon) (with is_match) (MAIN PC)
        let parser = GFAParser::default();
        let start = Instant::now();
        let _gfa2: GFA = parser
            .parse_file("./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa")
            .unwrap();
        println!("Create gfa from file: {:?}", start.elapsed());
    }

    #[test]
    fn parse_med_file() {
        // Create gfa from file: Duration { seconds: 0, nanoseconds: 271638800 } (with is_match) (MAIN PC)
        // Create gfa from file: Duration { seconds: 0, nanoseconds: 494128300 } (with is_match) (PORTABLE PC)
        // Create gfa from file: Duration { seconds: 0, nanoseconds: 47069800 } (with rayon) (with is_match) (MAIN PC)
        // Create gfa from file: Duration { seconds: 0, nanoseconds: 205899300 } (with rayon) (with is_match) (PORTABLE PC)
        let parser = GFAParser::default();
        let start = Instant::now();
        let _gfa2: GFA = parser.parse_file("./tests/big_files/test.gfa").unwrap();
        println!("Create gfa from file: {:?}", start.elapsed());
    }

    #[test]
    #[ignore]
    fn parse_big_file1() {
        // Create gfa from file: Duration { seconds: 535, nanoseconds: 662080200 } (with is_match)
        // Create gfa from file: Duration { seconds: 63, nanoseconds: 340782100 } (with rayon) (with is_match) (MAIN PC)
        let parser = GFAParser::default();
        let start = Instant::now();
        let _gfa2: GFA = parser
            .parse_file("./tests/big_files/ape-4-0.10b.gfa")
            .unwrap();
        println!("Create gfa from file: {:?}", start.elapsed());
    }

    #[test]
    fn parse_header() {
        let header = "VN:Z:1.0";
        let header_ = Header {
            version: "VN:Z:1.0".into(),
        };
        let fields = header.split_terminator('\t');
        match Header::parse_line(fields) {
            Ok(h) => assert_eq!(h, header_),
            Err(why) => println!("Error: {}", why),
        }
    }
    #[test]
    fn can_parse_segment() {
        let segment = "A\tAAAAAAACGT";
        let segment_: Segment = Segment {
            name: convert_to_usize(b"A").unwrap(),
            sequence: "AAAAAAACGT".into(),
        };

        let fields = segment.split_terminator('\t');
        match Segment::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(s) => assert_eq!(s, segment_),
        }
    }

    #[test]
    fn can_parse_link() {
        let link = "15\t-\t10\t+\t20M";
        let link_: Link = Link {
            from_segment: 15,
            from_orient: Orientation::Backward,
            to_segment: 10,
            to_orient: Orientation::Forward,
        };
        let fields = link.split_terminator('\t');
        match Link::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(l) => assert_eq!(l, link_),
        }
    }

    #[test]
    fn can_parse_containments() {
        let containment = "15\t-\t10\t+\t4\t20M";
        let containment_: Containment = Containment {};

        let fields = containment.split_terminator('\t');
        match Containment::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(c) => assert_eq!(c, containment_),
        }
    }

    #[test]
    fn can_parse_path() {
        let path = "14\t11+,12-,13+\t4M,5M";
        let path_: Path = Path {
            path_name: "14".into(),
            segment_names: "11+,12-,13+".into(),
        };

        let fields = path.split_terminator('\t');
        match Path::parse_line(fields) {
            Err(why) => println!("Error: {}", why),
            Ok(p) => assert_eq!(p, path_),
        }
    }
}
