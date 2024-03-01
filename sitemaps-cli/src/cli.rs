use clap::Parser;

#[derive(Parser)]
#[command(name = "Simple Sitemap")]
#[command(version = "0.1.0")]
#[command(about = "Read data from sitemap.xml files", long_about = None)]
pub struct Cli {
    pub path: Option<String>,
    #[arg(short, long)]
    pub loc: bool,
    #[arg(short = 'm', long)]
    pub lastmod: bool,
    #[arg(short, long)]
    pub changefreq: bool,
    #[arg(short, long)]
    pub priority: bool,
    #[arg(short = 'r', long)]
    pub pretty: bool,
}
