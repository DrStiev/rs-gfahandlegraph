use crate::handlegraph::error::*;
use bstr::ByteSlice;
/// define a custom error for the GFA2 format
use std::{error, fmt};

/// New type defining the [Result] obtained after parsing a line of a file
///
/// [Result]: https://doc.rust-lang.org/std/result/
pub type ParserFieldResult<T> = Result<T, ParseFieldError>;
/// New type defining the [Result] obtained after parsing a file
///
/// [Result]: https://doc.rust-lang.org/std/result/
pub type ParserResult<T> = Result<T, ParseError>;

/// Type encapsulating the kind of error encountered
/// ```ignore
/// pub enum ParserTolerance {
///     IgnoreAll,
///     Safe,
///     Pedantic,
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub enum ParserTolerance {
    IgnoreAll,
    Safe,
    Pedantic,
}

impl Default for ParserTolerance {
    fn default() -> Self {
        Self::Safe
    }
}

/// Type encapsulating different kinds of GFA fields parsing errors
/// ```ignore
/// pub enum ParseFieldError {
///     UintIdError,
///     Utf8Error,
///     ParseFromStringError,
///     OrientationError,
///     InvalidField(&'static str),
///     MissingFields,
///     Unknown,
/// }
/// ```
#[derive(Debug, Clone)]
pub enum ParseFieldError {
    /// A segment ID couldn't be parsed as a u64. Can only happen
    /// when parsing into a GFA<u64, T>.
    UintIdError,
    /// A bytestring couldn't be parsed as a bytestring, can happen
    /// when the contents aren't UTF8.
    Utf8Error,
    /// A field couldn't be parsed into the correct type
    ParseFromStringError,
    /// Attempted to parse an orientation that wasn't + or -.
    OrientationError,
    /// A required field was incorrectly formatted. Includes the field
    /// name as defined by the GFA1 spec.
    InvalidField(&'static str),
    MissingFields,
    Unknown,
}

macro_rules! impl_many_from {
    ($to:ty, ($from:ty, $out:expr)) => ();
    ($to:ty, ($from:ty, $out:expr), $(($f:ty, $o:expr)),* $(,)?) => (
        impl From<$from> for $to {
            fn from(_: $from) -> Self {
                $out
            }
        }
        impl_many_from!($to, $(($f, $o)),*);
    );
}

impl_many_from!(
    ParseFieldError,
    (std::str::Utf8Error, ParseFieldError::Utf8Error),
    (bstr::Utf8Error, ParseFieldError::Utf8Error),
    (
        std::num::ParseIntError,
        ParseFieldError::ParseFromStringError
    ),
    (
        std::num::ParseFloatError,
        ParseFieldError::ParseFromStringError
    )
);

impl fmt::Display for ParseFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseFieldError as PFE;
        match self {
            PFE::UintIdError => {
                write!(f, "Failed to parse a segment ID as an unsigned integer")
            }
            PFE::Utf8Error => {
                write!(f, "Failed to parse a bytestring as a UTF-8 string")
            }
            PFE::ParseFromStringError => {
                write!(f, "Failed to parse a field from a string")
            }
            PFE::OrientationError => {
                write!(f, "Failed to parse an orientation character")
            }
            PFE::InvalidField(field) => {
                write!(f, "Failed to parse field `{}`", field)
            }
            PFE::MissingFields => write!(f, "Line is missing required fields"),
            PFE::Unknown => write!(f, "Unknown error when parsing a field"),
        }
    }
}

impl error::Error for ParseFieldError {}

/// Type encapsulating different kinds of GFA parsing errors
/// ```ignore
/// pub enum ParseError {
///     UnknownLineType,
///     EmptyLine,
///     InvalidLine(ParseFieldError, String),
///     InvalidField(ParseFieldError),
///     IOError(std::io::Error),
///     ExtensionError(),
///     ConversionGFAToGraph(String),
///     Unknown,
/// }
/// ```
#[derive(Debug)]
pub enum ParseError {
    /// The line type was something other than 'H', 'S', 'F', 'E',
    /// 'G', 'O' or 'U'. This is ignored by the file parser rather than a fail
    /// condition.
    UnknownLineType,
    /// Tried to parse an empty line. Can be ignored.
    EmptyLine,
    /// A line couldn't be parsed. Includes the problem line and a
    /// variant describing the error.
    InvalidLine(ParseFieldError, String),
    /// A field couldn't be parsed
    InvalidField(ParseFieldError),
    /// Wrapper for an IO error.
    IOError(std::io::Error),
    ExtensionError(),
    ConversionGFAToGraph(String),
    Unknown,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError as PE;
        match self {
            PE::UnknownLineType => write!(f, "Unknown line type"),
            PE::EmptyLine => write!(f, "Line was empty"),
            PE::InvalidLine(field_err, line) => {
                write!(f, "Failed to parse line {}, error: {}", line, field_err)
            }
            PE::InvalidField(field_err) => {
                write!(f, "Failed to parse field: {}", field_err)
            }
            PE::IOError(err) => write!(f, "IO error: {}", err),
            PE::ExtensionError() => write!(f, "Extension not correct!"),
            PE::ConversionGFAToGraph(why) => write!(f, "{}", why),
            PE::Unknown => write!(f, "Unknown error when parsing a line"),
        }
    }
}

impl From<std::io::Error> for ParseError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<ParseFieldError> for ParseError {
    #[inline]
    fn from(err: ParseFieldError) -> Self {
        Self::InvalidField(err)
    }
}

impl From<GraphError> for ParseError {
    #[inline]
    fn from(err: GraphError) -> Self {
        Self::ConversionGFAToGraph(err.to_string())
    }
}

impl error::Error for ParseError {}

impl ParseError {
    pub(crate) fn invalid_line(error: ParseFieldError, line: &[u8]) -> Self {
        let mut dest = String::new();
        line.to_str_lossy_into(&mut dest);
        Self::InvalidLine(error, dest)
    }

    #[inline]
    pub fn can_safely_continue(&self, tol: &ParserTolerance) -> bool {
        use ParserTolerance as Tol;
        match tol {
            Tol::IgnoreAll => true,
            Tol::Safe => matches!(
                self,
                ParseError::EmptyLine | ParseError::UnknownLineType
            ),
            Tol::Pedantic => false,
        }
    }
}
