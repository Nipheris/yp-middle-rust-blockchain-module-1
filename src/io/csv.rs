use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    todo,
};

use crate::{
    Transaction,
    io::{csv::TransactionReadError::BadHeaderLine, parsing::TransactionFieldValueParseError},
};

#[derive(Debug)]
pub enum TransactionReadError {
    BadHeaderLine(String),
    BadDataLine(String),
    FieldValue(TransactionFieldValueParseError),
}
impl std::fmt::Display for TransactionReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadHeaderLine(line) => {
                write!(
                    f,
                    "incorrectly formatted header line in input stream: {line}"
                )
            }
            Self::BadDataLine(line) => {
                write!(f, "incorrectly formatted data line in input stream: {line}")
            }
            Self::FieldValue(err) => write!(f, "non-parseable field value: {err}"),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
enum TransactionField {
    Type,
    Date,
    Category,
    Amount,
}
type TransactionFieldIndex = u8;
type TransactionFieldIndexMap = HashMap<TransactionField, TransactionFieldIndex>;
fn parse_header(header_line: &str) -> Result<TransactionFieldIndexMap, TransactionReadError> {
    let mut result = TransactionFieldIndexMap::new();

    let field_names: [&str; 4] = header_line
        .trim()
        .split(',') // comma-separated, isn't it? :)
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|components: Vec<&str>| {
            TransactionReadError::BadHeaderLine(components.join(","))
        })?;

    for _field_name in field_names {
        result.insert(TransactionField::Type, 0);
        result.insert(TransactionField::Date, 1);
        result.insert(TransactionField::Category, 2);
        result.insert(TransactionField::Amount, 3);
        todo!();
    }

    Ok(result)
}

fn parse_transaction(
    _tx_line: &str,
    _field_index_map: &TransactionFieldIndexMap,
) -> Result<Transaction, TransactionReadError> {
    todo!();
}

pub fn read_transactions<R: Read>(r: R) -> Result<Vec<crate::Transaction>, TransactionReadError> {
    let mut reader = BufReader::new(r);
    let mut result: Vec<Transaction> = vec![];

    let mut header_line = String::new();
    reader
        .read_line(&mut header_line)
        .map_err(|_e| BadHeaderLine("".to_string()))?;
    let field_index_map = parse_header(&header_line)?;

    for line in reader.lines().map_while(Result::ok) {
        result.push(parse_transaction(&line, &field_index_map)?);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{TransactionReadError, read_transactions};
    use crate::{Transaction, TransactionType};

    #[test]
    fn read_transactions_read_all() -> Result<(), TransactionReadError> {
        let r = std::io::Cursor::new(String::from(
            "\
            date,category,kind,amount\n\
            2026-04-01,salary,income,120000\n\
            2026-04-02,rent,expense,40000\n",
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
    fn read_transactions_malformed_header() {
        let r = std::io::Cursor::new(String::from(
            "\
            date,category,unknown,amount\n\
            2026-04-01,salary,income,120000\n\
            2026-04-02,rent,expense,40000\n",
        ));

        assert!(read_transactions(r).is_err());
    }
}
