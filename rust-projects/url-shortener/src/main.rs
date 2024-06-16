use tokio::net::TcpListener;
use url_shortener::modules::setup;

#[tokio::main]
async fn main() {
    let app = setup::setup_server().await;
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
