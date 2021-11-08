use anyhow::Result;

use crate::models::*;
use crate::sqlx_client::*;

impl SqlxClient {
    pub async fn create_token_transaction(
        &self,
        transaction: TokenTransactionDb,
    ) -> Result<TokenTransactionDb> {
        sqlx::query_as!(TokenTransactionDb,
            r#" INSERT INTO token_transactions
            (id, service_id, transaction_hash, transaction_timestamp, message_hash, owner_message_hash, account_workchain_id, account_hex,
            value, root_address, payload, error, block_hash, block_time, direction, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            RETURNING id, service_id as "service_id: _", transaction_hash, transaction_timestamp, message_hash, owner_message_hash, account_workchain_id, account_hex,
            value, root_address, payload, error, block_hash, block_time, direction as "direction: _", status as "status: _", created_at, updated_at"#,
                transaction.id,
                transaction.service_id as ServiceId,
                transaction.transaction_hash,
                transaction.transaction_timestamp,
                transaction.message_hash,
                transaction.owner_message_hash,
                transaction.account_workchain_id,
                transaction.account_hex,
                transaction.value,
                transaction.root_address,
                transaction.payload,
                transaction.error,
                transaction.block_hash,
                transaction.block_time,
                transaction.direction as TonTransactionDirection,
                transaction.status as TonTokenTransactionStatus,
                transaction.created_at,
                transaction.updated_at,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn get_all_token_transactions(
        &self,
        service_id: ServiceId,
    ) -> Result<Vec<TokenTransactionDb>> {
        sqlx::query_as!(TokenTransactionDb, r#"SELECT id, service_id as "service_id: _", transaction_hash, transaction_timestamp, message_hash,
            owner_message_hash, account_workchain_id, account_hex, value, root_address, payload, error, block_hash, block_time, direction as "direction: _",
            status as "status: _", created_at, updated_at
            FROM token_transactions
            WHERE service_id = $1"#,
            service_id as ServiceId,
        )
            .fetch_all(&self.pool)
            .await
            .map_err(From::from)
    }
}
