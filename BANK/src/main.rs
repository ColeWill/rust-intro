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

    fn summary(&self) -> String {
        format!("Account Summary: Holder: {} has a balance of {}", self.holder, self.balance)
    }

    fn deposit (&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw (&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
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

    fn add_account (&mut self, account: Account) {
        self.accounts.push(account)
    }

    fn total_balance (&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }

}



fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));
    let mut account2 = Account::new(2, String::from("steve"));
    
    account.deposit(10500);
    account.withdraw(250);
    account2.deposit(123303);
    account2.withdraw(250);
  
    bank.add_account(account);
    bank.add_account(account2);
    println!("{:#?}, {}", bank.summary(), bank.total_balance());
}
