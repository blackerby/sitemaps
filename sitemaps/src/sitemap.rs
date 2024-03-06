use crate::MAX_URL_LENGTH;
use core::fmt;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::reader::Reader;
use serde::Serialize;
use std::borrow::Cow;
use std::io::BufRead;
use url::Url;

use crate::{error::Error, w3c_datetime::W3CDateTime};

/// A Sitemap is an entity-escaped, UTF-8 encoded list of `<url>` elements contained in
/// in a `<urlset>` element.
#[derive(Debug, PartialEq, Serialize)]
pub struct Sitemap {
    /// The set of URLs in the sitemap.
    pub urlset: Urlset,
}

impl Sitemap {
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
        'outer: loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => {
                    break;
                }
                Ok(Event::Decl(e)) => match Self::check_encoding(e) {
                    Err(err) => return Err(err),
                    Ok(()) => {}
                },
                Ok(Event::Start(start)) => {
                    'inner: loop {
                        match reader.read_event_into(&mut nested_buf) {
                            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                            Ok(Event::Start(e)) => {
                                if e.name().as_ref() == b"url" {
                                    nested_buf.clear();
                                    buf.clear();
                                    continue 'outer;
                                }
                            }
                            Ok(Event::Text(e)) => {
                                let text = e.unescape()?.to_string();
                                match start.name().as_ref() {
                                    b"loc" => {
                                        url.loc.push_str(&text);
                                    }
                                    b"lastmod" => {
                                        url.last_mod = Some(W3CDateTime::parse(&text)?);
                                    }
                                    b"priority" => {
                                        let priority = Priority::new(text.parse()?)?;
                                        url.priority = Some(priority);
                                    }
                                    b"changefreq" => {
                                        url.change_freq = Some(text.to_string().into());
                                    }
                                    _ => {}
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
    
                                if e.name().as_ref() == b"urlset" {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }    
                },
                _ => {}
            }
            buf.clear();
        }
        println!("{:#?}", sitemap);
        Ok(sitemap)
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
