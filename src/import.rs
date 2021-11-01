use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;

use crate::models::*;
use crate::sqlx_client::*;
use crate::utils::*;

pub async fn run_import(service_id: Option<String>, path: PathBuf, key: [u8; 32]) -> Result<()> {
    let pool = get_pg_pool().await?;
    let sqlx_client = SqlxClient::new(pool);

    let service_id = match service_id {
        Some(service_id) => Some(ServiceId::from_str(&service_id)?),
        None => None,
    };

    import_addresses(&service_id, &sqlx_client, path.clone(), key).await?;
    import_transactions(&service_id, &sqlx_client, path.clone()).await?;

    Ok(())
}

async fn import_transactions(
    service_id: &Option<ServiceId>,
    sqlx_client: &SqlxClient,
    mut path: PathBuf,
) -> Result<()> {
    path.push("transactions.jsonl");

    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let mut transaction: TransactionDb = serde_json::from_str(line?.as_str())?;
        if let Some(service_id) = service_id {
            transaction.service_id = *service_id;
        }
        sqlx_client.create_transaction(transaction).await?;
    }

    Ok(())
}

async fn import_addresses(
    service_id: &Option<ServiceId>,
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
        if let Some(service_id) = service_id {
            address.service_id = *service_id;
        }
        let private_key = encrypt(&address.private_key, key, &address.id)?;
        address.private_key = private_key;
        addresses.push(address);
    }

    sqlx_client.create_addresses(addresses).await?;

    Ok(())
}
