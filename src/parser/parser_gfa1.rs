/// This file provides the function to parse all the fields of a GFA file
use crate::gfa::{gfa1::*, orientation::Orientation, segment_id::*};
use crate::parser::error::*;

use bstr::{BStr, BString, ByteSlice};
use lazy_static::lazy_static;
use regex::bytes::Regex;

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

        let invalid_line =
            |e: ParseFieldError| ParseError::invalid_line(e, bytes);

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
    ///
    /// println!("{}", gfa);
    ///
    /// /*
    /// H	VN:Z:1.0
    /// S	11	ACCTT
    /// S	12	TCAAGG
    /// S	13	CTTGATT
    /// L	11	+	12	-	4M
    /// L	12	-	13	+	5M
    /// L	11	+	13	+	3M
    /// P	14	11+,12-,13+	4M,5M
    /// */
    ///
    /// ```
    pub fn parse_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<GFA, ParseError> {
        use {
            bstr::io::BufReadExt,
            std::{fs::File, io::BufReader},
        };

        let file = File::open(path.as_ref())?;
        let lines = BufReader::new(file).byte_lines();
        let mut gfa = GFA::default();

        for line in lines {
            match self.parse_gfa_line(line?.as_ref()) {
                Ok(parsed) => gfa.insert_line(parsed),
                Err(err) if err.can_safely_continue(&self.tolerance) => (),
                Err(err) => return Err(err),
            };
        }

        Ok(gfa)
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
            Regex::new(r"(?-u)([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*)*")
                .unwrap();
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
        /*
        let mut optional: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        optional.pop();
         */
        input.into_iter().filter_map(|f| parse_tag(f.as_ref()));
        Ok(Header {
            version, /*optional*/
        })
    }
}

/// function that parses the overlap tag
/// ```<overlap> <- * | <CIGAR> <- ([0-9]+[MIDNSHPX=])+```
#[inline]
fn parse_overlap<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_OVERLAP: Regex =
            Regex::new(r"(?-u)\*|([0-9]+[MIDNSHPX=])+").unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_OVERLAP
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Overlap"))
     */
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
        static ref RE_SEQUENCE: Regex =
            Regex::new(r"(?-u)\*|[A-Za-z=.]+").unwrap();
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
        /*
        let mut optional: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        optional.pop();
         */
        input.into_iter().filter_map(|f| parse_tag(f.as_ref()));
        Ok(Segment {
            name,
            sequence,
            //optional,
        })
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
        /*
        let overlap = parse_overlap(&mut input)?;
        let mut optional: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        optional.pop();
         */
        parse_overlap(&mut input)?;
        input.into_iter().filter_map(|f| parse_tag(f.as_ref()));
        Ok(Link {
            from_segment,
            from_orient,
            to_segment,
            to_orient,
            //overlap,
            //optional,
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
        /*
        let container_name = usize::parse_next(&mut input, IdType::ID())?;
        let container_orient = parse_orientation(&mut input)?;
        let contained_name = usize::parse_next(&mut input, IdType::ID())?;
        let contained_orient = parse_orientation(&mut input)?;
        let pos = next_field(&mut input)?;
        let pos = pos.as_ref().to_str()?.parse()?;
        let overlap = parse_overlap(&mut input)?;
        let mut optional: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        optional.pop();
         */
        parse_id(&mut input)?;
        parse_orient(&mut input)?;
        parse_id(&mut input)?;
        parse_orient(&mut input)?;
        parse_pos(&mut input)?;
        parse_overlap(&mut input)?;
        input.into_iter().filter_map(|f| parse_tag(f.as_ref()));

        Ok(Containment {
            /*
            container_name,
            container_orient,
            contained_name,
            contained_orient,
            overlap,
            pos,
            optional,
             */
        })
    }
}

