use super::OperationsOnAccount;
use super::Transaction;
use super::TransactionType;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct AccountHDFC {
    account_number: String,
    balance: f64,
    transactions: Vec<Transaction>,
}

impl OperationsOnAccount for AccountHDFC {
    fn add_details_from_file(&mut self, args: &crate::cmd::Cmd) {
        let file = OpenOptions::new()
            .read(true)
            .open(args.get_statement_path())
            .unwrap();
        let buffered_file = BufReader::new(file);
        let prop_to_regex = args.get_prop_to_regex();
        for line in buffered_file.lines() {
            let line = line.unwrap();

            for (prop, reg) in prop_to_regex.iter() {
                if let Some(caps) = reg.1.captures(&line) {
                    if prop == "transaction" {
                        let txndate = caps["txndate"].to_owned();
                        let info = caps["info"].to_owned();
                        let debitamt = caps["debitamt"].parse::<f64>().unwrap();
                        let creditamt = caps["creditamt"].parse::<f64>().unwrap();
                        let balance_after_transaction = caps["balance"].parse::<f64>().unwrap();

                        if creditamt == 0.00 {
                            self.transactions.push(Transaction {
                                amount: debitamt,
                                description: info,
                                on: txndate,
                                tr_type: TransactionType::Debit,
                            });
                        } else {
                            self.transactions.push(Transaction {
                                amount: debitamt,
                                description: info,
                                on: txndate,
                                tr_type: TransactionType::Credit,
                            });
                        }
                        self.balance = balance_after_transaction;
                    }
                }
            }
        }
    }
}

impl AccountHDFC {
    pub fn new() -> Self {
        Self {
            account_number: "".to_owned(),
            balance: 0.0,
            transactions: Vec::new(),
        }
    }
}

impl Default for AccountHDFC {
    fn default() -> Self {
        Self::new()
    }
}
