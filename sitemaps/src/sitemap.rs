use core::fmt;
use serde::Serialize;

use crate::{error::SitemapError, w3c_datetime::W3CDateTime};

/// A Sitemap is an entity-escaped, UTF-8 encoded list of `<url>` elements contained in
/// in a `<urlset>` element.
#[derive(Debug, PartialEq, Serialize)]
pub struct Sitemap {
    /// The set of URLs in the sitemap.
    pub urlset: Urlset,
}

/// `<urlset>` is the XML root element. Here it is represented as a list of URLs.
#[derive(Debug, PartialEq, Serialize)]
pub struct Urlset(pub Vec<Url>);

/// The priority of this URL relative to other URLs on the site.
/// Valid values range from 0.0 to 1.0.
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Priority(pub f32);

/// A URL entry. It is a parent XML tag containing the required `<loc>` element
/// and the three optional `<lastmod>`, `<changrefreq>`, and `<priority>` elements.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Url {
    /// The URL of the described page. It is required.
    pub loc: String,
    /// The optional date of last modification of the page.
    pub last_mod: Option<W3CDateTime>,
    /// Optional. How frequently the page is likely to change.
    pub change_freq: Option<ChangeFreq>,
    /// Optional. The priority of this URL relative to other URLs on the site.
    pub priority: Option<Priority>,
}

impl Url {
    /// Create a new, empty Url.
    pub fn new() -> Self {
        Self {
            loc: String::new(),
            last_mod: None,
            change_freq: None,
            priority: None,
        }
    }
}

impl Priority {
    /// Create a new, valid Priority.
    pub fn new(priority: f32) -> Result<Self, SitemapError> {
        if priority < 0.0 {
            return Err(SitemapError::PriorityTooLow);
        }

        if priority > 1.0 {
            return Err(SitemapError::PriorityTooHigh);
        }

        Ok(Self(priority))
    }
}

/// ChangeFreq represents how frequently the page is likely to change.
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub enum ChangeFreq {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

impl From<String> for ChangeFreq {
    fn from(value: String) -> Self {
        let normalized_value = value.to_lowercase();

        match normalized_value.as_ref() {
            "always" => ChangeFreq::Always,
            "hourly" => ChangeFreq::Hourly,
            "daily" => ChangeFreq::Daily,
            "weekly" => ChangeFreq::Weekly,
            "monthly" => ChangeFreq::Monthly,
            "yearly" => ChangeFreq::Yearly,
            "never" => ChangeFreq::Never,
            _ => panic!("Unrecognized change frequency"),
        }
    }
}

impl fmt::Display for ChangeFreq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match *self {
            ChangeFreq::Always => "always",
            ChangeFreq::Hourly => "hourly",
            ChangeFreq::Daily => "daily",
            ChangeFreq::Weekly => "weekly",
            ChangeFreq::Monthly => "monthly",
            ChangeFreq::Yearly => "yearly",
            ChangeFreq::Never => "never",
        };

        f.write_str(data)
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:.1}", &self.0))
    }
}
