use crate::error::Error;
use crate::sitemap::{Priority, Sitemap, Url, Urlset};
use crate::w3c_datetime::W3CDateTime;
use crate::MAX_URL_LENGTH;
use std::borrow::Cow;
use std::io::Read;
use std::{fs, io};

use quick_xml::events::{BytesDecl, Event};
use quick_xml::reader::Reader;
use ureq;
use url::Url as WebUrl;

pub struct SitemapReader<'a> {
    pub path: &'a str,
    pub contents: String,
}

impl<'a> SitemapReader<'a> {
    fn new(path: &'a str) -> Result<SitemapReader, Error> {
        Ok(Self {
            path,
            contents: if let Ok(_) = WebUrl::parse(&path) {
                ureq::get(&path).call()?.into_string()?
            } else if path == "-" {
                let mut buf = String::new();
                io::stdin().read_to_string(&mut buf)?;
                buf
            } else {
                fs::read_to_string(&path)?
            },
        })
    }

    pub fn read(path: &'a str) -> Result<Sitemap, Error> {
        let reader = Self::new(path)?;

        reader.parse()
    }

    fn parse(&self) -> Result<Sitemap, Error> {
        let mut sitemap = Sitemap {
            urlset: Urlset(vec![]),
        };
        let mut url = Url::new();
        let mut url_count: u32 = 0;
        let mut reader = Reader::from_str(&self.contents);
        loop {
            match reader.read_event() {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => {
                    break;
                }
                Ok(Event::Decl(e)) => match Self::check_encoding(e) {
                    Err(err) => return Err(err),
                    Ok(()) => {}
                },
                Ok(Event::Start(start)) => {
                    let next_event = reader.read_event()?;
                    if let Event::Text(e) = next_event {
                        let text = e.unescape()?.to_string();
                        match start.name().as_ref() {
                            b"loc" => {
                                url.loc = Self::validate_url(&text)?.to_string();
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
                }
                Ok(Event::End(e)) => {
                    if e.name().as_ref() == b"url" {
                        url_count += 1;

                        if url_count > 50_000 {
                            return Err(Error::TooManyUrls);
                        }

                        sitemap.urlset.0.push(url);
                        url = Url::new();
                        continue;
                    }
                }
                _ => {}
            }
        }
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

    fn validate_url(string: &str) -> Result<String, Error> {
        if string.chars().count() > MAX_URL_LENGTH {
            return Err(Error::UrlValueTooLong);
        }

        let url = WebUrl::parse(string)?;

        Ok(url.as_str().into())
    }
}

#[cfg(test)]
mod tests {
    const ONE_URL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
       <url>
          <loc>http://www.example.com/</loc>
          <lastmod>2005-01-01</lastmod>
          <changefreq>monthly</changefreq>
          <priority>0.8</priority>
       </url>
    </urlset>"#;

    const TWO_URLS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
       <url>
          <loc>http://www.example.com/</loc>
          <lastmod>2005-01-01</lastmod>
          <changefreq>monthly</changefreq>
          <priority>0.8</priority>
       </url>
       <url>
          <loc>http://www.examples.com/</loc>
          <lastmod>2006-01-01</lastmod>
          <changefreq>weekly</changefreq>
          <priority>0.5</priority>
       </url>
    </urlset>"#;

    use super::*;
    use crate::sitemap::ChangeFreq;
    use chrono::prelude::*;

    #[test]
    fn test_parse_one_happy() -> Result<(), Error> {
        let reader = SitemapReader {
            path: "",
            contents: ONE_URL.to_string(),
        };

        let sitemap = reader.parse()?;

        assert_eq!(sitemap.urlset.0.len(), 1);
        assert_eq!(
            sitemap.urlset.0[0].loc.to_string(),
            "http://www.example.com/"
        );
        assert_eq!(
            sitemap.urlset.0[0].last_mod,
            Some(W3CDateTime::Date("2005-01-01".parse::<NaiveDate>()?))
        );
        assert_eq!(sitemap.urlset.0[0].change_freq, Some(ChangeFreq::Monthly));
        assert_eq!(sitemap.urlset.0[0].priority, Some(Priority(0.8)));

        Ok(())
    }

    #[test]
    fn test_parse_two_happy() -> Result<(), Error> {
        let reader = SitemapReader {
            path: "",
            contents: TWO_URLS.to_string(),
        };

        let sitemap = reader.parse()?;

        assert_eq!(sitemap.urlset.0.len(), 2);
        assert_eq!(
            sitemap.urlset.0[1].loc.to_string(),
            "http://www.examples.com/"
        );
        assert_eq!(
            sitemap.urlset.0[1].last_mod,
            Some(W3CDateTime::Date("2006-01-01".parse::<NaiveDate>()?))
        );
        assert_eq!(sitemap.urlset.0[1].change_freq, Some(ChangeFreq::Weekly));
        assert_eq!(sitemap.urlset.0[1].priority, Some(Priority(0.5)));

        Ok(())
    }

    #[test]
    fn test_parse_external_happy() -> Result<(), Error> {
        let url = "https://www.govinfo.gov/sitemap/bulkdata/PLAW/117pvtl/sitemap.xml";

        let sitemap = SitemapReader::read(url)?;

        assert_eq!(sitemap.urlset.0.len(), 3);

        Ok(())
    }
}
