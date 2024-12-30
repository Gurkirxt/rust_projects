use std::time::Instant;
use tokio::time::interval;
use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        match run_speed_test(&client).await {
            Ok((download_speed, upload_speed)) => {
                println!("Download: {:.2} Mbps, Upload: {:.2} Mbps", download_speed, upload_speed);
                if download_speed < 10.0 || upload_speed < 10.0 {
                    println!("Warning: Network speed is below the threshold!");
                }
            }
            Err(e) => {
                eprintln!("Error running speed test: {}", e);
            }
        }
    }
}

async fn run_speed_test(client: &Client) -> Result<(f64, f64), reqwest::Error> {
    let download_speed = measure_download_speed(client).await?;
    let upload_speed = measure_upload_speed(client).await?;
    Ok((download_speed, upload_speed))
}

async fn measure_download_speed(client: &Client) -> Result<f64, reqwest::Error> {
    let url = "https://download.blender.org/peach/bigbuckbunny_movies/big_buck_bunny_1080p_h264.mov"; // A large file for download test
    let start = Instant::now();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    let duration = start.elapsed();
    let download_speed = (bytes.len() as f64 * 8.0) / duration.as_secs_f64() / 1_000_000.0; // Convert to Mbps
    Ok(download_speed)
}

async fn measure_upload_speed(client: &Client) -> Result<f64, reqwest::Error> {
    let url = "https://httpbin.org/post"; // A test endpoint for upload
    let data = vec![0u8; 10_000_000]; // 10 MB of data
    let start = Instant::now();
    let _response = client.post(url).body(data.clone()).send().await?;
    let duration = start.elapsed();
    let upload_speed = (data.len() as f64 * 8.0) / duration.as_secs_f64() / 1_000_000.0; // Convert to Mbps
    Ok(upload_speed)
}
