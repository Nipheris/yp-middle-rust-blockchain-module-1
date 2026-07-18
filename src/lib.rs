use chrono::NaiveDate;

pub mod io;

#[derive(Debug, PartialEq)]
pub enum TransactionType {
    Income,
    Expense,
}

// TODO: impl Display for Transaction
#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub type_: TransactionType,
    pub date: NaiveDate,
    pub category: String,
    pub amount: u64,
}
