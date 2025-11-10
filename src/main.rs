use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let static_files = ServeDir::new("static");

    let app = Router::new()
        .fallback_service(static_files);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
