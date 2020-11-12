/// file that is used to define all the common types that can be
/// parsed and used as SegmentId
use crate::parser::ParseFieldError;

use bstr::{BString, ByteSlice};
use lazy_static::lazy_static;
use regex::bytes::Regex;

/// Trait for the types that can be parsed and used as segment IDs;
/// will probably only be usize and BString.
pub trait SegmentId: std::fmt::Display + Sized + Default {
    const ERROR: ParseFieldError;

    // define the functions
    fn parse_opt_id(input: &[u8]) -> Option<Self>;
    fn parse_id(input: &[u8]) -> Option<Self>;
    fn parse_ref(input: &[u8]) -> Option<Self>;

    fn parse_next<I>(mut input: I) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        Self::parse_id(next.as_ref()).ok_or(Self::ERROR)
    }

    fn parse_next_opt<I>(mut input: I) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        Self::parse_opt_id(next.as_ref()).ok_or(Self::ERROR)
    }

    fn parse_next_ref<I>(mut input: I) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        Self::parse_ref(next.as_ref()).ok_or(Self::ERROR)
    }
}

impl SegmentId for usize {
    const ERROR: ParseFieldError = ParseFieldError::UintIdError;

    fn parse_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref REX: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
        }
        if REX.is_match(input.as_ref()) {
            //convert_alphanumeric(input)
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
        } else {
            panic!("Error! the id tag it's not correct")
        }
    }

    fn parse_opt_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref REX: Regex = Regex::new(r"(?-u)[!-~]+|\*").unwrap();
        }
        if REX.is_match(input.as_ref()) {
            //convert_alphanumeric(input)
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
        } else {
            panic!("Error! the optional id tag it's not correct")
        }
    }

    fn parse_ref(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref REX: Regex = Regex::new(r"(?-u)[!-~]+[+-]").unwrap();
        }
        if REX.is_match(input.as_ref()) {
            let last = input.len() - 1;

            let orient = match input[last] {
                b'+' => 0 as usize,
                b'-' => 1 as usize,
                _ => panic!("reference segment did not include orientation"),
            };
            let segment_id = &input[..last];
            let my_vec: Vec<char> = segment_id.to_str().unwrap().chars().collect();
            let mut x = 0;
            let mut res: String = "".to_string();
            while x < last {
                res = format!(
                    "{}{}",
                    res,
                    &get_code_from_char(&my_vec[x].to_string()).to_string()
                );
                x += 1;
            }
            match res.len() {
                1..=20 => format!("{}{}", res, orient).parse::<usize>().ok(),
                _ => panic!(
                    "Error! the conversion of the string: {} (length: {}) into usize: {} (lenght {}) exceeds {} ",
                    segment_id.to_str().unwrap(), segment_id.len(), res, res.len(), "the maximum length (20 digits)"
                    ),
            }
        } else {
            panic!("Error! the reference tag it's not correct")
        }
    }
}

impl SegmentId for BString {
    const ERROR: ParseFieldError = ParseFieldError::Utf8Error;

    fn parse_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
        }
        RE.find(input).map(|s| BString::from(s.as_bytes()))
    }

    fn parse_opt_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?-u)[!-~]+|\*").unwrap();
        }
        RE.find(input).map(|s| BString::from(s.as_bytes()))
    }

    fn parse_ref(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?-u)[!-~]+[+-]").unwrap();
        }
        RE.find(input).map(|s| BString::from(s.as_bytes()))
    }
}

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

/// function that performs the conversion from the code to the associated symbol
/// # Example
/// ```ignore
///  let a: &str = "a";
/// let a_: i32 = 97;
/// assert_eq!(a, get_char_from_code(a_));
/// ```
fn get_char_from_code(c: i32) -> &'static str {
    CHARS.get(c as usize).unwrap_or(&"")
}

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
