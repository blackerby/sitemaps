use crate::NAMESPACE;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer};
use serde::Serialize;
use std::io::prelude::*;
use std::io::BufRead;

use crate::error::Error;
use crate::w3c_datetime::W3CDateTime;
use crate::SitemapRead;

#[derive(Debug, PartialEq, Serialize)]
pub struct SitemapIndex {
    pub entries: Vec<SitemapEntry>,
    pub schema_instance: Option<String>,
    pub schema_location: Option<String>,
    pub namespace: String,
}

impl SitemapIndex {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            schema_location: None,
            schema_instance: None,
            namespace: String::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SitemapEntry {
    pub loc: String,
    pub last_mod: Option<W3CDateTime>,
}

impl SitemapEntry {
    pub fn new() -> SitemapEntry {
        Self {
            loc: String::new(),
            last_mod: None,
        }
    }
}

impl SitemapRead for SitemapIndex {
    fn read_from<R: BufRead>(reader: R) -> Result<Self, Error> {
        let mut reader = Reader::from_reader(reader);
        reader.trim_text(true).expand_empty_elements(true);

        let mut buf = Vec::new();
        let mut nested_buf = Vec::new();
        let mut sitemap_index = SitemapIndex::new();

        let mut entry = SitemapEntry::new();
        let mut entry_count: u32 = 0;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Decl(e)) => Self::check_encoding(e)?,
                Ok(Event::Start(start)) => {
                    if start.name().as_ref() == b"sitemapindex" {
                        for attr_result in start.attributes() {
                            let a = attr_result?;
                            match a.key.as_ref() {
                                b"xmlns:xsi" => {
                                    sitemap_index.schema_instance =
                                        Some(a.decode_and_unescape_value(&reader)?.to_string());
                                }
                                b"xsi:schemaLocation" => {
                                    sitemap_index.schema_location =
                                        Some(a.decode_and_unescape_value(&reader)?.to_string());
                                }
                                b"xmlns" => {
                                    sitemap_index.namespace =
                                        a.decode_and_unescape_value(&reader)?.to_string();
                                }
                                _ => {}
                            }
                        }
                    }

                    if start.name().as_ref() == b"sitemap" {
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
                                    b"loc" => entry.loc.push_str(&text),
                                    b"lastmod" => entry.last_mod = Some(W3CDateTime::new(&text)?),
                                    _ => {}
                                }
                            }
                            _ => break,
                        }
                    }
                }

                Ok(Event::End(e)) => {
                    if e.name().as_ref() == b"sitemap" {
                        entry_count += 1;

                        if entry_count > 50_000 {
                            return Err(Error::TooManyUrls);
                        }

                        sitemap_index.entries.push(entry);
                        entry = SitemapEntry::new();
                    }
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(sitemap_index)
    }

    fn write<W: Write>(&self, mut writer: Writer<W>) -> Result<W, Error> {
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

        let name = "sitemapindex";
        let mut element = BytesStart::new(name);
        if let Some(ref schema_instance) = self.schema_instance {
            element.push_attribute(("xmlns:xsi", schema_instance.as_str()));
        }
        if let Some(ref schema_location) = self.schema_location {
            element.push_attribute(("xsi:schemaLocation", schema_location.as_str()));
        }
        let namespace = if self.namespace.is_empty() {
            NAMESPACE
        } else {
            self.namespace.as_str()
        };
        element.push_attribute(("xmlns", namespace));
        writer.write_event(Event::Start(element))?;

        for entry in &self.entries {
            let inner_name = "sitemap";
            writer.write_event(Event::Start(BytesStart::new(inner_name)))?;

            Self::write_text_element(&mut writer, "loc", entry.loc.clone())?;

            if let Some(lastmod) = entry.last_mod {
                Self::write_text_element(&mut writer, "lastmod", lastmod.to_string())?;
            }

            writer.write_event(Event::End(BytesEnd::new(inner_name)))?;
        }

        writer.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(writer.into_inner())
    }
}
