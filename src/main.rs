use clap::builder::TypedValueParser;
use clap::Parser;
use clio::ClioPath;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::{fs::File, fs::OpenOptions, io::BufRead, io::BufReader};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    statement: ClioPath,

    #[clap(short, long, value_parser = clap::value_parser!(ClioPath).exists().is_file())]
    parser: ClioPath,
}

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

#[derive(Debug)]
struct AccountSBI {
    account_number: String,
    balance: f64,
    transactions: Vec<Transaction>,
}

fn main() {
    let args = Args::parse();
    let raw_file = OpenOptions::new()
        .read(true)
        .open(args.statement.path())
        .unwrap();
    let buff_file = BufReader::new(raw_file);

    let prop_to_regex = reg_template_config_parser(args.parser.path());
    let mut sbi_account = AccountSBI::new();
    sbi_account.add_deatils_from_file(buff_file, &prop_to_regex);
    println!("{:?}", sbi_account);
}

impl AccountSBI {
    fn new() -> Self {
        Self {
            account_number: "".to_owned(),
            balance: 0.0,
            transactions: Vec::new(),
        }
    }

    fn add_deatils_from_file(
        &mut self,
        buffered_file: BufReader<File>,
        prop_to_regex: &HashMap<String, (String, Regex)>,
    ) {
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

fn reg_template_config_parser<P: AsRef<Path>>(
    template_file: P,
) -> HashMap<String, (String, Regex)> {
    let template_file = OpenOptions::new().read(true).open(template_file).unwrap();
    let template_file_buf = BufReader::new(template_file);

    let re = Regex::new(r"^(?<prop>\w+)\s+:\s+(?<regex>.*)").unwrap();
    let mut map = HashMap::new();

    for line in template_file_buf.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("//") {
            continue;
        }

        if let Some(caps) = re.captures(&line) {
            let prop_re = Regex::new(&caps["regex"]).unwrap();
            map.insert(
                caps["prop"].to_string(),
                (caps["regex"].to_string(), prop_re),
            );
        }
    }

    map
}
