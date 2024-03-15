extern crate sitemaps;

use sitemaps::error::Error;
use sitemaps::siteindex::SiteIndex;
use sitemaps::Sitemaps;
use std::fs::File;

use std::io::BufReader;

#[test]
fn test_parse_sitemap_index() -> Result<(), Error> {
    let file = File::open("tests/data/sitemap_index.xml")?;
    let reader = BufReader::new(file);

    let sitemap_index = SiteIndex::read_from(reader)?;

    assert_eq!(sitemap_index.namespace, sitemaps::NAMESPACE);
    assert!(sitemap_index.schema_location.is_none());
    assert!(sitemap_index.schema_instance.is_none());
    assert_eq!(sitemap_index.entries.len(), 2);
    assert_eq!(
        sitemap_index.entries[0].loc,
        "http://www.example.com/sitemap1.xml.gz"
    );
    assert_eq!(
        sitemap_index.entries[1].last_mod.unwrap().to_string(),
        String::from("2005-01-01")
    );
    assert_eq!(
        sitemap_index.entries[0].last_mod.unwrap().to_string(),
        String::from("2004-10-01T18:23:17+00:00")
    );

    Ok(())
}

#[test]
fn test_write_sitemap_index_with_schema() -> Result<(), Error> {
    let expected = std::fs::read_to_string("tests/data/sitemap_index.xml")?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let file = File::open("tests/data/sitemap_index.xml")?;
    let reader = BufReader::new(file);
    let sitemap = SiteIndex::read_from(reader)?;

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
