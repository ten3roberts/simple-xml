#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseError(ParseError, usize),
    ContentOutsideRoot,
    TagNotFound(String, String),
    AttributeNotFound(String, String),
}

#[derive(Debug)]
pub enum ParseError {
    MissingClosingTag(String),
    MissingClosingDelimiter,
    MissingAttributeValue(String),
    MissingQuotes(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}
