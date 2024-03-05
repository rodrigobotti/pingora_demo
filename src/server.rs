use axum::Router;
use axum::{http::StatusCode, routing::get};
use std::env;

async fn get_server_name() -> Result<String, StatusCode> {
    match env::var("SERVER_NAME") {
        Ok(val) => Ok(val),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").expect("PORT envvar not set");

    let app = Router::new().route("/", get(get_server_name));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("Server listening on port {port}\n");

    axum::serve(listener, app).await.unwrap();
}
