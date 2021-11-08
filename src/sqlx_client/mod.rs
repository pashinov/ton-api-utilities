use sqlx::PgPool;

mod addresses;
mod token_owners;
mod token_transactions;
mod transactions;

#[derive(Clone)]
pub struct SqlxClient {
    pool: PgPool,
}

impl SqlxClient {
    pub fn new(pool: PgPool) -> SqlxClient {
        SqlxClient { pool }
    }
}
