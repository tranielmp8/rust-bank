
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

fn main() {
    println!("Bank in Rust -- Ownership, Borrowing");

    let account: Account = Account::new(1, String::from("Me"));
    let account_ref1 = &account; // Create a reference to THIS value
    let account_ref2 = &account; // Create a reference to THIS value

    // read only reference means immutable, while writable reference means mutable

    print_account(account_ref1);
    print_account(account_ref2);
    print_account(account_ref2);

    println!("{:#?}", account);

}
