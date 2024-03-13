use chrono::ParseError as ChronoParseError;
use quick_xml::events::attributes::AttrError;
use quick_xml::Error as XmlError;
use std::io::Error as IoError;
use std::num::ParseFloatError;
use thiserror::Error;
use url::ParseError as UrlParseError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("XML parsing error")]
    XmlError(#[from] XmlError),
    #[error("XML attribute error")]
    AttrError(#[from] AttrError),
    #[error("IO error")]
    IoError(#[from] IoError),
    #[error("Floating point parse error for Priority")]
    ParsePriorityError(#[from] ParseFloatError),
    #[error("Sitemap encoding error")]
    EncodingError,
    #[error("Invalid URL error")]
    UrlParseError(#[from] UrlParseError),
    #[error("Too many URLs in document")]
    TooManyUrls,
    #[error("URL exceeds length limit")]
    UrlValueTooLong,
    #[error("Priority must not be lower than 0.0")]
    PriorityTooLow,
    #[error("Priority must not be higher than 1.0")]
    PriorityTooHigh,
    #[error("Problem parsing into W3C datetime format")]
    W3CDatetimeParseError(#[from] ChronoParseError),
    #[error("Unexpected EOF")]
    UnexpectedEof,
    #[error("Not a sitemap: root element must be <urlset> or <sitemapindex>")]
    NotASitemap,
}
