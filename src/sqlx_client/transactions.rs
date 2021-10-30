use anyhow::Result;

use crate::models::*;
use crate::sqlx_client::*;

impl SqlxClient {
    pub async fn create_transaction(&self, transaction: TransactionDb) -> Result<TransactionDb> {
        sqlx::query_as!(TransactionDb,
                r#"
                 INSERT INTO transactions
            (id, service_id, message_hash, transaction_hash, transaction_lt, transaction_timeout, transaction_scan_lt,
            transaction_timestamp, sender_workchain_id, sender_hex, account_workchain_id, account_hex, messages, messages_hash,
            data, original_value, original_outputs, value, fee, balance_change, direction, status, error, aborted, bounce,
            created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27)
            RETURNING id, service_id as "service_id: _", message_hash, transaction_hash, transaction_lt, transaction_timeout,
                transaction_scan_lt, transaction_timestamp, sender_workchain_id, sender_hex, account_workchain_id, account_hex, messages, messages_hash, data,
                original_value, original_outputs, value, fee, balance_change, direction as "direction: _", status as "status: _",
                error, aborted, bounce, created_at, updated_at"#,
                transaction.id,
                transaction.service_id as ServiceId,
                transaction.message_hash,
                transaction.transaction_hash,
                transaction.transaction_lt,
                transaction.transaction_timeout,
                transaction.transaction_scan_lt,
                transaction.transaction_timestamp,
                transaction.sender_workchain_id,
                transaction.sender_hex,
                transaction.account_workchain_id,
                transaction.account_hex,
                transaction.messages,
                transaction.messages_hash,
                transaction.data,
                transaction.original_value,
                transaction.original_outputs,
                transaction.value,
                transaction.fee,
                transaction.balance_change,
                transaction.direction as TonTransactionDirection,
                transaction.status as TonTransactionStatus,
                transaction.error,
                transaction.aborted,
                transaction.bounce,
                transaction.created_at,
                transaction.updated_at,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn get_all_transactions(&self, service_id: ServiceId) -> Result<Vec<TransactionDb>> {
        sqlx::query_as!(TransactionDb, r#"SELECT id, service_id as "service_id: _", message_hash, transaction_hash, transaction_lt, transaction_timeout,
                transaction_scan_lt, transaction_timestamp, sender_workchain_id, sender_hex, account_workchain_id, account_hex, messages, messages_hash, data,
                original_value, original_outputs, value, fee, balance_change, direction as "direction: _", status as "status: _",
                error, aborted, bounce, created_at, updated_at
                FROM transactions WHERE service_id = $1"#,
                service_id as ServiceId,
        )
            .fetch_all(&self.pool)
            .await
            .map_err(From::from)
    }
}
