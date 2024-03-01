use clap::{Parser, Subcommand};
use clio::ClioPath;
use regex::Regex;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;
// use std::path::Path;

pub struct Cmd {
    args: Args,
    prop_to_regex: HashMap<String, (String, Regex)>,
}

/// Automated Expense Parsing
#[derive(Parser)]
struct Args {
    /// Path to bank statement file
    #[arg(short, long)]
    statement: ClioPath,

    /// Regex template path for parsing
    #[clap(short, long, value_parser = clap::value_parser!(ClioPath).exists().is_file())]
    parser: ClioPath,

    #[command(subcommand)]
    accounts_supported: Option<AccountSupported>,
}

/// Supported Accounts
#[derive(Subcommand)]
enum AccountSupported {
    /// Use Delimited Text File.
    /// Reg template must contain transaction property with following details:
    /// txndate : transaction date,
    /// info : description,
    /// debitamt: debit amount,
    /// creditamt: credit amount,
    /// balance: balance,
    HDFC,

    /// Use Text File.
    /// Reg template must contain transaction property with following details:
    /// txndate & valdate : transaction date,
    /// info : description,
    /// txnamt: transaction amount,
    /// finalbal: balance,
    SBI,
}

impl Cmd {
    pub fn new() -> Self {
        Self {
            args: Args::parse(),
            prop_to_regex: HashMap::new(),
        }
    }

    pub fn get_statement_path(&self) -> &Path {
        self.args.statement.path()
    }

    pub fn populate_prop_to_regex(&mut self) {
        if self.prop_to_regex.is_empty() {
            let template_file = OpenOptions::new()
                .read(true)
                .open(self.args.parser.path())
                .unwrap();
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

            self.prop_to_regex = map
        }
    }

    pub fn get_prop_to_regex(&self) -> &HashMap<String, (String, Regex)> {
        if self.prop_to_regex.is_empty() {
            eprintln!("Empty prop2regex fetched");
        }
        &self.prop_to_regex
    }
}
