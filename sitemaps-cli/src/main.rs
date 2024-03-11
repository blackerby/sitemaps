pub mod cli;
pub mod utils;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::cli::Cli;
use crate::utils::build_output;
use clap::Parser;

use sitemaps::error::Error;
use sitemaps::Sitemaps;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    if let Some(ref path) = cli.path {
        let reader: Box<dyn BufRead> = match path.as_str() {
            "-" => Box::new(BufReader::new(io::stdin())),
            _ => Box::new(BufReader::new(File::open(path)?)),
        };

        let sitemap = Sitemaps::read(reader)?;

        let output = build_output(sitemap, &cli).unwrap();

        println!("{}", output.trim_end());
    }

    Ok(())
}
