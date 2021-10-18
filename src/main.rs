use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context, Error, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chacha20poly1305::aead::AeadMut;
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use clap::{AppSettings, Clap};
use csv::Reader;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::types::Uuid;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    #[clap(short, long)]
    input: PathBuf,
    #[clap(short, long)]
    key: String,
    #[clap(short, long)]
    salt: Option<String>,
    #[clap(short, long)]
    with_headers: bool,
}

#[tokio::main(worker_threads = 16)]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Options::parse();
    let Options {
        input,
        key,
        salt,
        with_headers,
    } = args;
    let key_string = key;
    let reader = csv::ReaderBuilder::new()
        .has_headers(with_headers)
        .from_path(input)
        .context("Couldn't read input file")?;

    let database_url =
        env::var("DATABASE_URL").context("The DATABASE_URL environment variable must be set")?;
    let db_pool_size = env::var("DATABASE_POOL")
        .context("The DATABASE_POOL environment variable must be set")?
        .parse::<u32>()
        .context("failed to get pg pool")?;

    let pool = PgPoolOptions::new()
        .max_connections(db_pool_size)
        .connect(&database_url)
        .await
        .context("fail pg pool")?;

    let sqlx_client = SqlxClient::new(pool);

    let salt = match salt {
        Some(salt) => SaltString::new(&salt).map_err(Error::msg)?,
        None => SaltString::generate(&mut OsRng),
    };
    println!("Password salt: {}", salt.as_str());

    let mut options = argon2::ParamsBuilder::default();
    let options = options
        .output_len(32) //chacha key size
        .and_then(|x| x.clone().params())
        .map_err(Error::msg)?;

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, options);

    let key = argon2
        .hash_password(key_string.as_bytes(), &salt)
        .map_err(Error::msg)?
        .hash
        .context("No hash")?
        .as_bytes()
        .try_into()?;
    run(sqlx_client, reader, key).await
}

async fn run(sqlx_client: SqlxClient, mut reader: Reader<File>, key: [u8; 32]) -> Result<()> {
    let mut buffer = Vec::new();
    let mut count = 0;
    let iter = reader.deserialize();
    for (i, result) in iter.enumerate() {
        let record: AddressDb = result.context("Failed mapping to AddressDb")?;
        let private_key = encrypt(&record.private_key, key, &record.id)?;

        let item = AddressDb {
            id: record.id,
            workchain_id: record.workchain_id,
            hex: record.hex.clone(),
            private_key,
        };

        buffer.push(item);

        if count == 1000 || iter.count() == i - 1 {
            count = 0;
            let buffer_copy = buffer.clone();
            let sqlx_client_copy = sqlx_client.clone();
            let count_copy = i;
            tokio::spawn(async move {
                if let Err(err) = sqlx_client_copy.update(buffer_copy).await {
                    println!("ERROR: Failed to make db transaction: {:?}", err);
                }
                println!("Counter: {}", count_copy);
            });
            buffer.clear();
        }

        count += 1;
    }

    Ok(())
}

fn encrypt(private_key: &str, key: [u8; 32], id: &uuid::Uuid) -> Result<String> {
    use chacha20poly1305::aead::NewAead;
    let nonce = Nonce::from_slice(&id.as_bytes()[0..12]);
    let key = chacha20poly1305::Key::from_slice(&key[..]);
    let mut encryptor = ChaCha20Poly1305::new(key);
    let res = encryptor
        .encrypt(nonce, base64::decode(&private_key)?.as_slice())
        .unwrap();

    Ok(base64::encode(res))
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct AddressDb {
    pub id: Uuid,
    pub workchain_id: i32,
    pub hex: String,
    pub private_key: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct NewAddressDb {
    pub id: Uuid,
    pub workchain_id: i32,
    pub hex: String,
}

#[derive(Clone)]
pub struct SqlxClient {
    pool: PgPool,
}

impl SqlxClient {
    pub fn new(pool: PgPool) -> SqlxClient {
        SqlxClient { pool }
    }

    pub async fn get_id(&self, workchain_id: i32, hex: &str) -> Result<Uuid> {
        sqlx::query!(
            r#"SELECT id
                FROM address
                WHERE workchain_id = $1 AND hex = $2"#,
            workchain_id,
            hex,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(From::from)
        .map(|x| x.id)
    }

    pub async fn update_private_key(
        &self,
        workchain_id: i32,
        hex: &str,
        private_key: &str,
    ) -> Result<()> {
        let _address = sqlx::query!(
            r#"UPDATE address SET private_key = $1
            WHERE workchain_id = $2 AND hex = $3"#,
            private_key,
            workchain_id,
            hex,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update(&self, items: Vec<AddressDb>) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for address in items.iter() {
            let _address = sqlx::query!(
                r#"UPDATE address SET private_key = $1
                WHERE workchain_id = $2 AND hex = $3"#,
                address.private_key,
                address.workchain_id,
                address.hex,
            )
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
