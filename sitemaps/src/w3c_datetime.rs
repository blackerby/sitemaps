use chrono::{DateTime, FixedOffset, NaiveDate, ParseError};
use serde::Serialize;
use std::fmt;

// https://developers.google.com/search/blog/2006/04/using-lastmod-attribute
// https://www.w3.org/TR/NOTE-datetime

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub enum W3CDateTime {
    Date(NaiveDate),
    DateTime(DateTime<FixedOffset>),
}

impl W3CDateTime {
    pub fn new(string: &str) -> Result<W3CDateTime, ParseError> {
        Self::parse(string)
    }

    fn parse(string: &str) -> Result<W3CDateTime, ParseError> {
        if string.len() == 10 {
            Ok(W3CDateTime::Date(string.parse::<NaiveDate>()?))
        } else {
            Ok(W3CDateTime::DateTime(
                string.parse::<DateTime<FixedOffset>>()?,
            ))
        }
    }
}

impl fmt::Display for W3CDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            W3CDateTime::Date(date) => {
                let formatted = date.format("%Y-%m-%d").to_string();
                f.write_str(&formatted)
            }
            W3CDateTime::DateTime(datetime) => {
                let formatted = datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                f.write_str(&formatted)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_w3c_date_only() -> Result<(), ParseError> {
        let date_string = "2024-02-27";
        let result = W3CDateTime::parse(date_string)?;

        assert_eq!(date_string, result.to_string());

        Ok(())
    }

    #[test]
    fn test_w3c_midnight_utc() -> Result<(), ParseError> {
        let date_string = "2024-02-27T00:00:00Z";
        let result = W3CDateTime::parse(date_string)?;
        let expected = "2024-02-27T00:00:00Z";

        assert_eq!(expected, result.to_string());

        Ok(())
    }
}
