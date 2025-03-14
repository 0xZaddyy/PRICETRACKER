use reqwest::{self};
use serde::Deserialize;
use tokio::time::{sleep, Duration};

#[derive(Deserialize)]
struct ApiResponse {
    bitcoin: CurrencyData,

}

#[derive(Deserialize)]

struct CurrencyData {
    usd: f64,
}

#[tokio::main]
async fn main() {
    let url =  "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
    loop {
        match reqwest::get(url).await {
            Ok(response) => match response.json::<ApiResponse>().await  {
                Ok(data) => println!("Current Bitcoin Price: ${}", data.bitcoin.usd),
                Err(_) => println!("Failed to parse response"),
            },
            Err(_) => println!("Failed to fetch price, check your internet"),
        }
        sleep(Duration::from_secs(10)).await; // Wait 10 seconds before updating
    }
}
