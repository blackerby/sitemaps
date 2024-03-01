use quick_xml::Error;
use std::io::Error as IoError;
use std::num::ParseFloatError;
use time::error::Parse;
use ureq::Error as UreqError;
use url::ParseError;

#[derive(Debug)]
pub enum SitemapError {
    QuickXMLError(Error),
    IoError(IoError),
    ParsePriorityError(ParseFloatError),
    EncodingError,
    HttpRequestError(UreqError),
    UrlParseError(ParseError),
    TooManyUrls,
    UrlValueTooLong,
    PriorityTooLow,
    PriorityTooHigh,
    TimeParseError(Parse),
}

impl From<Error> for SitemapError {
    fn from(value: Error) -> Self {
        SitemapError::QuickXMLError(value)
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

impl From<UreqError> for SitemapError {
    fn from(value: UreqError) -> Self {
        SitemapError::HttpRequestError(value)
    }
}

impl From<ParseError> for SitemapError {
    fn from(value: ParseError) -> Self {
        SitemapError::UrlParseError(value)
    }
}

impl From<Parse> for SitemapError {
    fn from(value: Parse) -> Self {
        SitemapError::TimeParseError(value)
    }
}
