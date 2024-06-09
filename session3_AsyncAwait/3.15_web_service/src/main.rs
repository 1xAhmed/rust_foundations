use axum::response::Html;
use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/json", get(hello_json))
        .route("/post", post(hello_post));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use serde::Serialize;
#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_text() -> Html<String> {
    // const HTML: &str = include_str!("hello.html");
    let path = std::path::Path::new("src/hello.html");
    let content = tokio::fs::read_to_string(path).await;
    Html(content.expect("REASON"))
}

async fn hello_json() -> axum::Json<HelloJson> {
    let message = HelloJson {
        message: "Hi from JSON".to_string(),
    };
    axum::Json(message)
}

async fn hello_post() -> Html<String> {
    Html("Hello from post".to_string())
}
