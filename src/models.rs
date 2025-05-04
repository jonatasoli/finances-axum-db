use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use bigdecimal::BigDecimal;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub kind: String,
    pub amount: BigDecimal,
    pub description: String,
    pub tag: Option<String>,
}

#[derive(Deserialize)]
pub struct NewTransaction {
    pub kind: String,
    pub amount: BigDecimal,
    pub description: String,
    pub tag: Option<String>,
}
