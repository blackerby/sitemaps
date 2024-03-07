extern crate sitemaps;

use sitemaps::error::Error;
use sitemaps::sitemap::Sitemap;
use std::fs::File;
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
