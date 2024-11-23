use ::anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

// Wrapper type to hold the DB pool.
#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    // Initialize a DB connection and return the Pool.
    pub async fn new(db_str: &str, max_pool_size: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_pool_size)
            .connect(db_str)
            .await?;
        Ok(Db { pool })
    }

    // Run DB Migrations using the sqlx migrate! macro.
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}
