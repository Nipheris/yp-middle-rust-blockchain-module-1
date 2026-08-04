use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use crate::{
    Transaction,
    io::{
        csv::TransactionReadError::BadHeaderLine,
        parsing::{
            TransactionFieldValueParseError, parse_transaction_amount, parse_transaction_category,
            parse_transaction_date, parse_transaction_type,
        },
    },
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
type TransactionFieldIndex = usize;
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

    for (field_index, field_name) in field_names.iter().enumerate() {
        match *field_name {
            "kind" => result.insert(TransactionField::Type, field_index),
            "date" => result.insert(TransactionField::Date, field_index),
            "category" => result.insert(TransactionField::Category, field_index),
            "amount" => result.insert(TransactionField::Amount, field_index),
            _ => {
                return Err(TransactionReadError::BadHeaderLine(
                    "unknown field".to_string(),
                ));
            }
        };
    }

    Ok(result)
}

fn parse_transaction(
    tx_line: &str,
    field_index_map: &TransactionFieldIndexMap,
) -> Result<Transaction, TransactionReadError> {
    let field_value_strs: [&str; 4] = tx_line
        .trim()
        .split(',') // comma-separated, isn't it? :)
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|components: Vec<&str>| {
            TransactionReadError::BadHeaderLine(components.join(","))
        })?;

    let date_str = field_value_strs[field_index_map[&TransactionField::Date]];
    let category_str = field_value_strs[field_index_map[&TransactionField::Category]];
    let type_str = field_value_strs[field_index_map[&TransactionField::Type]];
    let amount_str = field_value_strs[field_index_map[&TransactionField::Amount]];

    use TransactionFieldValueParseError as FVPE;
    let date = parse_transaction_date(date_str)
        .map_err(|e| TransactionReadError::FieldValue(FVPE::Date(e)))?;
    let category = parse_transaction_category(category_str)
        .map_err(|e| TransactionReadError::FieldValue(FVPE::Category(e)))?;
    let type_ = parse_transaction_type(type_str)
        .map_err(|e| TransactionReadError::FieldValue(FVPE::Type(e)))?;
    let amount = parse_transaction_amount(amount_str)
        .map_err(|e| TransactionReadError::FieldValue(FVPE::Amount(e)))?;

    Ok(Transaction {
        type_,
        date,
        category,
        amount,
    })
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
        let expected = [
            Transaction {
                type_: TransactionType::Income,
                date: chrono::NaiveDate::from_ymd_opt(2026, 04, 01).unwrap(),
                category: String::from("salary"),
                amount: 120000,
            },
            Transaction {
                type_: TransactionType::Expense,
                date: chrono::NaiveDate::from_ymd_opt(2026, 04, 02).unwrap(),
                category: String::from("rent"),
                amount: 40000,
            },
        ];

        let r1 = std::io::Cursor::new(String::from(
            "\
            date,category,kind,amount\n\
            2026-04-01,salary,income,120000\n\
            2026-04-02,rent,expense,40000\n",
        ));
        assert_eq!(read_transactions(r1)?, expected);

        let r2 = std::io::Cursor::new(String::from(
            "\
            category,date,amount,kind\n\
            salary,2026-04-01,120000,income\n\
            rent,2026-04-02,40000,expense  \n",
        ));
        assert_eq!(read_transactions(r2)?, expected);

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
