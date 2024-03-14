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

        match Sitemaps::read(reader) {
            Ok(sitemap) => match build_output(sitemap, &cli) {
                Ok(output) => println!("{}", output.trim_end()),
                Err(err) => println!("{}", err.to_string()),
            },
            Err(err) => println!("{}", err.to_string()),
        }
    }

    Ok(())
}
