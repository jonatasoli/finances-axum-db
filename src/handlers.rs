use axum::{http::StatusCode, response::{Html, IntoResponse, Redirect}, Extension, Form, Json};
use serde::Deserialize;
use sqlx::PgPool;
use tera::{Tera};
use crate::models::{Transaction};
use uuid::Uuid;


#[derive(Deserialize)]
pub struct TransactionForm {
    pub kind: String,
    pub amount: f64,
    pub description: String,
    pub tag: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTransactionForm {
    pub id: Uuid,
    pub kind: String,
    pub amount: f64,
    pub description: String,
    pub tag: Option<String>,
}

pub async fn create_tx(
    Extension(pool): Extension<PgPool>,
    Extension(tera): Extension<Tera>,
    Form(form): Form<TransactionForm>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let tx = sqlx::query_as::<_, Transaction>(
        "INSERT INTO transactions (id, kind, amount, description, tag) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(id)
    .bind(&form.kind)
    .bind(form.amount)
    .bind(&form.description)
    .bind(&form.tag)
    .fetch_one(&pool)
    .await
    .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("transactions", &vec![tx]);
    let rendered = tera.render("transaction_row.html", &ctx).unwrap();
    Html(rendered)
}

pub async fn list_txs(
    Extension(pool): Extension<PgPool>
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let txs = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions ORDER BY timestamp DESC"
    )
    .fetch_all(&pool).await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(txs))
}

pub async fn update_tx(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<UpdateTransactionForm>,
) -> Result<Redirect, (StatusCode, String)> {
    sqlx::query(
        "UPDATE transactions SET kind=$1, amount=$2, description=$3, tag=$4 WHERE id=$5"
    )
    .bind(&form.kind)
    .bind(form.amount)
    .bind(&form.description)
    .bind(&form.tag)
    .bind(form.id)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Redirect::to("/"))
}

pub async fn delete_tx(
    Extension(pool): Extension<PgPool>,
    axum::extract::Path(id): axum::extract::Path<Uuid>
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM transactions WHERE id=$1")
        .bind(id)
        .execute(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

pub async fn view_index(
    Extension(pool): Extension<PgPool>,
    Extension(tera): Extension<Tera>,
) -> impl IntoResponse {
    let txs = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions ORDER BY timestamp DESC")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("transactions", &txs);
    let rendered = tera.render("index.html", &ctx).unwrap();
    Html(rendered)
}

pub async fn view_list_txs(
    Extension(pool): Extension<PgPool>,
    Extension(tera): Extension<Tera>
) -> impl IntoResponse {
    let txs = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions ORDER BY timestamp DESC")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("transactions", &txs);
    let rendered = tera.render("transaction_row.html", &ctx).unwrap();
    Html(rendered)
}

pub async fn edit_tx(
    Extension(pool): Extension<PgPool>,
    Extension(tera): Extension<Tera>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> impl IntoResponse {
    let tx = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("transaction", &tx);
    let rendered = tera.render("edit_transaction.html", &ctx).unwrap();
    Html(rendered)
}
