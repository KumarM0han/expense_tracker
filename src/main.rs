use regex::Regex;
use std::{fs::OpenOptions, io::BufRead, io::BufReader};

use std::collections::HashMap;

fn reg_template_config_parser() -> HashMap<String, (String, Regex)> {
    let template_file = OpenOptions::new()
        .read(true)
        .open("src/templates/sbi.reg")
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

    map
}

fn main() {
    let raw_file = OpenOptions::new().read(true).open("sbi.txt").unwrap();
    let buff_file = BufReader::new(raw_file);

    let prop_to_regex = reg_template_config_parser();

    for line in buff_file.lines() {
        let line = line.unwrap();

        // Parsers

        for (prop, reg) in prop_to_regex.iter() {
            if let Some(caps) = reg.1.captures(&line) {
                for c in caps.iter() {
                    println!("{:?}", c);
                }
            }
        }

        // if let Some(acc_number) = get_account_number(&line) {
        //     println!("AccNum: {}", acc_number);
        // }

        // if let Some(start_date) = get_start_date(&line) {
        //     println!("Start Date: {}", start_date);
        // }

        // if let Some(end_date) = get_end_date(&line) {
        //     println!("Start Date: {}", end_date);
        // }

        extract_transaction_entry(&line);
    }
}

fn extract_transaction_entry(line: &str) {
    let transaction_search_pattern = r"^(?<txndate>\d{1,2}\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+\d{4})\s+(?<valdate>\d{1,2}\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+\d{4})\s+(?<info>.*)\s+(?<txnamt>\d{0,2},?\d{0,2},?\d{0,3}\.\d{2})\s+(?<finalbal>\d{0,2},?\d{0,2},?\d{0,3}\.\d{2})";
    let re = Regex::new(transaction_search_pattern).unwrap();
    if let Some(caps) = re.captures(line) {
        println!("txndate: {}", &caps["txndate"]);
        println!("valdate: {}", &caps["valdate"]);
        println!("info: {}", &caps["info"]);
        println!("txnamt: {}", &caps["txnamt"]);
        println!("finalbal: {}\n", &caps["finalbal"]);
    }
}

fn get_end_date(line: &str) -> Option<&str> {
    if !line.starts_with("End Date") {
        return None;
    }

    let end_date = line
        .split(':')
        .map(|s| s.trim())
        .last()
        .expect("Failed to extract end date from iterator");

    Some(end_date)
}

fn get_start_date(line: &str) -> Option<&str> {
    if !line.starts_with("Start Date") {
        return None;
    }

    let start_date = line
        .split(':')
        .map(|s| s.trim())
        .last()
        .expect("Failed to extract start date from iterator");

    Some(start_date)
}

fn get_account_number(line: &str) -> Option<&str> {
    if !line.starts_with("Account Number") {
        return None;
    }

    let acc_number = line
        .split(':')
        .map(|s| s.trim())
        .last()
        .expect("Failed to extract account number from iterator");

    Some(acc_number)
}
