// Demonstrates `self` parameter as a mutable struct instance.
//
// Methods can take `self` in different ways:
// - `&self`     — immutable borrow (read-only)
// - `&mut self` — mutable borrow (can modify fields)
// - `self`      — takes ownership (consumes the struct)
// - `mut self`  — takes ownership AND can modify before consuming
//
// Run: cargo run --example s09_self_mut

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    /// Creates a new account with zero balance.
    fn new(owner: &str) -> Self {
        BankAccount {
            owner: owner.to_string(),
            balance: 0.0,
        }
    }

    /// Deposits money (modifies self via mutable borrow).
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to deposit (must be positive)
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    /// Withdraws money if sufficient funds exist.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to withdraw
    ///
    /// # Returns
    ///
    /// true if withdrawal succeeded
    fn withdraw(&mut self, amount: f64) -> bool {
        if amount > 0.0 && self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    /// Closes the account and returns the remaining balance.
    ///
    /// Takes ownership of `self` — the account is consumed.
    /// After calling this, the account variable is no longer usable.
    ///
    /// # Returns
    ///
    /// The final balance
    fn close(self) -> f64 {
        println!("Closing account for {}", self.owner);
        self.balance
        // self is dropped after this function returns
    }
}

fn main() {
    // --- 1. Create account ---
    let mut acct = BankAccount::new("Alice");
    println!("{acct:?}");

    // --- 2. Deposit (uses &mut self) ---
    acct.deposit(100.0);
    acct.deposit(50.0);
    println!("After deposits: {acct:?}");

    // --- 3. Withdraw (uses &mut self) ---
    let ok = acct.withdraw(30.0);
    println!("Withdraw 30: {ok}, balance: {}", acct.balance);

    let ok = acct.withdraw(200.0); // insufficient funds
    println!("Withdraw 200: {ok}, balance: {}", acct.balance);

    // --- 4. Close account (takes ownership via self) ---
    let final_balance = acct.close();
    println!("Final balance: {final_balance}");
    // println!("{acct:?}"); // error: value used after move
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let acct = BankAccount::new("Test");
        assert_eq!(acct.owner, "Test");
        assert!((acct.balance - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_deposit() {
        let mut acct = BankAccount::new("Test");
        acct.deposit(100.0);
        assert!((acct.balance - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_deposit_negative_ignored() {
        let mut acct = BankAccount::new("Test");
        acct.deposit(-50.0);
        assert!((acct.balance - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_withdraw_success() {
        let mut acct = BankAccount::new("Test");
        acct.deposit(100.0);
        assert!(acct.withdraw(30.0));
        assert!((acct.balance - 70.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_withdraw_insufficient() {
        let mut acct = BankAccount::new("Test");
        acct.deposit(10.0);
        assert!(!acct.withdraw(20.0));
        assert!((acct.balance - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_close() {
        let mut acct = BankAccount::new("Test");
        acct.deposit(50.0);
        let b = acct.close();
        assert!((b - 50.0).abs() < f64::EPSILON);
    }
}
