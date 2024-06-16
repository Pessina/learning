use tokio::net::TcpListener;
use url_shortener::modules::routes::setup_router;

#[tokio::main]
async fn main() {
    let app = setup_router().await;
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
