#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new () -> Bank {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_num_accounts(bank: &Bank){
    println!("Number of accounts: {}", bank.accounts.len());
}

fn change_account(account: &mut Account) {
    account.balance = 10;

    println!("{}", account.holder);
}

fn add_account (bank: &mut Bank, account: Account) {
    bank.accounts.push(account)
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("Leo"));

    add_account(&mut bank, account);
    

   println!("{:#?}, {:#?}", bank, bank);
}
