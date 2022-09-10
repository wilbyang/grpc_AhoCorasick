
mod generated;
use generated::sanitize::{sanitize_service_server::{SanitizeServiceServer, SanitizeService}, SanitizeRequest, SanitizeResponse};
fn main(){
    let mut client = sanitize_client::sanitize_client::SanitizeClient::connect("http://[::1]:50051").unwrap();
    let request = tonic::Request::new(sanitize_client::sanitize_request{
        text: "Hello, world!".to_string(),
    });
    let response = client.sanitize(request).await.unwrap();
    println!("RESPONSE={:?}", response);
}