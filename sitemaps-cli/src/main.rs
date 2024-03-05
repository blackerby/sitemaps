pub mod cli;
pub mod utils;

use crate::cli::Cli;
use crate::utils::build_output;
use clap::Parser;

use sitemaps::error::Error;
use sitemaps::reader::SitemapReader;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    if let Some(ref path) = cli.path {
        let sitemap = SitemapReader::read(&path)?;

        let output = build_output(sitemap, &cli).unwrap();

        println!("{}", output.trim_end());
    }

    Ok(())
}
