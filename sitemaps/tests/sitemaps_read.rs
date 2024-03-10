extern crate sitemaps;

use chrono::prelude::*;
use sitemaps::error::Error;
use sitemaps::sitemap::{ChangeFreq, Priority};
use sitemaps::w3c_datetime::W3CDateTime;
use sitemaps::Sitemaps;
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_parse_sitemap() -> Result<(), Error> {
    let file = File::open("tests/data/example_1_url.xml")?;
    let reader = BufReader::new(file);

    let sitemap = match Sitemaps::read(reader)? {
        Sitemaps::Sitemap(sitemap) => sitemap,
        _ => unreachable!(),
    };
    println!("{:#?}", sitemap);
    assert_eq!(sitemap.namespace, sitemaps::NAMESPACE);
    assert_eq!(sitemap.urls.len(), 1);
    assert_eq!(sitemap.urls[0].loc.to_string(), "http://www.example.com/");
    assert_eq!(
        sitemap.urls[0].last_mod,
        Some(W3CDateTime::Date("2005-01-01".parse::<NaiveDate>()?))
    );
    assert_eq!(sitemap.urls[0].change_freq, Some(ChangeFreq::Monthly));
    assert_eq!(sitemap.urls[0].priority, Some(Priority(0.8)));
    Ok(())
}
