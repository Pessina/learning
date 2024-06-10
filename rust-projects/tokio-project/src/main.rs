mod connection;

use core::panic;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type Db = Mutex<HashMap<String, Bytes>>;
type SharedDb = Arc<Vec<Db>>;

fn new_shared_db(num_shards: usize) -> SharedDb {
    let mut db = Vec::with_capacity(num_shards);

    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }

    Arc::new(db)
}

fn hash<T: Hash>(t: T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[tokio::main]
async fn main() {
    let lister = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = new_shared_db(5);

    loop {
        let (socket, _) = lister.accept().await.unwrap();
        let db = db.clone();

        tokio::spawn(async move { process(socket, db).await });
    }
}

async fn process(socket: TcpStream, db: SharedDb) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut shard = db[hash(cmd.key()) as usize % db.len()].lock().unwrap();
                shard.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let shard = db[hash(cmd.key()) as usize % db.len()].lock().unwrap();
                if let Some(value) = shard.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
