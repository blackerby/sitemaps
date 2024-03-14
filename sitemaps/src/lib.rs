use std::io::{BufRead, BufReader};

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use serde::Serialize;
use sitemap::Sitemap;
use sitemap_index::SitemapIndex;

use crate::error::Error;
use quick_xml::Writer;
use std::borrow::Cow;
use std::io::Write;
use url::Url;

pub mod error;
pub mod sitemap;
pub mod sitemap_index;
pub mod w3c_datetime;

pub const NAMESPACE: &str = "http://www.sitemaps.org/schemas/sitemap/0.9";
pub const MAX_URL_LENGTH: usize = 2048;

/// A type representing Sitemaps.
/// Sitemaps are one of:
/// - a Sitemap
/// - a SitemapIndex
#[derive(Serialize)]
pub enum Sitemaps {
    Sitemap(Sitemap),
    SitemapIndex(SitemapIndex),
}

impl Sitemaps {
    /// Reads a buffer and returns a Sitemap or SitemapIndex wrapped by the
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
                            let sitemap_index = SitemapIndex::read_from(buf_reader)?;
                            return Ok(Self::SitemapIndex(sitemap_index));
                        }
                        _ => return Err(Error::NotASitemap),
                    }
                }
                _ => {}
            }
        }
    }
}

/// A trait to support entries in sitemap and sitemap index files. In a sitemap file,
/// an entry is indicated by a `<url>` element. In a sitemap index file, an entry is
/// indicated by a `<sitemap>` element. Both elements have a required ``<loc>`` child element
/// and an optional ``<lastmod>`` element.
pub trait SitemapsEntry {
    /// Return the text value of the entry's ``<loc>`` element, a URL.
    fn loc(&self) -> String;
    /// Return the value of the entry's ``<lastmod>`` element as a
    /// formatted date string.
    fn last_mod(&self) -> String;
    /// Validate the URL contained in the entry's ``<loc>`` element.
    fn validate_loc(&self) -> Result<String, Error> {
        if self.loc().chars().count() > MAX_URL_LENGTH {
            return Err(Error::UrlValueTooLong);
        }

        let url = Url::parse(&self.loc())?;

        Ok(url.as_str().into())
    }
}

/// A trait to support reading from sitemap and sitemap index files.
pub trait SitemapRead {
    /// Read from a buffer and try to return a Sitemap or a SitemapIndex.
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
}

/// A trait to support writing `Sitemap`s and `SitemapIndex`es to files
/// as XML.
pub trait SitemapWrite {
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

/// A trait to support collecting `<loc>` and `<lastmod>` values
/// from `SitemapEntry`s.
pub trait Entries {
    /// Collect the all `<loc>`s from the Sitemap or SitemapIndex.
    fn locs(&self) -> Vec<String>;
    /// Collect the all `<lastmod>`s from the Sitemap or SitemapIndex.
    fn lastmods(&self) -> Vec<String>;
}

impl Entries for Sitemaps {
    fn locs(&self) -> Vec<String> {
        match self {
            Sitemaps::Sitemap(sitemap) => sitemap.locs(),
            Sitemaps::SitemapIndex(index) => index.locs(),
        }
    }
    fn lastmods(&self) -> Vec<String> {
        match self {
            Sitemaps::Sitemap(sitemap) => sitemap.lastmods(),
            Sitemaps::SitemapIndex(index) => index.lastmods(),
        }
    }
}
