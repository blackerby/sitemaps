extern crate sitemaps;

use sitemaps::error::Error;
use sitemaps::sitemap::{ChangeFreq, Priority, Sitemap};
use sitemaps::w3c_datetime::W3CDateTime;
use sitemaps::{SitemapRead, NAMESPACE};
use std::fs::File;
use std::io::BufReader;

use chrono::prelude::*;

#[test]
fn test_parse_one_happy() -> Result<(), Error> {
    let file = File::open("tests/data/example_1_url.xml")?;
    let reader = BufReader::new(file);

    let sitemap = Sitemap::read_from(reader)?;

    assert_eq!(sitemap.urlset.namespace, sitemaps::NAMESPACE);
    assert_eq!(sitemap.urlset.urls.len(), 1);
    assert_eq!(
        sitemap.urlset.urls[0].loc.to_string(),
        "http://www.example.com/"
    );
    assert_eq!(
        sitemap.urlset.urls[0].last_mod,
        Some(W3CDateTime::Date("2005-01-01".parse::<NaiveDate>()?))
    );
    assert_eq!(
        sitemap.urlset.urls[0].change_freq,
        Some(ChangeFreq::Monthly)
    );
    assert_eq!(sitemap.urlset.urls[0].priority, Some(Priority(0.8)));

    Ok(())
}

#[test]
fn test_parse_two_happy() -> Result<(), Error> {
    let file = File::open("tests/data/example_2_url.xml")?;
    let reader = BufReader::new(file);

    let sitemap = Sitemap::read_from(reader)?;

    assert_eq!(sitemap.urlset.urls.len(), 2);
    assert_eq!(
        sitemap.urlset.urls[1].loc.to_string(),
        "http://www.examples.com/"
    );
    assert_eq!(
        sitemap.urlset.urls[1].last_mod,
        Some(W3CDateTime::Date("2006-01-01".parse::<NaiveDate>()?))
    );
    assert_eq!(sitemap.urlset.urls[1].change_freq, Some(ChangeFreq::Weekly));
    assert_eq!(sitemap.urlset.urls[1].priority, Some(Priority(0.5)));

    Ok(())
}

#[test]
fn test_parse_external_happy() -> Result<(), Error> {
    let url = "https://www.govinfo.gov/sitemap/bulkdata/PLAW/117pvtl/sitemap.xml";

    let content = ureq::get(url).call().unwrap().into_reader();
    let reader = BufReader::new(content);

    let sitemap = Sitemap::read_from(reader).unwrap();

    assert_eq!(sitemap.urlset.urls.len(), 3);
    assert_eq!(
        sitemap.urlset.urls[1].change_freq,
        Some(ChangeFreq::Monthly)
    );
    assert_eq!(
        sitemap.urlset.urls[1].loc,
        "https://www.govinfo.gov/bulkdata/PLAW/117/private/PLAW-117pvtl2.xml"
    );

    assert_eq!(sitemap.urlset.namespace, NAMESPACE);
    assert!(sitemap.urlset.schema_location.is_some());
    assert!(sitemap.urlset.schema_instance.is_some());

    Ok(())
}
