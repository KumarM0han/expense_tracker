pub mod hdfc;
pub mod sbi;
use super::cmd::Cmd;

#[derive(Debug)]
enum TransactionType {
    Credit,
    Debit,
}

#[derive(Debug)]
struct Transaction {
    on: String,
    description: String,
    amount: f64,
    tr_type: TransactionType,
}
pub trait OperationsOnAccount {
    fn add_details_from_file(&mut self, args: &Cmd);
}
