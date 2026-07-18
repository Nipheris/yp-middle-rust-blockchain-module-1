use crate::TransactionType;
use chrono::{NaiveDate, ParseError as ChronoParseError, ParseResult as ChronoParseResult};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum TransactionFieldValueParseError {
    Type(String),
    Date(ChronoParseError),
    Category(String),
    Amount(ParseIntError),
}
impl std::fmt::Display for TransactionFieldValueParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type(s) => write!(f, "invalid type string: \"{s}\""),
            Self::Date(err) => write!(f, "cannot parse date: {err}"),
            Self::Category(s) => write!(f, "invalid category string: \"{s}\""), // TODO s vs err
            Self::Amount(err) => write!(f, "cannot parse amount: {err}"),
        }
    }
}

pub fn parse_transaction_type(type_str: &str) -> Result<TransactionType, String> {
    match type_str {
        "income" => Ok(TransactionType::Income),
        "expense" => Ok(TransactionType::Expense),
        _ => Err(String::from(type_str)),
    }
}

pub fn parse_transaction_date(date_str: &str) -> ChronoParseResult<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

pub fn parse_transaction_category(category_str: &str) -> Result<String, String> {
    match category_str {
        "" => Err(String::from(category_str)),
        _ => Ok(String::from(category_str)),
    }
}

pub fn parse_transaction_amount(amount_str: &str) -> Result<u64, ParseIntError> {
    amount_str.parse::<u64>()
}

#[cfg(test)]
mod tests {
    use super::parse_transaction_date;

    #[test]
    fn parse_transaction_date_works() {
        assert_eq!(
            parse_transaction_date("2020-12-31"),
            Ok(chrono::NaiveDate::from_ymd_opt(2020, 12, 31).unwrap())
        );
        assert_eq!(
            parse_transaction_date("1900-01-01"),
            Ok(chrono::NaiveDate::from_ymd_opt(1900, 01, 01).unwrap())
        );
        assert!(parse_transaction_date("ABCD-AA-BB").is_err());
        assert!(parse_transaction_date("").is_err());
    }
}
