use crate::gfa::{gfa1::*, orientation::Orientation, segment_id::*};
use crate::parser::error::*;

use bstr::{BString, ByteSlice};
use lazy_static::lazy_static;
use regex::bytes::Regex;

fn next_field<I, P>(mut input: I) -> ParserFieldResult<P>
where
    I: Iterator<Item = P>,
    P: AsRef<[u8]>,
{
    input.next().ok_or(ParseFieldError::MissingFields)
}

fn parse_orientation<I>(mut input: I) -> ParserFieldResult<Orientation>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    let next = next_field(&mut input)?;
    let parsed = Orientation::from_bytes_plus_minus(next.as_ref());
    Orientation::parse_error(parsed)
}

/// function that parses the version of the header tag
/// ```<header> <- {VN:Z:1.0}  <- ([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[0-9]+\.[0-9]+)?```
fn parse_header_tag<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?-u)([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[0-9]+\.[0-9]+)?").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Version"))
}

/// function that parses the tag element
/// ```<tag> <- [A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*```
fn parse_tag<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?-u)([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*)*").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Tag"))
}

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
        let version = parse_header_tag(&mut input)?;
        let optional: BString = parse_tag(&mut input).unwrap_or_else(|_| BString::from(""));

        Ok(Header { version, optional })
    }
}

/// function that parses the overlap tag
/// ```<overlap> <- * | <CIGAR> <- ([0-9]+[MIDNSHPX=])+```
fn parse_overlap<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\*|([0-9]+[MIDNSHPX=])+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Overlap"))
}

fn parse_sequence<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\*|[A-Za-z=.]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Sequence"))
}

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
        let name = N::parse_next(&mut input, IdType::ID())?;
        let sequence = parse_sequence(&mut input)?;
        let optional: BString = parse_tag(&mut input).unwrap_or_else(|_| BString::from(""));

        Ok(Segment {
            name,
            sequence,
            optional,
        })
    }
}

impl<N: SegmentId> Link<N> {
    #[inline]
    pub fn wrap(self) -> Line<N> {
        Line::Link(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let from_segment = N::parse_next(&mut input, IdType::ID())?;
        let from_orient = parse_orientation(&mut input)?;
        let to_segment = N::parse_next(&mut input, IdType::ID())?;
        let to_orient = parse_orientation(&mut input)?;
        let overlap = parse_overlap(&mut input)?;
        let optional: BString = parse_tag(&mut input).unwrap_or_else(|_| BString::from(""));

        Ok(Link {
            from_segment,
            from_orient,
            to_segment,
            to_orient,
            overlap,
            optional,
        })
    }
}

impl<N: SegmentId> Containment<N> {
    #[inline]
    pub fn wrap(self) -> Line<N> {
        Line::Containment(self)
    }

    #[inline]
    pub fn parse_line<I>(mut input: I) -> ParserFieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let container_name = N::parse_next(&mut input, IdType::ID())?;
        let container_orient = parse_orientation(&mut input)?;
        let contained_name = N::parse_next(&mut input, IdType::ID())?;
        let contained_orient = parse_orientation(&mut input)?;
        let pos = next_field(&mut input)?;
        let pos = pos.as_ref().to_str()?.parse()?;
        let overlap = parse_overlap(&mut input)?;
        let optional: BString = parse_tag(&mut input).unwrap_or_else(|_| BString::from(""));

        Ok(Containment {
            container_name,
            container_orient,
            contained_name,
            contained_orient,
            overlap,
            pos,
            optional,
        })
    }
}

/// function that parses the overlap tag
/// ```<overlap> <- * | <CIGAR> <- [0-9]+[MIDNSHPX=](,[0-9]+[MIDNSHPX=])*```
fn parse_path_overlap<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?-u)\*|[0-9]+[MIDNSHPX=](,[0-9]+[MIDNSHPX=])*").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Overlap"))
}

/// function that parses the segment names tag
/// ```<overlap> <- * | <CIGAR> <- [!-~]+(,[!-~]+)*```
fn parse_segment_names<I>(input: &mut I) -> ParserFieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        // that's a little meh but still ok
        static ref RE: Regex = Regex::new(r"(?-u)[!-~]+(,[!-~]+)*").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Segment names"))
}

impl<N: SegmentId> Path<N> {
    #[inline]
    pub fn wrap(self) -> Line<N> {
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
        let overlaps = parse_path_overlap(&mut input)?;
        let optional: BString = parse_tag(&mut input).unwrap_or_else(|_| BString::from(""));

        Ok(Path::new(path_name, segment_names, overlaps, &optional))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let segment_: Segment<BString> = Segment {
            name: "A".into(),
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
        let link_: Link<BString> = Link {
            from_segment: "15".into(),
            from_orient: Orientation::Backward,
            to_segment: "10".into(),
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
        let containment_: Containment<BString> = Containment {
            container_name: "15".into(),
            container_orient: Orientation::Backward,
            contained_name: "10".into(),
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
        let path_: Path<BString> =
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
}
