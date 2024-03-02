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

        if cli.pretty {
            let table = build_table(sitemap, &cli);

            println!("{table}");
        } else {
            let output = build_buffer(sitemap, &cli);

            print!("{output}");
        }
    }

    Ok(())
}

// TODO: get rid of all unwraps!
// don't print column if None
fn build_buffer(sitemap: Sitemap, cli: &Cli) -> String {
    let mut tw = TabWriter::new(vec![]);

    let mut lines = String::new();

    for url in &sitemap.urlset.0 {
        lines.push_str(&url.loc.to_string());
        lines.push('\t');

        if cli.lastmod {
            lines.push_str(&format!("{}\t", url.last_mod.unwrap()));
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

fn build_table(sitemap: Sitemap, cli: &Cli) -> Table {
    let mut table = Table::new();
    let mut header = vec![];
    // table headers for pretty printing.
    // from left to right: loc, lastmod, changefreq, priority
    let mut header_flags = [false; HEADER_COUNT];
    let headers = ["loc", "lastmod", "changefreq", "priority"];

    for url in &sitemap.urlset.0 {
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

        table.add_row(row);
    }

    for (i, &flag) in header_flags.iter().enumerate() {
        if flag {
            header.push(headers[i]);
        }
    }

    table.set_header(header);

    table
}

// fn build_output(sitemap: Sitemap, cli: &Cli) -> String {
//     let mut header = vec![];
//     let mut header_flags = [false; HEADER_COUNT];
//     let headers = ["loc", "lastmod", "changefreq", "priority"];
//     let mut rows = vec![];

//     for url in sitemap.urlset.0 {
//         let mut row = vec![];
//         if cli.loc {
//             header_flags[0] = true;
//             row.push(url.loc.as_bytes());
//         }

//         if cli.lastmod {
//             if let Some(lastmod) = url.last_mod {
//                 header_flags[1] = true;
//                 row.push(lastmod.to_string().as_bytes());
//             }
//         }

//         if cli.changefreq {
//             if let Some(changefreq) = url.change_freq {
//                 header_flags[2] = true;
//                 row.push(changefreq.to_string().as_bytes());
//             }
//         }

//         if cli.priority {
//             if let Some(priority) = url.priority {
//                 header_flags[3] = true;
//                 row.push(priority.to_string().as_bytes());
//             }
//         }

//         rows.push(row);
//     }

//     for (i, &flag) in header_flags.iter().enumerate() {
//         if flag {
//             header.push(headers[i].as_bytes());
//         }
//     }

//     if cli.pretty {
//     } else {
//     }

//     String::new()
// }
