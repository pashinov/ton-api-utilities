use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, sqlx::Type, Copy)]
#[sqlx(type_name = "twa_account_type", rename_all = "PascalCase")]
pub enum AccountType {
    HighloadWallet,
    Wallet,
    SafeMultisig,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "twa_transaction_status", rename_all = "PascalCase")]
pub enum TonTransactionStatus {
    New,
    Done,
    PartiallyDone,
    Error,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "twa_transaction_direction", rename_all = "PascalCase")]
pub enum TonTransactionDirection {
    Send,
    Receive,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "twa_token_transaction_status", rename_all = "PascalCase")]
pub enum TonTokenTransactionStatus {
    New,
    Done,
    Error,
}
