//! Shared types for the wallet library.
//!
//! Everything in this file is correct. You should not need to change it to fix any bug.

/// One XLM is ten million stroops. Stellar stores every amount in stroops.
pub const STROOPS_PER_XLM: i64 = 10_000_000;

/// The base network fee for one operation, in stroops.
pub const BASE_FEE_STROOPS: u64 = 100;

/// An asset on Stellar: either native XLM, or a credit asset issued by an account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asset {
    Native,
    Credit { code: String, issuer: String },
}

/// A payment from one account to another, in stroops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub asset: Asset,
    pub memo: Option<String>,
}

impl Payment {
    /// A native XLM payment with no memo.
    pub fn native(from: &str, to: &str, amount: u64) -> Self {
        Payment {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            asset: Asset::Native,
            memo: None,
        }
    }
}

/// A very small wallet: who owns it, its address, and its balance in stroops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wallet {
    pub owner: String,
    pub address: String,
    pub balance: u64,
}

impl Wallet {
    pub fn new(owner: &str, address: &str, balance: u64) -> Self {
        Wallet {
            owner: owner.to_string(),
            address: address.to_string(),
            balance,
        }
    }
}

/// Where a submitted transaction is right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxStatus {
    Pending,
    Success,
    Failed,
    Expired,
}

/// Errors the wallet can return.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletError {
    InsufficientFunds { needed: u64, available: u64 },
    InvalidAmount(String),
}
