extern crate sitemaps;

use sitemaps::error::Error;
use sitemaps::sitemap::{ChangeFreq, Priority, Sitemap, UrlEntry};
use sitemaps::w3c_datetime::W3CDateTime;
use sitemaps::{Sitemaps, NAMESPACE};
use std::fs::{self, File};
use std::io::BufReader;

use chrono::prelude::*;

const EXPECTED: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
                        <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\"> 
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
                        </urlset>";

#[test]
fn test_parse_one_happy() -> Result<(), Error> {
    let file = File::open("tests/data/example_1_url.xml")?;
    let reader = BufReader::new(file);

    let sitemap = Sitemap::read_from(reader)?;

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
fn test_parse_two_happy() -> Result<(), Error> {
    let file = File::open("tests/data/example_2_url.xml")?;
    let reader = BufReader::new(file);

    let sitemap = Sitemap::read_from(reader)?;

    assert_eq!(sitemap.entries.len(), 2);
    assert_eq!(
        sitemap.entries[1].loc.to_string(),
        "http://www.examples.com/"
    );
    assert_eq!(
        sitemap.entries[1].last_mod,
        Some(W3CDateTime::Date("2006-01-01".parse::<NaiveDate>()?))
    );
    assert_eq!(sitemap.entries[1].change_freq, Some(ChangeFreq::Weekly));
    assert_eq!(sitemap.entries[1].priority, Some(Priority(0.5)));

    Ok(())
}

#[test]
fn test_parse_external_happy() -> Result<(), Error> {
    let url = "https://www.govinfo.gov/sitemap/bulkdata/PLAW/117pvtl/sitemap.xml";

    let content = ureq::get(url).call().unwrap().into_reader();
    let reader = BufReader::new(content);

    let sitemap = Sitemap::read_from(reader).unwrap();

    assert_eq!(sitemap.entries.len(), 3);
    assert_eq!(sitemap.entries[1].change_freq, Some(ChangeFreq::Monthly));
    assert_eq!(
        sitemap.entries[1].loc,
        "https://www.govinfo.gov/bulkdata/PLAW/117/private/PLAW-117pvtl2.xml"
    );

    assert_eq!(sitemap.namespace, NAMESPACE);
    assert!(sitemap.schema_location.is_some());
    assert!(sitemap.schema_instance.is_some());

    Ok(())
}

#[test]
fn test_write_two_happy() -> Result<(), Error> {
    let file = File::open("tests/data/example_2_url.xml")?;
    let reader = BufReader::new(file);

    let sitemap = Sitemap::read_from(reader)?;

    let mut buf = Vec::new();
    let written = sitemap.write_to(&mut buf)?;

    assert_eq!(
        EXPECTED
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        std::str::from_utf8(&written[..])
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    );
    Ok(())
}

#[test]
fn test_new_sitemap() -> Result<(), Error> {
    let mut urls = vec![];

    let mut url_entry = UrlEntry::new();
    url_entry.loc = String::from("http://www.example.com/");
    url_entry.last_mod = Some(W3CDateTime::new("2005-01-01")?);
    url_entry.change_freq = Some(ChangeFreq::new(String::from("monthly")));
    url_entry.priority = Some(Priority(0.8));

    urls.push(url_entry);

    let mut sitemap = Sitemap::new();
    sitemap.entries = urls;

    let mut buf = Vec::new();
    let written = sitemap.write_to(&mut buf)?;
    let result = std::str::from_utf8(written)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let expected = fs::read_to_string("tests/data/example_1_url.xml")?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_write_with_schema() -> Result<(), Error> {
    let expected = std::fs::read_to_string("tests/data/sitemap.xml")?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let file = File::open("tests/data/sitemap.xml")?;
    let reader = BufReader::new(file);
    let sitemap = Sitemap::read_from(reader)?;

    let mut buf = Vec::new();
    let written = sitemap.write_to(&mut buf)?;
    let result = std::str::from_utf8(written)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    assert_eq!(result, expected);
    Ok(())
}
