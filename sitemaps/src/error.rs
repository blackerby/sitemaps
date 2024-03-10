use chrono::ParseError as ChronoParseError;
use quick_xml::events::attributes::AttrError;
use quick_xml::Error as XmlError;
use std::io::Error as IoError;
use std::num::ParseFloatError;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum Error {
    XmlError(XmlError),
    AttrError(AttrError),
    IoError(IoError),
    ParsePriorityError(ParseFloatError),
    EncodingError,
    UrlParseError(UrlParseError),
    TooManyUrls,
    UrlValueTooLong,
    PriorityTooLow,
    PriorityTooHigh,
    W3CDatetimeParseError(ChronoParseError),
    UnexpectedEof,
    NotASitemap,
}

impl From<XmlError> for Error {
    fn from(value: XmlError) -> Self {
        Error::XmlError(value)
    }
}

impl From<AttrError> for Error {
    fn from(value: AttrError) -> Self {
        Error::AttrError(value)
    }
}

impl From<IoError> for Error {
    fn from(value: IoError) -> Self {
        Error::IoError(value)
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Error::ParsePriorityError(value)
    }
}

impl From<ChronoParseError> for Error {
    fn from(value: ChronoParseError) -> Self {
        Error::W3CDatetimeParseError(value)
    }
}

impl From<UrlParseError> for Error {
    fn from(value: UrlParseError) -> Self {
        Error::UrlParseError(value)
    }
}
