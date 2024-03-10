use std::io::BufRead;

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};

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

pub trait SitemapRead {
    fn read_from<R: BufRead>(reader: R) -> Result<Self, Error>
    where
        Self: Sized;
    fn write<W: Write>(&self, writer: Writer<W>) -> Result<W, Error>;
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
    fn _validate_url(string: &str) -> Result<String, Error> {
        if string.chars().count() > MAX_URL_LENGTH {
            return Err(Error::UrlValueTooLong);
        }

        let url = Url::parse(string)?;

        Ok(url.as_str().into())
    }

    /// Serialize a Sitemap to a Writer as XML.                   
    fn write_to<W: Write>(&self, writer: W) -> Result<W, Error> {
        self.write(Writer::new(writer))
    }

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
