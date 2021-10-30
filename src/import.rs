use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

use anyhow::Result;

use crate::models::*;
use crate::sqlx_client::*;
use crate::utils::*;

pub async fn run_import(path: PathBuf, key: [u8; 32]) -> Result<()> {
    let pool = get_pg_pool().await?;
    let sqlx_client = SqlxClient::new(pool);

    import_addresses(&sqlx_client, path.clone(), key).await?;
    import_transactions(&sqlx_client, path.clone()).await?;

    Ok(())
}

async fn import_transactions(sqlx_client: &SqlxClient, mut path: PathBuf) -> Result<()> {
    path.push("transactions.jsonl");

    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let transaction: TransactionDb = serde_json::from_str(line?.as_str())?;
        sqlx_client.create_transaction(transaction).await?;
    }

    Ok(())
}

async fn import_addresses(
    sqlx_client: &SqlxClient,
    mut path: PathBuf,
    key: [u8; 32],
) -> Result<()> {
    path.push("addresses.jsonl");

    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut addresses = Vec::new();
    for line in reader.lines() {
        let mut address: AddressDb = serde_json::from_str(line?.as_str())?;
        let private_key = encrypt(&address.private_key, key, &address.id)?;
        address.private_key = private_key;
        addresses.push(address);
    }

    sqlx_client.create_addresses(addresses).await?;

    Ok(())
}
