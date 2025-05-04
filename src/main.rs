use axum::{Router, routing::{get, post, put, delete}, Extension};
use tokio::net::TcpListener;
use tower_http::services::fs::ServeDir;
use tera::Tera;
mod db;
mod models;
mod handlers;
mod forms;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let tera = Tera::new("templates/**/*")?;

    let database_url = std::env::var("DATABASE_URL")?;
    let db_pool = db::init_db(&database_url).await?;

    let app = Router::new()
        .route("/", get(handlers::view_index))
        .route("/transactions/{id}/edit", get(handlers::edit_tx))
        .route("/transactions", get(handlers::view_list_txs))
        .route("/transactions", post(handlers::create_tx))
        .route("/transactions/{id}/update", post(handlers::update_tx))
        .route("/transactions/{id}", delete(handlers::delete_tx))
        .nest_service("/static", ServeDir::new("static"))
        .layer(Extension(db_pool))
        .layer(Extension(tera));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("Server running in http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
