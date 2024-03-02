pub mod cli;

use std::io::Write;

use crate::cli::Cli;
use clap::Parser;
use comfy_table::Table;
use sitemaps::error::SitemapError;
use sitemaps::reader::SitemapReader;
use sitemaps::sitemap::Sitemap;
use tabwriter::TabWriter;

const HEADERS: [&str; 4] = ["loc", "lastmod", "changefreq", "priority"];

struct Headers([bool; HEADERS.len()]);
struct Rows(Vec<Vec<String>>);

fn main() -> Result<(), SitemapError> {
    let cli = Cli::parse();

    if let Some(ref path) = cli.path {
        let sitemap = SitemapReader::read(&path)?;

        let output = build_output(sitemap, &cli);

        println!("{output}");
    }

    Ok(())
}

fn build_output(sitemap: Sitemap, cli: &Cli) -> String {
    let mut header = vec![];

    let (header_flags, rows) = build_rows(sitemap, cli);

    for (i, &flag) in header_flags.0.iter().enumerate() {
        if flag {
            header.push(HEADERS[i]);
        }
    }

    if cli.pretty {
        pretty(header, rows)
    } else {
        plain(header, rows)
    }
}

fn pretty(header: Vec<&str>, rows: Rows) -> String {
    let mut table = Table::new();

    table.set_header(header);

    for row in rows.0 {
        table.add_row(row);
    }

    format!("{table}")
}

fn plain(_header: Vec<&str>, rows: Rows) -> String {
    let mut tw = TabWriter::new(vec![]);

    let lines = rows
        .0
        .iter()
        .map(|row| row.join("\t"))
        .collect::<Vec<String>>();
    let buf = lines.join("\n");

    // I am skeptical of these unwraps, but I think the logic used in
    // `build_rows` might prevent panicking
    tw.write_all(buf.as_bytes()).unwrap();
    tw.flush().unwrap();

    String::from_utf8(tw.into_inner().unwrap()).unwrap()
}

fn build_rows(sitemap: Sitemap, cli: &Cli) -> (Headers, Rows) {
    let mut header_flags = Headers([false; HEADERS.len()]);
    let mut rows = vec![];

    for url in sitemap.urlset.0 {
        let mut row = vec![];
        if cli.loc {
            header_flags.0[0] = true;
            row.push(url.loc.to_string());
        }

        if cli.lastmod {
            if let Some(lastmod) = url.last_mod {
                header_flags.0[1] = true;
                row.push(lastmod.to_string());
            }
        }

        if cli.changefreq {
            if let Some(changefreq) = url.change_freq {
                header_flags.0[2] = true;
                row.push(changefreq.to_string());
            }
        }

        if cli.priority {
            if let Some(priority) = url.priority {
                header_flags.0[3] = true;
                row.push(priority.to_string());
            }
        }

        rows.push(row);
    }

    (header_flags, Rows(rows))
}
