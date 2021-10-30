use anyhow::Result;

use crate::models::*;
use crate::sqlx_client::*;

impl SqlxClient {
    pub async fn create_address(&self, address: AddressDb) -> Result<AddressDb> {
        sqlx::query_as!(AddressDb,
                r#"INSERT INTO address
                (id, service_id, workchain_id, hex, base64url, public_key, private_key, account_type, custodians,
                confirmations, custodians_public_keys, balance, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8::twa_account_type, $9, $10, $11, $12, $13, $14)
                RETURNING
                id, service_id as "service_id: _", workchain_id, hex, base64url, public_key, private_key, account_type as "account_type: _",
                custodians, confirmations, custodians_public_keys, balance, created_at, updated_at"#,
                address.id,
                address.service_id as ServiceId,
                address.workchain_id,
                address.hex,
                address.base64url,
                address.public_key,
                address.private_key,
                address.account_type as AccountType,
                address.custodians,
                address.confirmations,
                address.custodians_public_keys,
                address.balance,
                address.created_at,
                address.updated_at
            )
            .fetch_one(&self.pool)
            .await
            .map_err(From::from)
    }

    pub async fn create_addresses(&self, addresses: Vec<AddressDb>) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for address in addresses.iter() {
            let _ = sqlx::query_as!(AddressDb,
                r#"INSERT INTO address
                (id, service_id, workchain_id, hex, base64url, public_key, private_key, account_type, custodians,
                confirmations, custodians_public_keys, balance, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8::twa_account_type, $9, $10, $11, $12, $13, $14)
                RETURNING
                id, service_id as "service_id: _", workchain_id, hex, base64url, public_key, private_key, account_type as "account_type: _",
                custodians, confirmations, custodians_public_keys, balance, created_at, updated_at"#,
                address.id,
                address.service_id as ServiceId,
                address.workchain_id,
                address.hex,
                address.base64url,
                address.public_key,
                address.private_key,
                address.account_type as AccountType,
                address.custodians,
                address.confirmations,
                address.custodians_public_keys,
                address.balance,
                address.created_at,
                address.updated_at
            )
                .fetch_one(&mut tx)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn get_all_addresses(&self, service_id: ServiceId) -> Result<Vec<AddressDb>> {
        sqlx::query_as!(AddressDb,
                r#"SELECT id, service_id as "service_id: _", workchain_id, hex, base64url, public_key, private_key, account_type as "account_type: _",
                custodians, confirmations, custodians_public_keys, balance, created_at, updated_at
                FROM address WHERE service_id = $1"#,
                service_id as ServiceId,
            )
            .fetch_all(&self.pool)
            .await
            .map_err(From::from)
    }
}
