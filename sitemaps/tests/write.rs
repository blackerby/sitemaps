extern crate sitemaps;

use sitemaps::error::Error;
use sitemaps::sitemap::{ChangeFreq, Priority, Sitemap, UrlEntry};
use sitemaps::w3c_datetime::W3CDateTime;
use sitemaps::SitemapRead;
use std::fs::{self, File};
use std::io::BufReader;

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
    sitemap.urls = urls;

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
