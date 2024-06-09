use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Command>(32);
    let tx2 = tx.clone();

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: "Name".to_string(),
            val: "Felipe".into(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "Name".to_string(),
            resp: resp_tx,
        };

        tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
