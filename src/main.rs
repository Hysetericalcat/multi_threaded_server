mod bank_account;

fn main() {
    let mut user1 = bank_account::Account {
        name:String::from("Akshat"),
        amount:1000
    };
    user1.deposit(1000);
    user1.withdraw(500);
    user1.display();
}
