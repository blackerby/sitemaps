pub mod cli;

use std::io::Write;

use crate::cli::Cli;
use clap::Parser;
use comfy_table::Table;
use sitemaps::error::SitemapError;
use sitemaps::reader::SitemapReader;
use sitemaps::sitemap::Sitemap;
use tabwriter::TabWriter;

fn main() -> Result<(), SitemapError> {
    let cli = Cli::parse();

    if let Some(ref path) = cli.path {
        let sitemap = SitemapReader::read(&path)?;

        if cli.pretty {
            let table = build_table(sitemap);

            print!("{table}");
        } else {
            let output = build_buffer(sitemap, &cli);

            print!("{output}");
        }
    }

    Ok(())
}

fn build_buffer(sitemap: Sitemap, cli: &Cli) -> String {
    let mut tw = TabWriter::new(vec![]);

    let mut lines = String::new();

    for url in &sitemap.urlset.0 {
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

        lines.push('\n');
    }

    tw.write_all(lines.as_bytes()).unwrap();
    tw.flush().unwrap();

    String::from_utf8(tw.into_inner().unwrap()).unwrap()
}

fn build_table(sitemap: Sitemap) -> Table {
    let mut table = Table::new();
    table.set_header(vec!["loc", "lastmod", "changefreq", "priority"]);

    for url in sitemap.urlset.0 {
        table.add_row(vec![
            url.loc,
            url.last_mod.unwrap().to_string(),
            url.change_freq.unwrap().to_string(),
            url.priority.unwrap().to_string(),
        ]);
    }

    table
}
