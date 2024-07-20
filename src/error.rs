use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Parsing failed")]
    ParseError(ParseError, usize),
    #[error("Found extra content before the root node")]
    ContentOutsideRoot,
    #[error("No such tag {1:?} inside {0:?}")]
    TagNotFound(String, String),
    #[error("No such attribute {1:?} inside {0:?}")]
    AttributeNotFound(String, String),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Missing closing tag for {0:?}")]
    MissingClosingTag(String),
    #[error("Missing closing delimiter")]
    MissingClosingDelimiter,
    #[error("Missing attribute value for {0:?}")]
    MissingAttributeValue(String),
    #[error("Missing quotes for {0:?}")]
    MissingQuotes(String),
}
