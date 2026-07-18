use altair::Transaction;

fn print_transactions(transactions: &Vec<Transaction>) {
    for tx in transactions {
        println!("{:?}", tx);
    }
}

fn main() {
    let reader = std::io::Cursor::new(String::from("2026-04-30;rent;income;2000"));
    let transactions_or_error = altair::io::read_transactions(reader);
    match transactions_or_error {
        Ok(transactions) => print_transactions(&transactions),
        Err(error) => println!("Error: {error}"),
    }
}
