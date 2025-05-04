use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub async fn init_db(url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;
    Ok(pool)
}
