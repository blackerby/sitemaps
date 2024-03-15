//! Read and write files in the [Sitemaps XML format](https://sitemaps.org/protocol.html)
//!
//! ```rust
//! use std::fs::File;
//! use std::io::BufReader;
//! use sitemaps::SitemapsFile;
//!
//! let file = File::open("tests/data/example_1_url.xml").unwrap();
//! let reader = BufReader::new(file);
//! let sitemap = SitemapsFile::read(reader).unwrap();
//! ```

use std::io::{BufRead, BufReader};

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use serde::Serialize;
use siteindex::SiteIndex;
use sitemap::Sitemap;

use crate::error::Error;
use quick_xml::{Reader, Writer};
use std::borrow::Cow;
use std::io::Write;
use url::Url;

pub mod error;
pub mod siteindex;
pub mod sitemap;
pub mod w3c_datetime;

pub const NAMESPACE: &str = "http://www.sitemaps.org/schemas/sitemap/0.9";
pub const MAX_URL_LENGTH: usize = 2048;

/// A type representing the data in a sitemap file.
///
/// A SitemapsFile is one of:
/// - a Sitemap, representing sitemap.xml files with `<urlset>` as the root element
/// - a SiteIndex, representing sitemap.xml files with `<sitemapindex>` as the root element
#[derive(Serialize)]
pub enum SitemapsFile {
    Sitemap(Sitemap),
    SiteIndex(SiteIndex),
}

impl SitemapsFile {
    /// Reads a buffer and returns a Sitemap or SiteIndex wrapped by the
    /// Sitemaps enum wrapper.
    pub fn read<R: BufRead>(mut reader: R) -> Result<Self, Error> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let mut xml_reader = Reader::from_str(&buf);
        xml_reader.trim_text(true).expand_empty_elements(true);

        loop {
            match xml_reader.read_event() {
                Err(e) => panic!(
                    "Error at position {}: {:?}",
                    xml_reader.buffer_position(),
                    e
                ),
                Ok(Event::Eof) => return Err(Error::UnexpectedEof),
                Ok(Event::Start(start)) => {
                    let buf_reader = BufReader::new(buf.as_bytes());
                    match start.name().as_ref() {
                        b"urlset" => {
                            let sitemap = Sitemap::read_from(buf_reader)?;
                            return Ok(Self::Sitemap(sitemap));
                        }
                        b"sitemapindex" => {
                            let siteindex = SiteIndex::read_from(buf_reader)?;
                            return Ok(Self::SiteIndex(siteindex));
                        }
                        _ => return Err(Error::NotASitemap),
                    }
                }
                _ => {}
            }
        }
    }
}

/// A trait containing the behavior [`Sitemap`s](Sitemap) and [`SiteIndex`es](SiteIndex).
pub trait Sitemaps {
    fn new() -> Self;
    fn read_from<R: BufRead>(reader: R) -> Result<Self, Error>
    where
        Self: Sized;

    /// Check that the encoding of the file being read from is UTF-8.
    fn check_encoding(e: BytesDecl) -> Result<(), Error> {
        let encoding = e.encoding();

        if encoding.is_none() {
            return Err(Error::EncodingError);
        }

        if let Some(Ok(Cow::Borrowed(encoding))) = encoding {
            if encoding.to_ascii_lowercase() != b"utf-8" {
                return Err(Error::EncodingError);
            }
        }
        Ok(())
    }
    fn write<W: Write>(&self, writer: Writer<W>) -> Result<W, Error>;

    /// Serialize a Sitemap to a Writer as XML.
    fn write_to<W: Write>(&self, writer: W) -> Result<W, Error> {
        self.write(Writer::new(writer))
    }

    /// Write an XML text element.
    fn write_text_element<W: Write, N: AsRef<str>, T: AsRef<str>>(
        writer: &mut Writer<W>,
        name: N,
        text: T,
    ) -> Result<(), Error> {
        let name = name.as_ref();

        writer.write_event(Event::Start(BytesStart::new(name)))?;
        writer.write_event(Event::Text(BytesText::new(text.as_ref())))?;
        writer.write_event(Event::End(BytesEnd::new(name)))?;

        Ok(())
    }
}

/// A trait to support entries in sitemap and sitemap index files. In a sitemap file,
/// an entry is indicated by a `<url>` element. In a sitemap index file, an entry is
/// indicated by a `<sitemap>` element. Both elements have a required `<loc>` child element
/// and an optional `<lastmod>` element.
pub trait SitemapsEntry {
    /// Return the text value of the entry's `<loc>` element, a URL.
    fn loc(&self) -> String;
    /// Return the value of the entry's `<lastmod>` element as a
    /// formatted date string.
    fn last_mod(&self) -> String;
    /// Validate the URL contained in the entry's `<loc>` element.
    fn validate_loc(&self) -> Result<String, Error> {
        if self.loc().chars().count() > MAX_URL_LENGTH {
            return Err(Error::UrlValueTooLong);
        }

        let url = Url::parse(&self.loc())?;

        Ok(url.as_str().into())
    }
}

/// A trait to support collecting `<loc>` and `<lastmod>` values
/// from `SitemapEntry`s.
pub trait Entries {
    /// Collect the all `<loc>`s from the Sitemap or SiteIndex.
    fn locs(&self) -> Vec<String>;
    /// Collect the all `<lastmod>`s from the Sitemap or SiteIndex.
    fn lastmods(&self) -> Vec<String>;
}

impl Entries for SitemapsFile {
    fn locs(&self) -> Vec<String> {
        match self {
            SitemapsFile::Sitemap(sitemap) => sitemap.locs(),
            SitemapsFile::SiteIndex(index) => index.locs(),
        }
    }
    fn lastmods(&self) -> Vec<String> {
        match self {
            SitemapsFile::Sitemap(sitemap) => sitemap.lastmods(),
            SitemapsFile::SiteIndex(index) => index.lastmods(),
        }
    }
}
