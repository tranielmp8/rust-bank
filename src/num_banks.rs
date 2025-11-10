
// Account
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
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

// Bank
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

// think of impl or implementation like receivers in go which creates methods for func, structs etc..
impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

// Helper function to do println!("{:#?}", variable);
// anytime you see an & operator next to a type it means this argument needs to be a reference to A value
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_num_accounts(bank: &Bank) {
    println!("Number of Bank accounts: {}", bank.accounts.len());
}

fn main() {
    println!("Bank in Rust -- Ownership, Borrowing");

    // bank
    let mut bank = Bank::new();

    let account: Account = Account::new(1, String::from("Me"));
    let account2 = Account::new(2, String::from("You")); // Create a reference to THIS value
    let account3 = Account::new(3, String::from("We")); // Create a reference to THIS value

    bank.accounts.push(account);
    bank.accounts.push(account2);
    bank.accounts.push(account3);

    print_num_accounts(&bank);

    // read only reference means immutable, while writable reference means mutable


    println!("{:#?}", bank);

}
