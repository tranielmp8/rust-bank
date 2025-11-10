
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

fn change_account(account: &mut Account) {
    account.balance = 10;

    println!("{}", account.holder);
}

fn main() {
    println!("Bank in Rust -- Ownership, Borrowing");

    let mut account: Account = Account::new(1, String::from("Me"));

    // read only reference means immutable, while writable reference means mutable
    change_account(&mut account);

    println!("{:#?}", account);

}
