use chrono::ParseError as ChronoParseError;
use quick_xml::Error as QuickXMLError;
use std::io::Error as IoError;
use std::num::ParseFloatError;
use ureq::Error as UreqError;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum SitemapError {
    QuickXMLError(QuickXMLError),
    IoError(IoError),
    ParsePriorityError(ParseFloatError),
    EncodingError,
    HttpRequestError(UreqError),
    UrlParseError(UrlParseError),
    TooManyUrls,
    UrlValueTooLong,
    PriorityTooLow,
    PriorityTooHigh,
    ChronoParseError(ChronoParseError),
}

impl From<QuickXMLError> for SitemapError {
    fn from(value: QuickXMLError) -> Self {
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

impl From<ChronoParseError> for SitemapError {
    fn from(value: ChronoParseError) -> Self {
        SitemapError::ChronoParseError(value)
    }
}

impl From<UrlParseError> for SitemapError {
    fn from(value: UrlParseError) -> Self {
        SitemapError::UrlParseError(value)
    }
}
