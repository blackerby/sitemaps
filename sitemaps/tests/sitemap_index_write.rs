extern crate sitemaps;

use sitemaps::error::Error;
use sitemaps::sitemap_index::SitemapIndex;
use sitemaps::SitemapRead;
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_write_sitemap_index_with_schema() -> Result<(), Error> {
    let expected = std::fs::read_to_string("tests/data/sitemap_index.xml")?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let file = File::open("tests/data/sitemap_index.xml")?;
    let reader = BufReader::new(file);
    let sitemap = SitemapIndex::read_from(reader)?;

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
