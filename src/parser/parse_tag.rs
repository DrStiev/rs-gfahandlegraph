/// file that tries to mimic the behaviour of the file optfields.rs
/// optfields.rs find, parse and store all the different types of
/// optional fields associated to each kind of lines.
/// with the format GFA2 the optional field tag is been replaced by a
/// simple tag element with 0 or N occurencies.
/// So, I don't think this file could be useful as the original.
use bstr::BString;
use lazy_static::lazy_static;
use regex::bytes::Regex;

/// These type aliases are useful for configuring the parsers, as the
/// type of the optional field container must be given when creating a
/// GFAParser or GFA object.
pub type OptionalFields = Vec<OptField>;
pub type NoOptionalFields = ();

/// An optional field a la SAM. Identified by its tag, which is any
/// two characters matching [A-Za-z0-9][A-Za-z0-9].
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OptField {
    pub value: BString,
}

impl OptField {
    /// Create a new OptField from a tag name and a value, panicking
    /// if the provided tag doesn't fulfill the requirements of
    /// OptField::tag().
    pub fn new(value: BString) -> Self {
        OptField { value }
    }

    /// Parses the header and optional fields from a bytestring in the format\
    /// ```<tag> <- <TAG>:<TYPE>:<VALUE> <- [A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*```
    pub fn parse_tag(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?-u)([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*)*")
                    .unwrap();
        }

        let o_val: BString =
            RE.find(input).map(|s| BString::from(s.as_bytes()))?;

        Some(Self::new(o_val))
    }
}

/// The Display implementation produces spec-compliant strings in the
/// ```<TAG>:<TYPE>:<VALUE>``` format, and can be parsed back using
/// OptField::parse().
impl std::fmt::Display for OptField {
    fn fmt(&self, form: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(form, "{}", self.value)
    }
}

/// The OptFields trait describes how to parse, store, and query
/// optional fields. Each of the GFA line types and the GFA struct
/// itself are generic over the optional fields, so the choice of
/// OptFields implementor can impact memory usage, which optional
/// fields are parsed, and possibly more in the future
pub trait OptFields: Sized + Default + Clone {
    /// Return a slice over all optional fields. NB: This may be
    /// replaced by an iterator or something else in the future
    fn fields(&self) -> &[OptField];

    /// Given an iterator over bytestrings, each expected to hold one
    /// optional field (in the <TAG>:<TYPE>:<VALUE> format), parse
    /// them as optional fields to create a collection. Returns `Self`
    /// rather than `Option<Self>` for now, but this may be changed to
    /// become fallible in the future.
    fn parse_tag<T>(input: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<[u8]>;
}

/// This implementation is useful for performance if we don't actually
/// need any optional fields. () takes up zero space, and all
/// methods are no-ops.
impl OptFields for () {
    fn fields(&self) -> &[OptField] {
        &[]
    }

    fn parse_tag<T>(_input: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<[u8]>,
    {
    }
}

/// Stores all the optional fields in a vector. `get_field` simply
/// uses std::iter::Iterator::find(), but as there are only a
/// relatively small number of optional fields in practice, it should
/// be efficient enough.
impl OptFields for Vec<OptField> {
    fn fields(&self) -> &[OptField] {
        self.as_slice()
    }

    fn parse_tag<T>(input: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<[u8]>,
    {
        input
            .into_iter()
            .filter_map(|f| OptField::parse_tag(f.as_ref()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bstr::ByteSlice;

    #[test]
    fn parse_single_tag() {
        let tag = b"DP:i:1";
        let result = OptField::parse_tag(tag);
        match result {
            None => println!("Tag not found"),
            Some(t) => assert_eq!(tag.to_str().unwrap(), t.to_string()),
        }
    }

    #[test]
    fn parse_multiple_tag() {
        let tag = "DP:i:1\tRC:i:1";
        let fields = tag.split_terminator('\t');
        let mut result: BString = OptionalFields::parse_tag(fields)
            .into_iter()
            .map(|x| BString::from(x.to_string() + "\t"))
            .collect::<BString>();
        // the last character of the result fields is always '\t' so
        // remember to pop it out otherwise it will raise an error
        result.pop();
        assert_eq!(result, tag);
    }

    #[test]
    fn parse_none_tag() {
        let tag = b"";
        let result = OptField::parse_tag(tag);
        match result {
            None => println!("Tag not found"),
            Some(t) => assert_eq!(tag.to_str().unwrap(), t.to_string()),
        }
    }
}
