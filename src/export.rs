use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use bigdecimal::BigDecimal;

use crate::models::*;
use crate::sqlx_client::*;
use crate::utils::*;

pub async fn run_export(service_id: ServiceId, path: PathBuf, key: [u8; 32]) -> Result<()> {
    let pool = get_pg_pool().await?;
    let sqlx_client = SqlxClient::new(pool);

    export_transactions(service_id, &sqlx_client, path.clone()).await?;
    export_addresses(service_id, &sqlx_client, path.clone(), key).await?;

    Ok(())
}

async fn export_transactions(
    service_id: ServiceId,
    sqlx_client: &SqlxClient,
    mut path: PathBuf,
) -> Result<()> {
    let transactions = sqlx_client.get_all_transactions(service_id).await?;

    path.push("transactions.jsonl");

    let mut output = File::create(path)?;
    for transaction in transactions.iter() {
        let transaction = serde_json::to_string(transaction)? + "\n";
        output.write(transaction.as_bytes())?;
    }

    output.flush()?;

    Ok(())
}

async fn export_addresses(
    service_id: ServiceId,
    sqlx_client: &SqlxClient,
    mut path: PathBuf,
    key: [u8; 32],
) -> Result<()> {
    let mut addresses = sqlx_client.get_all_addresses(service_id).await?;

    path.push("addresses.jsonl");

    let mut output = File::create(path)?;
    for address in addresses.iter_mut() {
        let private_key = decrypt(&address.private_key, key, &address.id)?;
        address.private_key = base64::encode(private_key);
        address.balance = BigDecimal::from(0);

        let address = serde_json::to_string(address)? + "\n";

        output.write(address.as_bytes())?;
    }

    output.flush()?;

    Ok(())
}
