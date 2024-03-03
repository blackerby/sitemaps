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

fn pretty(headers: Vec<&str>, rows: Vec<Vec<String>>) -> String {
    let mut table = Table::new();

    table.set_header(headers);

    for row in rows {
        table.add_row(row);
    }

    format!("{table}")
}

fn plain(_headers: Vec<&str>, rows: Vec<Vec<String>>) -> String {
    let mut tw = TabWriter::new(vec![]);

    let lines = rows
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

fn build_rows(sitemap: Sitemap, cli: &Cli) -> (Vec<&str>, Vec<Vec<String>>) {
    let mut headers = vec![];
    let mut columns = vec![];

    if cli.loc {
        headers.push(HEADERS[0]);
        let locs = sitemap
            .urlset
            .0
            .iter()
            .map(|url| url.loc.to_string())
            .collect::<Vec<String>>();
        columns.push(locs);
    }
    if cli.lastmod && sitemap.urlset.0.iter().any(|url| url.last_mod.is_some()) {
        headers.push(HEADERS[1]);
        let lastmods = sitemap
            .urlset
            .0
            .iter()
            .map(|url| {
                if let Some(lastmod) = url.last_mod {
                    lastmod.to_string()
                } else {
                    String::new()
                }
            })
            .collect::<Vec<String>>();
        columns.push(lastmods);
    }
    if cli.changefreq && sitemap.urlset.0.iter().any(|url| url.change_freq.is_some()) {
        headers.push(HEADERS[2]);
        let changefreqs = sitemap
            .urlset
            .0
            .iter()
            .map(|url| {
                if let Some(changefreq) = url.change_freq {
                    changefreq.to_string()
                } else {
                    String::new()
                }
            })
            .collect::<Vec<String>>();
        columns.push(changefreqs);
    }
    if cli.priority && sitemap.urlset.0.iter().any(|url| url.priority.is_some()) {
        headers.push(HEADERS[3]);
        let priorities = sitemap
            .urlset
            .0
            .iter()
            .map(|url| {
                if let Some(priority) = url.priority {
                    priority.to_string()
                } else {
                    String::new()
                }
            })
            .collect::<Vec<String>>();
        columns.push(priorities);
    }

    // TODO: get rid of clone
    let rows = (0..columns.len())
        .map(|i| {
            columns
                .iter()
                .map(|c| c[i].clone())
                .collect::<Vec<String>>()
        })
        .collect();

    (headers, rows)
}
