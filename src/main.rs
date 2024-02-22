use expense_tracker::banks;
use expense_tracker::banks::OperationsOnAccount;
use expense_tracker::cmd::Cmd;

fn main() {
    let mut args = Cmd::new();
    args.populate_prop_to_regex();

    let mut hdfc_account = banks::hdfc::AccountHDFC::new();
    hdfc_account.add_details_from_file(&args);
    println!("{:?}", hdfc_account);
}
