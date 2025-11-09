
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
fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn main() {
    println!("Bank in Rust");

    let bank = Bank::new();
    let other_bank = bank;

    println!("{:#?}", other_bank);

}
