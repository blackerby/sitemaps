use core::fmt;
use serde::Serialize;

use crate::{error::SitemapError, w3c_datetime::W3CDateTime};

#[derive(Debug, PartialEq, Serialize)]
pub struct Sitemap {
    pub urlset: Urlset,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Urlset(pub Vec<Url>);

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Priority(pub f32);

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Url {
    pub loc: String,
    pub last_mod: Option<W3CDateTime>,
    pub change_freq: Option<ChangeFreq>,
    pub priority: Option<Priority>,
}

impl Url {
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
