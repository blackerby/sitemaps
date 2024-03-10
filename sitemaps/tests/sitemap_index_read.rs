extern crate sitemaps;

use sitemaps::error::Error;
use sitemaps::sitemap_index::SitemapIndex;
use sitemaps::SitemapRead;
use std::fs::File;

use std::io::BufReader;

#[test]
fn test_parse_sitemap_index() -> Result<(), Error> {
    let file = File::open("tests/data/sitemap_index.xml")?;
    let reader = BufReader::new(file);

    let sitemap_index = SitemapIndex::read_from(reader)?;

    assert_eq!(sitemap_index.namespace, sitemaps::NAMESPACE);
    assert!(sitemap_index.schema_location.is_none());
    assert!(sitemap_index.schema_instance.is_none());
    assert_eq!(sitemap_index.entries.len(), 2);
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
