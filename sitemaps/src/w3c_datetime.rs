use std::fmt;
use time::format_description::well_known::Iso8601;
use time::{error, Date, OffsetDateTime};

// https://developers.google.com/search/blog/2006/04/using-lastmod-attribute
// https://www.w3.org/TR/NOTE-datetime

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum W3CDateTime {
    Date(Date),
    OffsetDateTime(OffsetDateTime),
}

impl W3CDateTime {
    pub fn parse(string: &str) -> Result<W3CDateTime, error::Parse> {
        if string.len() == 10 {
            Ok(W3CDateTime::Date(Date::parse(string, &Iso8601::DATE)?))
        } else {
            Ok(W3CDateTime::OffsetDateTime(OffsetDateTime::parse(
                string,
                &Iso8601::DATE_TIME_OFFSET,
            )?))
        }
    }
}

impl fmt::Display for W3CDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            W3CDateTime::Date(date) => f.write_str(&date.to_string()),
            W3CDateTime::OffsetDateTime(datetime) => f.write_str(&datetime.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_w3c_date_only() -> Result<(), error::Parse> {
        let date_string = "2024-02-27";
        let result = W3CDateTime::parse(date_string)?;

        assert_eq!(date_string, result.to_string());

        Ok(())
    }

    #[test]
    fn test_w3c_midnight_utc() -> Result<(), error::Parse> {
        let date_string = "2024-02-27T00:00:00Z";
        let result = W3CDateTime::parse(date_string)?;
        let expected = "2024-02-27 0:00:00.0 +00:00:00";

        assert_eq!(expected, result.to_string());

        Ok(())
    }
}
