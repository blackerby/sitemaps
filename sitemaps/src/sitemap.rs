use crate::{MAX_URL_LENGTH, NAMESPACE};
use core::fmt;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use serde::Serialize;
use std::borrow::Cow;
use std::io::{BufRead, Write};
use url::Url;

use crate::{error::Error, w3c_datetime::W3CDateTime};

/// A Sitemap is an entity-escaped, UTF-8 encoded list of `<url>` elements contained
/// in a `<urlset>` element.
#[derive(Debug, PartialEq, Serialize)]
pub struct Sitemap {
    /// The set of URLs in the sitemap.
    pub urlset: Urlset,
}

impl Sitemap {
    /// Read a sitemap from a Reader.
    pub fn read_from<R: BufRead>(reader: R) -> Result<Sitemap, Error> {
        let mut reader = Reader::from_reader(reader);
        reader.trim_text(true).expand_empty_elements(true);

        let mut buf = Vec::new();
        let mut nested_buf = Vec::new();

        let mut sitemap = Sitemap {
            urlset: Urlset(vec![]),
        };
        let mut url = UrlEntry::new();
        let mut url_count: u32 = 0;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Decl(e)) => Self::check_encoding(e)?,
                Ok(Event::Start(start)) => {
                    if start.name().as_ref() == b"url" {
                        continue;
                    }

                    loop {
                        match reader.read_event_into(&mut nested_buf) {
                            Err(e) => {
                                panic!("Error at position {}: {:?}", reader.buffer_position(), e)
                            }
                            Ok(Event::Text(e)) => {
                                let text = e.unescape()?.to_string();
                                match start.name().as_ref() {
                                    b"loc" => url.loc.push_str(&text),
                                    b"lastmod" => url.last_mod = Some(W3CDateTime::new(&text)?),
                                    b"priority" => {
                                        url.priority = Some(Priority::new(text.parse()?)?)
                                    }
                                    b"changefreq" => {
                                        url.change_freq = Some(text.to_string().into())
                                    }
                                    _ => {}
                                }
                            }
                            _ => break,
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    if e.name().as_ref() == b"url" {
                        url_count += 1;

                        if url_count > 50_000 {
                            return Err(Error::TooManyUrls);
                        }

                        sitemap.urlset.0.push(url);
                        url = UrlEntry::new();
                    }
                }
                _ => {}
            }
            buf.clear();
        }
        Ok(sitemap)
    }

    fn write<W: Write>(&self, mut writer: Writer<W>) -> Result<W, Error> {
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

        let name = "urlset";
        let mut element = BytesStart::new(name);
        element.push_attribute(("xmlns", NAMESPACE));
        writer.write_event(Event::Start(element))?;

        for url_entry in &self.urlset.0 {
            let inner_name = "url";
            writer.write_event(Event::Start(BytesStart::new(inner_name)))?;

            Self::write_text_element(&mut writer, "loc", url_entry.loc.clone())?;

            if let Some(lastmod) = url_entry.last_mod {
                Self::write_text_element(&mut writer, "lastmod", lastmod.to_string())?;
            }

            if let Some(changefreq) = url_entry.change_freq {
                Self::write_text_element(&mut writer, "changefreq", changefreq.to_string())?;
            }

            if let Some(priority) = url_entry.priority {
                Self::write_text_element(&mut writer, "priority", priority.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new(inner_name)))?;
        }

        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(writer.into_inner())
    }

    /// Serialize a Sitemap to a Writer as XML.
    pub fn write_to<W: Write>(&self, writer: W) -> Result<W, Error> {
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
}

/// `<urlset>` is the XML root element. Here it is represented as a list of URLs.
#[derive(Debug, PartialEq, Serialize)]
pub struct Urlset(pub Vec<UrlEntry>);

/// The priority of this URL relative to other URLs on the site.
/// Valid values range from 0.0 to 1.0.
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Priority(pub f32);

/// A URL entry. It is a parent XML tag containing the required `<loc>` element
/// and the three optional `<lastmod>`, `<changrefreq>`, and `<priority>` elements.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct UrlEntry {
    /// The URL of the described page. It is required.
    pub loc: String,
    /// The optional date of last modification of the page.
    pub last_mod: Option<W3CDateTime>,
    /// Optional. How frequently the page is likely to change.
    pub change_freq: Option<ChangeFreq>,
    /// Optional. The priority of this URL relative to other URLs on the site.
    pub priority: Option<Priority>,
}

impl UrlEntry {
    /// Create a new, empty UrlEntry.
    pub fn new() -> Self {
        Self {
            loc: String::new(),
            last_mod: None,
            change_freq: None,
            priority: None,
        }
    }
}

impl Default for UrlEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl Priority {
    /// Create a new, valid Priority.
    pub fn new(priority: f32) -> Result<Self, Error> {
        if priority < 0.0 {
            return Err(Error::PriorityTooLow);
        }

        if priority > 1.0 {
            return Err(Error::PriorityTooHigh);
        }

        Ok(Self(priority))
    }
}

/// ChangeFreq represents how frequently the page is likely to change.
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub enum ChangeFreq {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

impl ChangeFreq {
    pub fn new(string: String) -> Self {
        Self::from(string)
    }
}

impl From<String> for ChangeFreq {
    fn from(value: String) -> Self {
        let normalized_value = value.to_lowercase();

        match normalized_value.as_ref() {
            "always" => ChangeFreq::Always,
            "hourly" => ChangeFreq::Hourly,
            "daily" => ChangeFreq::Daily,
            "weekly" => ChangeFreq::Weekly,
            "monthly" => ChangeFreq::Monthly,
            "yearly" => ChangeFreq::Yearly,
            "never" => ChangeFreq::Never,
            _ => panic!("Unrecognized change frequency"),
        }
    }
}

impl fmt::Display for ChangeFreq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match *self {
            ChangeFreq::Always => "always",
            ChangeFreq::Hourly => "hourly",
            ChangeFreq::Daily => "daily",
            ChangeFreq::Weekly => "weekly",
            ChangeFreq::Monthly => "monthly",
            ChangeFreq::Yearly => "yearly",
            ChangeFreq::Never => "never",
        };

        f.write_str(data)
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:.1}", &self.0))
    }
}
