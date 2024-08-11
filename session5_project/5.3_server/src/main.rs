mod collector;
use axum::extract::Path;
use axum::response::Html;
use axum::{routing::get, Extension, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read .env
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    let handle = tokio::spawn(collector::data_collector(pool.clone()));

    // Launch Axum here
    // Start the web server
    let app = Router::new()
        .route("/", get(index))
        .route("/collector.html", get(collector))
        .route("/api/all", get(show_all))
        .route("/api/collectors", get(show_collectors))
        .route("/api/collector/:uuid", get(collector_data))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // Wait for the data collector to finish
    handle.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.
    Ok(())
}

async fn index() -> Html<String> {
    let path = std::path::Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn collector() -> Html<String> {
    let path = std::path::Path::new("src/collector.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize)]
pub struct DataPoint {
    id: i32,
    collector_id: String,
    received: i64,
    total_memory: i64,
    used_memory: i64,
    average_cpu: f32,
}

pub async fn show_all(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}

#[derive(FromRow, Debug, Serialize)]
pub struct Collector {
    id: i32,
    collector_id: String,
    last_seen: i64,
}

pub async fn show_collectors(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<Collector>> {
    const SQL: &str = "SELECT
    DISTINCT(id) AS id,
    collector_id,
    (SELECT MAX(received) FROM timeseries WHERE collector_id = ts.collector_id) AS last_seen
    FROM timeseries ts";
    Json(
        sqlx::query_as::<_, Collector>(SQL)
            .fetch_all(&pool)
            .await
            .unwrap(),
    )
}

pub async fn collector_data(
    Extension(pool): Extension<sqlx::SqlitePool>,
    uuid: Path<String>,
) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>(
        "SELECT * FROM timeseries WHERE collector_id = ? ORDER BY received",
    )
    .bind(uuid.as_str())
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(rows)
}
