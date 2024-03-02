pub mod cli;

use std::io::Write;

use crate::cli::Cli;
use clap::Parser;
use comfy_table::Table;
use sitemaps::error::SitemapError;
use sitemaps::reader::SitemapReader;
use tabwriter::TabWriter;

fn main() -> Result<(), SitemapError> {
    let cli = Cli::parse();

    if let Some(path) = cli.path {
        let sitemap = SitemapReader::read(&path)?;
        let mut table = Table::new();
        table.set_header(vec!["loc", "lastmod", "changefreq", "priority"]);
        let mut lines = String::new();

        let mut tw = TabWriter::new(vec![]);

        for url in sitemap.urlset.0 {
            lines.push_str(&url.loc.to_string());

            if cli.lastmod {
                lines.push_str(&format!("\t{}\t", url.last_mod.unwrap()));
            }

            if cli.changefreq {
                lines.push_str(&format!("{}\t", url.change_freq.unwrap()));
            }

            if cli.priority {
                lines.push_str(&format!("{}\t", url.priority.unwrap()));
            }

            table.add_row(vec![
                url.loc,
                url.last_mod.unwrap().to_string(),
                url.change_freq.unwrap().to_string(),
                url.priority.unwrap().to_string(),
            ]);
            lines.push('\n');
        }

        tw.write_all(lines.as_bytes()).unwrap();
        tw.flush().unwrap();
        let output = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        print!("{output}");

        println!("{table}");
    }

    Ok(())
}
