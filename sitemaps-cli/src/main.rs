pub mod cli;

use crate::cli::Cli;
use clap::Parser;
use comfy_table::Table;
use sitemaps::error::SitemapError;
use sitemaps::reader::SitemapReader;

fn main() -> Result<(), SitemapError> {
    let cli = Cli::parse();

    if let Some(path) = cli.path {
        let sitemap = SitemapReader::read(&path)?;
        let mut table = Table::new();
        table.set_header(vec!["loc", "lastmod", "changefreq", "priority"]);

        for url in sitemap.urlset.0 {
            let mut line = String::from(&url.loc);

            if cli.lastmod {
                line.push_str(&format!("\t{}\t", url.last_mod.unwrap()));
            }

            if cli.changefreq {
                line.push_str(&format!("{}\t", url.change_freq.unwrap()));
            }

            if cli.priority {
                line.push_str(&format!("{}", url.priority.unwrap()));
            }

            table.add_row(vec![
                url.loc,
                url.last_mod.unwrap().to_string(),
                url.change_freq.unwrap().to_string(),
                url.priority.unwrap().to_string(),
            ]);

            println!("{line}");
        }

        // println!("{table}");
    }

    Ok(())
}
