use crate::{
    Transaction,
    io::parsing::{
        TransactionFieldValueParseError, parse_transaction_amount, parse_transaction_category,
        parse_transaction_date, parse_transaction_type,
    },
};
use std::io::{BufRead, BufReader, Read};

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
    use super::{TransactionParseError, read_transactions};
    use crate::{Transaction, TransactionType};

    #[test]
    fn read_transactions_read_all() -> Result<(), TransactionParseError> {
        let r = std::io::Cursor::new(String::from(
            "\
            2026-04-01;salary;income;120000\n\
            2026-05-16;travel;expense;6000\n",
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
