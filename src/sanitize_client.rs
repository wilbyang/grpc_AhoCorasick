
mod generated;
use generated::sanitize::{sanitize_service_client, SanitizeRequest};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = sanitize_service_client::SanitizeServiceClient::connect("http://[::1]:50051").await?;
    
    let request = tonic::Request::new(SanitizeRequest{
        text: "Hello, xxhh, maple, this is apple!".to_string(),
    });
    let response = client.sanitize(request).await?;
    println!("RESPONSE={:?}", response.into_inner().filtered_text);
    Ok(())
    


}