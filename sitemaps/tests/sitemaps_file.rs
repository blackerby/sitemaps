extern crate sitemaps;

use chrono::prelude::*;
use sitemaps::error::Error;
use sitemaps::sitemap::{ChangeFreq, Priority};
use sitemaps::w3c_datetime::W3CDateTime;
use sitemaps::SitemapsFile;
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_parse_sitemap() -> Result<(), Error> {
    let file = File::open("tests/data/example_1_url.xml")?;
    let reader = BufReader::new(file);

    let sitemap = match SitemapsFile::read(reader)? {
        SitemapsFile::Sitemap(sitemap) => sitemap,
        _ => unreachable!(),
    };
    assert_eq!(sitemap.namespace, sitemaps::NAMESPACE);
    assert_eq!(sitemap.entries.len(), 1);
    assert_eq!(
        sitemap.entries[0].loc.to_string(),
        "http://www.example.com/"
    );
    assert_eq!(
        sitemap.entries[0].last_mod,
        Some(W3CDateTime::Date("2005-01-01".parse::<NaiveDate>()?))
    );
    assert_eq!(sitemap.entries[0].change_freq, Some(ChangeFreq::Monthly));
    assert_eq!(sitemap.entries[0].priority, Some(Priority(0.8)));
    Ok(())
}

#[test]
fn test_parse_sitemap_index() -> Result<(), Error> {
    let file = File::open("tests/data/sitemap_index.xml")?;
    let reader = BufReader::new(file);

    let sitemap_index = match SitemapsFile::read(reader)? {
        SitemapsFile::SiteIndex(sitemapindex) => sitemapindex,
        _ => unreachable!(),
    };
    assert_eq!(sitemap_index.namespace, sitemaps::NAMESPACE);
    assert_eq!(sitemap_index.entries.len(), 2);
    assert_eq!(
        sitemap_index.entries[0].loc.to_string(),
        "http://www.example.com/sitemap1.xml.gz"
    );
    assert_eq!(
        sitemap_index.entries[0].last_mod,
        Some(W3CDateTime::DateTime(
            "2004-10-01T18:23:17+00:00".parse::<DateTime<FixedOffset>>()?,
            false,
            false
        ))
    );
    Ok(())
}
