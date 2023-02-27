use kvstore::kv_store_client::KvStoreClient;
use kvstore::{GetRequest, PutRequest};
use rand::random;


pub mod kvstore {
    tonic::include_proto!("kvstore");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KvStoreClient::connect("http://127.0.0.1:6379").await?;
    let mut count = 0;
    loop {
        let key: i32 = random();
        let request = PutRequest {
            key: format!("testkey{}", key),
            value: format!("value{}", key)
        };
        let response = client.put(request).await?;
        println!("Put Response: {:?}", response);


        let request = GetRequest {
            key: format!("testkey{}", key)
        };
        let response = client.get(request).await?;
        println!("Get response: {:?}, {:?}", response, count);
        count += 1;
    }
    Ok(())
}