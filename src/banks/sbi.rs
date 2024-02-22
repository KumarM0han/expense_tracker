use super::OperationsOnAccount;
use super::Transaction;
use super::TransactionType;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct AccountSBI {
    account_number: String,
    balance: f64,
    transactions: Vec<Transaction>,
}

impl OperationsOnAccount for AccountSBI {
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
                    println!("{:?}", caps.name("accnumber"));
                    if prop == "account_number" {
                        self.account_number = caps["accnumber"].to_string();
                    } else if prop == "starting_balance" {
                        self.balance = caps["initbalance"].replace(',', "").parse().unwrap();
                    } else if prop == "transaction" {
                        let final_bal_from_transaction =
                            caps["finalbal"].replace(',', "").parse::<f64>().unwrap();
                        let txnamt = caps["txnamt"].replace(',', "").parse::<f64>().unwrap();
                        let valdate = caps["valdate"].to_owned();
                        let descript = caps["info"].to_owned();
                        if final_bal_from_transaction < self.balance {
                            self.transactions.push(Transaction {
                                amount: txnamt,
                                description: descript,
                                on: valdate,
                                tr_type: TransactionType::Debit,
                            });
                        } else {
                            self.transactions.push(Transaction {
                                amount: txnamt,
                                description: descript,
                                on: valdate,
                                tr_type: TransactionType::Credit,
                            });
                        }
                        self.balance = final_bal_from_transaction;
                    }
                }
            }
        }
    }
}

impl AccountSBI {
    pub fn new() -> Self {
        Self {
            account_number: "".to_owned(),
            balance: 0.0,
            transactions: Vec::new(),
        }
    }
}

impl Default for AccountSBI {
    fn default() -> Self {
        Self::new()
    }
}
