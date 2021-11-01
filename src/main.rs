use std::convert::TryInto;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Context, Result};
use argh::FromArgs;
use argon2::password_hash::PasswordHasher;

use ton_api_utility::export::*;
use ton_api_utility::import::*;
use ton_api_utility::models::*;

#[tokio::main]
async fn main() -> Result<()> {
    run(argh::from_env()).await
}
async fn run(app: App) -> Result<()> {
    match app.command {
        Subcommand::Export(run) => run.execute().await,
        Subcommand::Import(run) => run.execute().await,
    }
}

#[derive(Debug, PartialEq, FromArgs)]
#[argh(description = "TON API migration utility")]
struct App {
    #[argh(subcommand)]
    command: Subcommand,
}

#[derive(Debug, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Export(CmdExport),
    Import(CmdImport),
}

#[derive(Debug, PartialEq, FromArgs)]
/// Export addresses and transactions
/// from DB to jsonl
#[argh(subcommand, name = "export")]
struct CmdExport {
    /// service id
    #[argh(option, short = 'i')]
    id: String,
    /// export path
    #[argh(option, short = 'p')]
    path: Option<String>,
    /// secret
    #[argh(option, short = 'k')]
    key: String,
    /// salt
    #[argh(option, short = 's')]
    salt: String,
}

impl CmdExport {
    async fn execute(self) -> Result<()> {
        let service_id = ServiceId::from_str(&self.id)?;

        let mut options = argon2::ParamsBuilder::default();
        let options = options
            .output_len(32) //chacha key size
            .and_then(|x| x.clone().params())
            .unwrap();

        // Argon2 with default params (Argon2id v19)
        let argon2 =
            argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, options);

        let key = argon2
            .hash_password(self.key.as_bytes(), &self.salt)
            .unwrap()
            .hash
            .context("No hash")?
            .as_bytes()
            .try_into()?;

        let path = match self.path {
            Some(path) => PathBuf::from_str(&path)?,
            None => PathBuf::from_str("./data")?,
        };

        // Prepare data folder
        std::fs::create_dir_all(&path)?;

        run_export(service_id, path, key).await
    }
}

#[derive(Debug, PartialEq, FromArgs)]
/// Import addresses and transactions
/// from jsonl to DB
#[argh(subcommand, name = "import")]
struct CmdImport {
    /// service id
    #[argh(option, short = 'i')]
    id: Option<String>,
    /// export path
    #[argh(option, short = 'p')]
    path: Option<String>,
    /// secret
    #[argh(option, short = 'k')]
    key: String,
    /// salt
    #[argh(option, short = 's')]
    salt: String,
}

impl CmdImport {
    async fn execute(self) -> Result<()> {
        let service_id = self.id;

        let mut options = argon2::ParamsBuilder::default();
        let options = options
            .output_len(32) //chacha key size
            .and_then(|x| x.clone().params())
            .unwrap();

        // Argon2 with default params (Argon2id v19)
        let argon2 =
            argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, options);

        let key = argon2
            .hash_password(self.key.as_bytes(), &self.salt)
            .unwrap()
            .hash
            .context("No hash")?
            .as_bytes()
            .try_into()?;

        let path = match self.path {
            Some(path) => PathBuf::from_str(&path)?,
            None => PathBuf::from_str("./data")?,
        };

        run_import(service_id, path, key).await
    }
}