/// function that parses the overlap tag
/// ```<overlap> <- * | <CIGAR> <- [0-9]+[MIDNSHPX=](,[0-9]+[MIDNSHPX=])*```
#[inline]
fn parse_path_overlap<I>(input: &mut I) -> ParserFieldResult</*BString*/ bool>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE_PATH_OVERLAP: Regex =
            Regex::new(r"(?-u)\*|[0-9]+[MIDNSHPX=](,[0-9]+[MIDNSHPX=])*")
                .unwrap();
    }
    let next = next_field(input)?;
    /*
    RE_PATH_OVERLAP
        .find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Overlap"))
     */
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
        static ref RE_SEGMENT_NAMES: Regex =
            Regex::new(r"(?-u)[!-~]+(,[!-~]+)*").unwrap();
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
        // Use the SegmentId parser for the path name as well; it's
        // just always BString
        let path_name = BString::parse_next(&mut input, IdType::ID())?;
        let segment_names = parse_segment_names(&mut input)?;
        /*
        let overlaps = parse_path_overlap(&mut input)?;
        let mut optional: BString = OptionalFields::parse_tag(input)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        optional.pop();
         */
        parse_path_overlap(&mut input)?;
        input.into_iter().filter_map(|f| parse_tag(f.as_ref()));
        Ok(Path {
            path_name,
            segment_names,
            //overlaps,
            //optional,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Instant;

    #[test]
    #[ignore]
    fn parse_big_file() {
        // Create gfa from file: Duration { seconds: 432, nanoseconds: 428425000 } (with find)
        // Create gfa from file: Duration { seconds: 423, nanoseconds: 311465600 } (with is_match)
        let parser = GFAParser::default();
        let start = Instant::now();
        let _gfa2: GFA = parser
            .parse_file("./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa")
            .unwrap();
        println!("Create gfa from file: {:?}", start.elapsed());
    }

    #[test]
    fn parse_med_file() {
        // Create gfa from file: Duration { seconds: 0, nanoseconds: 271638800 } (with is_match)
        let parser = GFAParser::default();
        let start = Instant::now();
        let _gfa2: GFA =
            parser.parse_file("./tests/big_files/test.gfa").unwrap();
        println!("Create gfa from file: {:?}", start.elapsed());
    }

    #[test]
    #[ignore]
    fn parse_big_file1() {
        // Create gfa from file: Duration { seconds: 535, nanoseconds: 662080200 } (with is_match)
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
    /*
    #[test]
    fn blank_header() {
        let header = "";
        let header_ = Header {
            version: "".into(),
            optional: BString::from(""),
        };

        let fields = header.split_terminator('\t');
        let result = Header::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(h) => assert_eq!(h, header_),
        }
    }

    #[test]
    fn can_parse_header() {
        let header = "VN:Z:1.0";
        let header_ = Header {
            version: "VN:Z:1.0".into(),
            optional: BString::from(""),
        };

        let fields = header.split_terminator('\t');
        let result = Header::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(h) => assert_eq!(h, header_),
        }
    }

    #[test]
    fn can_parse_segment() {
        let segment = "A\tAAAAAAACGT";
        let segment_: Segment = Segment {
            name: convert_to_usize(b"A").unwrap(),
            sequence: "AAAAAAACGT".into(),
            optional: BString::from(""),
        };

        let fields = segment.split_terminator('\t');
        let result = Segment::parse_line(fields);

        match result {
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
            overlap: "20M".into(),
            optional: BString::from(""),
        };

        let fields = link.split_terminator('\t');
        let result = Link::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(l) => assert_eq!(l, link_),
        }
    }

    #[test]
    fn can_parse_containments() {
        let containment = "15\t-\t10\t+\t4\t20M";
        let containment_: Containment = Containment {
            container_name: 15,
            container_orient: Orientation::Backward,
            contained_name: 10,
            contained_orient: Orientation::Forward,
            pos: 4,
            overlap: "20M".into(),
            optional: BString::from(""),
        };

        let fields = containment.split_terminator('\t');
        let result = Containment::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(c) => assert_eq!(c, containment_),
        }
    }

    #[test]
    fn can_parse_path() {
        let path = "14\t11+,12-,13+\t4M,5M";
        let path_: Path =
            Path::new("14".into(), "11+,12-,13+".into(), "4M,5M".into(), b"");

        let fields = path.split_terminator('\t');
        let result = Path::parse_line(fields);

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(p) => assert_eq!(p, path_),
        }
    }

    #[test]
    fn can_parse_alignment_cigar() {
        let cigar = vec!["1M1I1M1I2M"];
        let result = parse_overlap(&mut cigar.iter());

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
    fn can_parse_no_alignment() {
        let no_aligment = vec!["*"];
        let result = parse_overlap(&mut no_aligment.iter());

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
        let result = parse_overlap(&mut error.iter());

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
     */
}
