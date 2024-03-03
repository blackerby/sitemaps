use clap::Parser;

#[derive(Parser)]
#[command(name = "Simple Sitemap")]
#[command(version = "0.1.0")]
#[command(about = "Read data from sitemap.xml files", long_about = None)]
pub(crate) struct Cli {
    #[arg(default_value = "-")]
    pub path: Option<String>,
    #[arg(short, long)]
    pub loc: bool,
    #[arg(short = 'L', long)]
    pub lastmod: bool,
    #[arg(short, long)]
    pub changefreq: bool,
    #[arg(short, long)]
    pub priority: bool,
    #[arg(short = 'P', long)]
    pub pretty: bool,
    #[arg(short = 'H', long)]
    pub header: bool,
}
