type Bitcoin = i32;

struct Wallet {
    balance: Bitcoin,
}

impl Wallet {
    fn deposit(&mut self, amount: Bitcoin) {
        self.balance += amount
    }

    fn with_draw(&mut self, amount: Bitcoin) {
        if amount > self.balance {
            panic!("cannot withdraw, insufficient funds")
        }
        self.balance -= amount
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet::Wallet;

    #[test]
    fn test_wallet() {
        let mut wallet1 = Wallet { balance: 0 };
        wallet1.deposit(10);
        assert_eq!(wallet1.balance, 10);
    }

    #[test]
    #[should_panic(expected = "cannot withdraw, insufficient funds")]
    fn test_wallet_panic() {
        let mut wallet2 = Wallet { balance: 20 };
        wallet2.with_draw(10);
        assert_eq!(wallet2.balance, 10);
        wallet2.with_draw(100);
    }
}
