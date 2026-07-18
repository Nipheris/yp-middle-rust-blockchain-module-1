use chrono::{NaiveDate, ParseError as ChronoParseError, ParseResult as ChronoParseResult};
use std::{
    io::{BufRead, BufReader, Read},
    num::ParseIntError,
};

use crate::{Transaction, TransactionType};

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

#[derive(Debug)]
pub enum TransactionParseError {
    BadInputLine(String),
    FieldValue(TransactionFieldValueParseError),
}
impl std::fmt::Display for TransactionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadInputLine(line) => {
                write!(f, "incorrectly formatted line in input data: {line}")
            }
            Self::FieldValue(err) => write!(f, "non-parseable field value: {err}"),
        }
    }
}

fn parse_transaction_type(type_str: &str) -> Result<TransactionType, String> {
    match type_str {
        "income" => Ok(TransactionType::Income),
        "expense" => Ok(TransactionType::Expense),
        _ => Err(String::from(type_str)),
    }
}

fn parse_transaction_date(date_str: &str) -> ChronoParseResult<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

fn parse_transaction_category(category_str: &str) -> Result<String, String> {
    match category_str {
        "" => Err(String::from(category_str)),
        _ => Ok(String::from(category_str)),
    }
}

fn parse_transaction_amount(amount_str: &str) -> Result<u64, ParseIntError> {
    amount_str.parse::<u64>()
}

fn parse_transaction(tx_line: &str) -> Result<Transaction, TransactionParseError> {
    let field_value_strs = tx_line
        .trim()
        .split(';')
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|components: Vec<&str>| {
            TransactionParseError::BadInputLine(components.join(";"))
        })?;

    let [date_str, category_str, type_str, amount_str] = field_value_strs;

    use TransactionFieldValueParseError as FVPE;
    let date = parse_transaction_date(date_str)
        .map_err(|e| TransactionParseError::FieldValue(FVPE::Date(e)))?;
    let category = parse_transaction_category(category_str)
        .map_err(|e| TransactionParseError::FieldValue(FVPE::Category(e)))?;
    let type_ = parse_transaction_type(type_str)
        .map_err(|e| TransactionParseError::FieldValue(FVPE::Type(e)))?;
    let amount = parse_transaction_amount(amount_str)
        .map_err(|e| TransactionParseError::FieldValue(FVPE::Amount(e)))?;

    Ok(Transaction {
        type_,
        date,
        category,
        amount,
    })
}

pub fn read_transactions<R: Read>(r: R) -> Result<Vec<crate::Transaction>, TransactionParseError> {
    let reader = BufReader::new(r);
    let mut result: Vec<Transaction> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        result.push(parse_transaction(&line)?);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        Transaction, TransactionType,
        io::text::{TransactionParseError, parse_transaction_date, read_transactions},
    };

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

    #[test]
    fn read_transactions_read_all() -> Result<(), TransactionParseError> {
        let r = std::io::Cursor::new(String::from(
            "2026-04-01;salary;income;120000\n2026-05-16;travel;expense;6000\n",
        ));

        let transactions = read_transactions(r)?;
        assert_eq!(
            transactions,
            [
                Transaction {
                    type_: TransactionType::Income,
                    date: chrono::NaiveDate::from_ymd_opt(2026, 04, 01).unwrap(),
                    category: String::from("salary"),
                    amount: 120000
                },
                Transaction {
                    type_: TransactionType::Expense,
                    date: chrono::NaiveDate::from_ymd_opt(2026, 5, 16).unwrap(),
                    category: String::from("travel"),
                    amount: 6000
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn read_transactions_read_fail() {
        let r = std::io::Cursor::new(String::from(
            "2026-04-01;salary;income;120000\n2026-04-07;travel;WTF;6000\n\n\n",
        ));

        assert!(read_transactions(r).is_err());
    }
}
