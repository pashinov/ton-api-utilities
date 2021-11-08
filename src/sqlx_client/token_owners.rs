use anyhow::Result;

use crate::models::*;
use crate::sqlx_client::*;

impl SqlxClient {
    pub async fn create_token_owner(&self, token_owner: TokenOwnerDb) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO token_owners (address, owner_account_workchain_id, owner_account_hex, root_address, code_hash, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT DO NOTHING"#,
            token_owner.address,
            token_owner.owner_account_workchain_id,
            token_owner.owner_account_hex,
            token_owner.root_address,
            token_owner.code_hash,
            token_owner.created_at
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_token_owner_by_owner_account(
        &self,
        owner_account_workchain_id: i32,
        owner_account_hex: &str,
    ) -> Result<TokenOwnerDb> {
        let res = sqlx::query_as!(
            TokenOwnerDb,
            r#"SELECT address, owner_account_workchain_id, owner_account_hex, root_address, code_hash, created_at
            FROM token_owners
            WHERE owner_account_workchain_id = $1 AND owner_account_hex = $2"#,
            owner_account_workchain_id,
            owner_account_hex
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(res)
    }
}
