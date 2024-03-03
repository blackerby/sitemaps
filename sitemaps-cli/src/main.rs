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

struct Headers(Vec<&'static str>);
struct Row(Vec<String>);
struct Rows(Vec<Row>);

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
    let (headers, rows) = build_rows(sitemap, cli);

    if cli.pretty {
        pretty(headers, rows)
    } else {
        plain(headers, rows)
    }
}

fn pretty(headers: Headers, rows: Rows) -> String {
    let mut table = Table::new();

    table.set_header(headers.0);

    for row in rows.0 {
        table.add_row(row.0);
    }

    format!("{table}")
}

fn plain(_headers: Headers, rows: Rows) -> String {
    let mut tw = TabWriter::new(vec![]);

    let lines = rows
        .0
        .iter()
        .map(|row| row.0.join("\t"))
        .collect::<Vec<String>>();
    let buf = lines.join("\n");

    // I am skeptical of these unwraps, but I think the logic used in
    // `build_rows` might prevent panicking
    tw.write_all(buf.as_bytes()).unwrap();
    tw.flush().unwrap();

    String::from_utf8(tw.into_inner().unwrap()).unwrap()
}

// TODO: reexamine the logic here. Checking the Cli flags and setting
// the header flags on each URL seems inefficient
fn build_rows(sitemap: Sitemap, cli: &Cli) -> (Headers, Rows) {
    let mut rows = vec![];

    let mut headers = vec![];

    if cli.loc {
        headers.push(HEADERS[0]);
    }
    if cli.lastmod && sitemap.urlset.0.iter().any(|url| url.last_mod.is_some()) {
        headers.push(HEADERS[1]);
    }
    if cli.changefreq && sitemap.urlset.0.iter().any(|url| url.change_freq.is_some()) {
        headers.push(HEADERS[2]);
    }
    if cli.priority && sitemap.urlset.0.iter().any(|url| url.priority.is_some()) {
        headers.push(HEADERS[3]);
    }

    for url in sitemap.urlset.0 {
        let mut row = Row(vec![]);

        if cli.loc {
            row.0.push(url.loc.to_string());
        }

        if cli.lastmod {
            if let Some(lastmod) = url.last_mod {
                row.0.push(lastmod.to_string());
            }
        }

        if cli.changefreq {
            if let Some(changefreq) = url.change_freq {
                row.0.push(changefreq.to_string());
            }
        }

        if cli.priority {
            if let Some(priority) = url.priority {
                row.0.push(priority.to_string());
            }
        }

        rows.push(row);
    }

    (Headers(headers), Rows(rows))
}
