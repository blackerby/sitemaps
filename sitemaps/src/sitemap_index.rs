use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::Reader;
use serde::Serialize;
use std::io::prelude::*;
use std::io::BufRead;

use crate::error::Error;
use crate::w3c_datetime::W3CDateTime;
use crate::SitemapRead;

#[derive(Debug, PartialEq, Serialize)]
pub struct SitemapIndex(Vec<SitemapEntry>);

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
        let mut sitemap_entries = Vec::new();

        let mut entry = SitemapEntry::new();
        let mut entry_count: u32 = 0;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Decl(e)) => Self::check_encoding(e)?,
                Ok(Event::Start(start)) => {
                    if start.name().as_ref() == b"urlset" {
                        for attr_result in start.attributes() {
                            let a = attr_result?;
                            match a.key.as_ref() {
                                b"xmlns:xsi" => {
                                    sitemap.urlset.schema_instance =
                                        Some(a.decode_and_unescape_value(&reader)?.to_string());
                                }
                                b"xsi:schemaLocation" => {
                                    sitemap.urlset.schema_location =
                                        Some(a.decode_and_unescape_value(&reader)?.to_string());
                                }
                                b"xmlns" => {
                                    sitemap.urlset.namespace =
                                        a.decode_and_unescape_value(&reader)?.to_string();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    fn write<W: Write>(&self, writer: quick_xml::Writer<W>) -> Result<W, Error> {}
