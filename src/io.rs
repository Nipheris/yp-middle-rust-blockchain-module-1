mod csv;
mod parsing;
mod text;

use std::{error::Error, io::Read};

pub enum Format {
    Csv,
    Text,
}

#[derive(Debug)]
pub enum TransactionReadError {
    Csv(csv::TransactionReadError),
    Text(text::TransactionReadError),
}

impl From<csv::TransactionReadError> for TransactionReadError {
    fn from(value: csv::TransactionReadError) -> Self {
        TransactionReadError::Csv(value)
    }
}

impl From<text::TransactionReadError> for TransactionReadError {
    fn from(value: text::TransactionReadError) -> Self {
        TransactionReadError::Text(value)
    }
}

impl std::fmt::Display for TransactionReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionReadError::Csv(csv_error) => csv_error.fmt(f),
            TransactionReadError::Text(text_error) => text_error.fmt(f),
        }
    }
}

impl Error for TransactionReadError {}

pub fn read_transactions<R: Read>(
    format: Format,
    reader: R,
) -> Result<Vec<crate::Transaction>, TransactionReadError> {
    match format {
        Format::Csv => Ok(csv::read_transactions(reader)?),
        Format::Text => Ok(text::read_transactions(reader)?),
    }
}
