use clap::Parser;

#[derive(Parser)]
#[command(name = "Sitemaps")]
#[command(version = "0.1.0")]
#[command(about = "Read data from sitemap.xml files", long_about = None)]
pub(crate) struct Cli {
    /// Path to sitemap.xml file
    #[arg(default_value = "-")]
    pub path: Option<String>,
    /// Include value of <loc> in output
    #[arg(short, long, default_value_t = true)]
    pub loc: bool,
    /// Include value of <lastmod> in output
    #[arg(short = 'L', long, default_value_t = true)]
    pub lastmod: bool,
    /// Include value of <changefreq> in output. Sitemaps only.
    #[arg(short, long)]
    pub changefreq: bool,
    /// Include value of <priority> in output. Sitemaps only.
    #[arg(short, long)]
    pub priority: bool,
    /// Print output table with cell borders.
    #[arg(short = 'P', long)]
    pub pretty: bool,
    /// Print output table with column headers.
    #[arg(short = 'H', long, default_value_t = true)]
    pub header: bool,
    /// Print output as JSON.
    #[arg(short, long)]
    pub json: bool,
    /// Print output as CSV.
    #[arg(short = 'C', long)]
    pub csv: bool,
    /// Print output as Markdown.
    #[arg(short = 'm', long)]
    pub markdown: bool,
}
