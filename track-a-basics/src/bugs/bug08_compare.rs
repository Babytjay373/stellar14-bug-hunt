//! Bug 08: can this wallet afford a payment?
//!
//! Run it:  cargo test -p wallet-basics bug08

use crate::types::{Wallet, BASE_FEE_STROOPS};

/// True when the wallet can pay `amount` plus the network fee.
/// Spending the exact balance, down to zero, is allowed.
pub fn can_pay(wallet: &Wallet, amount: u64) -> bool {
    wallet.balance >= amount + BASE_FEE_STROOPS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug08_can_pay_with_room_to_spare() {
        let wallet = Wallet::new("ama", "GAMA", 1_000);
        assert!(can_pay(&wallet, 500));
        assert!(!can_pay(&wallet, 950));
    }

    #[test]
    fn bug08_can_pay_the_exact_balance() {
        let wallet = Wallet::new("ama", "GAMA", 1_000);
        assert!(can_pay(&wallet, 900));
    }
}
