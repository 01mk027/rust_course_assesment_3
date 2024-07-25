trait Account{
    fn deposit(&mut self, amount: i64);
    fn withdraw(&mut self, amount: i64);
    fn balance(&mut self) -> i64;
}

struct BankAccount{
    account_number: u32,
    holder_name: String,
    balance: i64
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: i64) 
    {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i64)
    {
        self.balance -= amount;
    }

    fn balance(&mut self) -> i64{
        return self.balance;
    }
}

fn main() {
    let mut first_bank_account = BankAccount{
        account_number: 312423,
        holder_name: "Mustafa KOŞMAZ".to_string(),
        balance: 5e+12 as i64
    };

    let mut second_bank_account = BankAccount{
        account_number: 318426,
        holder_name: "Test User".to_string(),
        balance: 4e+8 as i64
    };

    first_bank_account.deposit(3e+6 as i64);
    second_bank_account.withdraw(2e+8 as i64);
    println!("Balance of first account is {}, and balance of second account is  {}", first_bank_account.balance(), second_bank_account.balance())
}
