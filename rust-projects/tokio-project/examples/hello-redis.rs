use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    client.set("name", "felipe".into()).await?;
    let result = client.get("name").await?;
    println!("got value from the server; result={:?}", result);

    client.set("age", "24".into()).await?;
    let result = client.get("age").await?;
    println!("got value from the server; result={:?}", result);

    Ok(())
}
