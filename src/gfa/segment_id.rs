/// file that is used to define all the common types that can be
/// parsed and used as SegmentId
use crate::parser::ParseFieldError;

use bstr::{BString, ByteSlice};
use lazy_static::lazy_static;
use regex::bytes::Regex;

pub enum IdType {
    ID(),
    OPTIONALID(),
    REFERENCEID(),
}

lazy_static! {
    static ref RE_ID: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
    static ref RE_OPTIONAL_ID: Regex = Regex::new(r"(?-u)[!-~]+|\*").unwrap();
    static ref RE_REFERENCE_ID: Regex = Regex::new(r"(?-u)[!-~]+[+-]").unwrap();
}

/// Trait for the types that can be parsed and used as segment IDs;
/// will probably only be usize and BString.
pub trait SegmentId: std::fmt::Display + Sized + Default {
    const ERROR: ParseFieldError;

    // define the functions
    fn parse_id(id: IdType, input: &[u8]) -> Option<Self>;

    fn parse_next<I>(mut input: I, id: IdType) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        match id {
            IdType::ID() => Self::parse_id(IdType::ID(), next.as_ref()).ok_or(Self::ERROR),
            IdType::OPTIONALID() => {
                Self::parse_id(IdType::OPTIONALID(), next.as_ref()).ok_or(Self::ERROR)
            }
            IdType::REFERENCEID() => {
                Self::parse_id(IdType::REFERENCEID(), next.as_ref()).ok_or(Self::ERROR)
            }
        }
    }
}

impl SegmentId for usize {
    const ERROR: ParseFieldError = ParseFieldError::UintIdError;

    fn parse_id(id: IdType, input: &[u8]) -> Option<Self> {
        match id {
            IdType::ID() => {
                if RE_ID.is_match(input) {
                    convert_to_usize(input)
                } else {
                    panic!("Error! the reference tag it's not correct")
                }
            }
            IdType::OPTIONALID() => {
                if RE_OPTIONAL_ID.is_match(input) {
                    convert_to_usize(input)
                } else {
                    panic!("Error! the reference tag it's not correct")
                }
            }
            IdType::REFERENCEID() => {
                if RE_REFERENCE_ID.is_match(input) {
                    convert_to_usize(input)
                } else {
                    panic!("Error! the reference tag it's not correct")
                }
            }
        }
    }
}

impl SegmentId for BString {
    const ERROR: ParseFieldError = ParseFieldError::Utf8Error;

    fn parse_id(id: IdType, input: &[u8]) -> Option<Self> {
        match id {
            IdType::ID() => RE_ID.find(input).map(|s| BString::from(s.as_bytes())),
            IdType::OPTIONALID() => RE_OPTIONAL_ID
                .find(input)
                .map(|s| BString::from(s.as_bytes())),
            IdType::REFERENCEID() => RE_REFERENCE_ID
                .find(input)
                .map(|s| BString::from(s.as_bytes())),
        }
    }
}

fn convert_to_usize(input: &[u8]) -> Option<usize> {
    let len = input.len();
    let my_vec: Vec<char> = input.to_str().unwrap().chars().collect();
    let mut x = 0;
    let mut res: String = "".to_string();
    while x < len {
        res = format!(
            "{}{}",
            res,
            &get_code_from_char(&my_vec[x].to_string()).to_string()
        );
        x += 1;
    }
    match res.len() {
        1..=20 => Some(res.parse::<usize>().unwrap()),
        _ => panic!(
            "Error! the conversion of the string: {} (length: {}) into usize: {} (lenght {}) exceeds {} ",
            input.to_str().unwrap(), input.len(), res, res.len(), "the maximum length (20 digits)"
        ),
    }
}

// + => 43, - => 45
/// array to perform the conversion from symbols to usize and viceversa
const CHARS: [&str; 128] = [
    // unprintable characters, never used but they need to be here
    "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "HT", "LF", "VT", "FF", "CR",
    "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM", "SUB", "ESC",
    "FS", "GS", "RS", "US", // printable characters, the ones that will be used
    " ", "!", "\"", "#", "$", "%", "&", "\'", "(", ")", "*", "+", ",", "-", ".", "/", "0", "1",
    "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", "@", "A", "B", "C", "D",
    "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W",
    "X", "Y", "Z", "[", "\\", "]", "^", "_", "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
    "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}",
    "~", // even if printable, this character it's not used
    "DEL",
];

/// function that performs the conversion from a symbol to the associated ascii code
/// # Example
/// ```ignore
///  let a: &str = "a";
/// let a_: usize = 97;
/// assert_eq!(a_, get_code_from_char(a));
/// ```
fn get_code_from_char(c: &str) -> usize {
    if c.parse::<u64>().is_ok() {
        c.parse::<usize>().unwrap()
    } else {
        CHARS.iter().position(|&x| x == c).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let edge = "45+";
        let edge_ = edge.split_terminator('\t');
        let res = usize::parse_next(edge_, IdType::REFERENCEID());
        println!("usize: {}", res.unwrap());

        let edge = "r1-";
        let edge_ = edge.split_terminator('\t');
        let res = usize::parse_next(edge_, IdType::REFERENCEID());
        println!("usize: {}", res.unwrap());

        let edge = "*";
        let edge_ = edge.split_terminator('\t');
        let res = BString::parse_next(edge_, IdType::OPTIONALID());
        println!("BString: {}", res.unwrap());

        let edge = "r1-";
        let edge_ = edge.split_terminator('\t');
        let res = BString::parse_next(edge_, IdType::REFERENCEID());
        println!("BString: {}", res.unwrap());
    }
}
