use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransactionForm {
    pub kind: String,
    pub amount: f64,
    pub description: String,
    pub tag: Option<String>,
}
