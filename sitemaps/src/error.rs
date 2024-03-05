use chrono::ParseError as ChronoParseError;
use quick_xml::Error as XmlError;
use std::io::Error as IoError;
use std::num::ParseFloatError;
use ureq::Error as HttpError;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum SitemapError {
    XmlError(XmlError),
    IoError(IoError),
    ParsePriorityError(ParseFloatError),
    EncodingError,
    HttpError(HttpError),
    UrlParseError(UrlParseError),
    TooManyUrls,
    UrlValueTooLong,
    PriorityTooLow,
    PriorityTooHigh,
    W3CDatetimeParseError(ChronoParseError),
}

impl From<XmlError> for SitemapError {
    fn from(value: XmlError) -> Self {
        SitemapError::XmlError(value)
    }
}

impl From<IoError> for SitemapError {
    fn from(value: IoError) -> Self {
        SitemapError::IoError(value)
    }
}

impl From<ParseFloatError> for SitemapError {
    fn from(value: ParseFloatError) -> Self {
        SitemapError::ParsePriorityError(value)
    }
}

impl From<HttpError> for SitemapError {
    fn from(value: HttpError) -> Self {
        SitemapError::HttpError(value)
    }
}

impl From<ChronoParseError> for SitemapError {
    fn from(value: ChronoParseError) -> Self {
        SitemapError::W3CDatetimeParseError(value)
    }
}

impl From<UrlParseError> for SitemapError {
    fn from(value: UrlParseError) -> Self {
        SitemapError::UrlParseError(value)
    }
}
