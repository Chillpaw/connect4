use connect4_server::app;

#[tokio::main]
async fn main() {
    let app = app();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.expect("failed to bind 127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
