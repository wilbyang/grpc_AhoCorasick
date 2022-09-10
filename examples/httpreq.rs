#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    get_weather("https://httpbin.org/get?xx=hh").await?;
    Ok(())
    
}

async fn get_weather(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(api_key).await?;
    let body = response.text().await?;
    println!("body = {:?}", body);
    Ok(())
}