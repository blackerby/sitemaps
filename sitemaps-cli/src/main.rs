pub mod cli;

use std::io::Write;

use crate::cli::Cli;
use clap::Parser;
use comfy_table::Table;
use sitemaps::error::SitemapError;
use sitemaps::reader::SitemapReader;
use sitemaps::sitemap::Sitemap;
use tabwriter::TabWriter;

const HEADER_COUNT: usize = 4;

fn main() -> Result<(), SitemapError> {
    let cli = Cli::parse();

    if let Some(ref path) = cli.path {
        let sitemap = SitemapReader::read(&path)?;

        let output = build_output(sitemap, &cli);

        println!("{output}");
    }

    Ok(())
}

// TODO: get rid of all unwraps!
// don't print column if None
fn build_output(sitemap: Sitemap, cli: &Cli) -> String {
    let mut header = vec![];
    let mut header_flags = [false; HEADER_COUNT];
    let headers = ["loc", "lastmod", "changefreq", "priority"];
    let mut rows = vec![];

    for url in sitemap.urlset.0 {
        let mut row = vec![];
        if cli.loc {
            header_flags[0] = true;
            row.push(url.loc.to_string());
        }

        if cli.lastmod {
            if let Some(lastmod) = url.last_mod {
                header_flags[1] = true;
                row.push(lastmod.to_string());
            }
        }

        if cli.changefreq {
            if let Some(changefreq) = url.change_freq {
                header_flags[2] = true;
                row.push(changefreq.to_string());
            }
        }

        if cli.priority {
            if let Some(priority) = url.priority {
                header_flags[3] = true;
                row.push(priority.to_string());
            }
        }

        rows.push(row);
    }

    for (i, &flag) in header_flags.iter().enumerate() {
        if flag {
            header.push(headers[i]);
        }
    }

    if cli.pretty {
        let mut table = Table::new();

        table.set_header(header);

        for row in rows {
            table.add_row(row);
        }

        format!("{table}")
    } else {
        let mut tw = TabWriter::new(vec![]);

        let lines = rows
            .iter()
            .map(|row| row.join("\t"))
            .collect::<Vec<String>>();
        let buf = lines.join("\n");

        tw.write_all(buf.as_bytes()).unwrap();
        tw.flush().unwrap();

        String::from_utf8(tw.into_inner().unwrap()).unwrap()
    }
}
