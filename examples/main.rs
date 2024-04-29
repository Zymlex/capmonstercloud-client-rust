use capmonster_cloud_client::CapMonsterCloudClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = CapMonsterCloudClient::new(env!("CMC_KEY")).unwrap();
    
    println!("{}", client.get_balance_async().await.unwrap());
    
    Ok(())
}