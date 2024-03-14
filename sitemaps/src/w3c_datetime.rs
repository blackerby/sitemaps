use chrono::{DateTime, FixedOffset, NaiveDate, ParseError};
use serde::Serialize;
use std::fmt;

/// A W3CDateTime is an ISO-8601 date or an RFC-3339 datetime.
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub enum W3CDateTime {
    DateTime(DateTime<FixedOffset>, bool, bool),
    Date(NaiveDate),
}

impl W3CDateTime {
    /// Create a new W3CDateTime by parsing a string.
    /// "YYYY-MM-DD" formatted strings will return a
    /// `W3CDateTime::Date`. "YYYY-MM-DDThh:mm:ssTZD"
    /// and "YYYY-MM-DDThh:mm:ss.sTZD" formatted strings
    /// will return a `W3CDateTime::DateTime`.
    pub fn new(string: &str) -> Result<W3CDateTime, ParseError> {
        Self::parse(string)
    }

    fn parse(string: &str) -> Result<W3CDateTime, ParseError> {
        if string.len() == 10 {
            Ok(W3CDateTime::Date(string.parse::<NaiveDate>()?))
        } else {
            Ok(W3CDateTime::DateTime(
                DateTime::parse_from_rfc3339(string)?,
                string.contains('.'),
                string.to_uppercase().ends_with('Z'),
            ))
        }
    }
}

impl fmt::Display for W3CDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Date(date) => f.write_str(&date.format("%Y-%m-%d").to_string()),
            Self::DateTime(datetime, fractional, use_z) => {
                let formatted = datetime.to_rfc3339_opts(
                    if fractional {
                        chrono::SecondsFormat::Millis
                    } else {
                        chrono::SecondsFormat::Secs
                    },
                    use_z,
                );
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
