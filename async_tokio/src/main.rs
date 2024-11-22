use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    match fetch_url(url).await {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

