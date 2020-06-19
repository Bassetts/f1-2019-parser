use std::fmt;

#[non_exhaustive]
#[derive(Debug)]
pub enum ErrorKind {
    NomParserError,
    NomIncompleteError(nom::Needed),
}

#[derive(Debug)]
pub struct ParseError(ErrorKind);

impl ParseError {
    pub(crate) fn new(kind: ErrorKind) -> ParseError {
        ParseError(kind)
    }
}

impl<I: std::fmt::Debug> From<nom::Err<(I, nom::error::ErrorKind)>> for ParseError {
    fn from(i: nom::Err<(I, nom::error::ErrorKind)>) -> Self {
        match i {
            nom::Err::Error(_) | nom::Err::Failure(_) => ParseError::new(ErrorKind::NomParserError),
            nom::Err::Incomplete(i) => ParseError::new(ErrorKind::NomIncompleteError(i)),
        }
    }
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ErrorKind::NomParserError => write!(f, "Error parsing packet"),
            ErrorKind::NomIncompleteError(ref needed) => {
                write!(f, "Error parsing, needs more date: {:?}", needed)
            }
        }
    }
}
