
use std::collections::HashMap;
use std::sync::Mutex;

use tonic::{Request, Response, Status};
use tonic::transport::Server;


use kvstore::kv_store_server::{KvStore, KvStoreServer};
use kvstore::{PutRequest, PutResponse, GetRequest, GetResponse};


pub mod kvstore {
    tonic::include_proto!("kvstore");
}


#[derive(Debug)]
pub struct KvStoreService {
    pub db: Mutex<HashMap<String, String>>
}

impl Default for KvStoreService {
    fn default() -> KvStoreService {
        KvStoreService {
            db: Mutex::new(HashMap::new())
        }
    }
}

#[tonic::async_trait]
impl KvStore for KvStoreService {
    async fn put(&self, request: Request<PutRequest>) -> Result<Response<PutResponse>, Status> {
        println!("Recieved Put request: {:?}", request);
        let response = PutResponse {
            success: true,
        };
        let message = request.into_inner();
        let mut db = self.db.lock().unwrap();
        db.insert(message.key, message.value);
        drop(db);


        Ok(Response::new(response))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        println!("Get request: {:?}", request);
        let message = request.into_inner();

        let db = self.db.lock().unwrap();
        let response = GetResponse {
            value: db.get(&message.key).cloned()
        };
        drop(db);

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:6379".parse()?;
    let service = KvStoreService::default();

    Server::builder()
        .add_service(KvStoreServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
