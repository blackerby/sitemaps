use chrono::ParseError as ChronoParseError;
use quick_xml::events::attributes::AttrError;
use quick_xml::Error as XmlError;
use std::io::Error as IoError;
use std::num::ParseFloatError;
use thiserror::Error;
use url::ParseError as UrlParseError;

#[derive(Debug, Error)]
/// Errors that occur when reading or writing a sitemap.xml file.
pub enum Error {
    /// An XML parsing error.
    #[error("XML parsing error")]
    XmlError(#[from] XmlError),
    /// An error when retriving attributes from an XML elment.
    #[error("XML attribute error")]
    AttrError(#[from] AttrError),
    /// An error when reading or writing to a file or buffer.
    #[error("IO error")]
    IoError(#[from] IoError),
    /// An error when creating a sitemap priority value.
    #[error("Floating point parse error for Priority")]
    ParsePriorityError(#[from] ParseFloatError),
    /// An error when an incorrect encoding declaration is encountered.
    #[error("Sitemap encoding error. Must be \"utf-8\"")]
    EncodingError,
    /// An error for an incorrectly formed URL in a <loc> element.
    #[error("Invalid URL error")]
    UrlParseError(#[from] UrlParseError),
    /// An error when there are more than 50,000 <url> elements in a sitemap file
    /// or more than 50,000 <sitemap> elements in a sitemap index file.
    #[error("Too many URLs in document. Cannot exceed 50,000.")]
    TooManyUrls,
    /// An error when a url in a <loc> element exceeds 2048 characters in length.
    #[error("URL exceeds length limit: must not be more than 2048 characters.")]
    UrlValueTooLong,
    /// An error when a priority value is less than 0.0.
    #[error("Priority must not be lower than 0.0")]
    PriorityTooLow,
    /// An error when a priority value is greater than 1.0.
    #[error("Priority must not be higher than 1.0")]
    PriorityTooHigh,
    /// A date/time parsing error.
    #[error("Problem parsing into W3C datetime format")]
    W3CDatetimeParseError(#[from] ChronoParseError),
    /// An error when EOF is encountered unexpectedly early.
    #[error("Unexpected EOF")]
    UnexpectedEof,
    /// An error when a file that is not a sitemap or sitemap index is passed in for reading.
    #[error("Not a sitemap: root element must be <urlset> or <sitemapindex>")]
    NotASitemap,
}
