extern crate dotenv;
use std::{
    env,
    io,
    error::Error,
};

use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TickerInfo {
    results: Vec<PriceInfo>,
    ticker: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PriceInfo {
    c: f64,
}


struct Config {
    polygon_api_key: String,
}

impl Config {
    fn read_api_key() -> Config {
        dotenv::dotenv().expect("Failed to read .env file");

        let polygon_api_key = env::var("POLYGON_API_KEY").expect("Could not read key");

        Config { polygon_api_key }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    loop {

        println!("Please input a ticker symbol to get the price of a stock.");

        let url = construct_url();

        let response = reqwest::get(url).await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<TickerInfo>().await {
                    Ok(parsed) => {

                        let stock_info = stock_info(parsed);

                        println!("Price for {} is {}", stock_info.ticker, stock_info.ticker_price);
                                                
                    },
                    Err(_) => {
                        println!("The provided input did not match a ticker symbol.");
                        continue;
                    },
                };
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token");
            }
            other => {
                panic!("Uh oh! Something unexpected happened: {:?}", other);
            }
        };    
        
        break Ok(());

    }
}

struct StockInfo {
    ticker_price: f64,
    ticker: String,
}

fn stock_info(ticker_info: TickerInfo) -> StockInfo {

    let mut ticker_price = 0.0;

    for price in ticker_info.results {
        ticker_price = price.c;
    }
    
    StockInfo { ticker_price, ticker: ticker_info.ticker }
}

fn construct_url() -> String {
    let mut ticker_input = String::new();

    io::stdin() 
        .read_line(&mut ticker_input)
        .expect("Failed to read input");
    
    let config = Config::read_api_key();

    let url = format!("https://api.polygon.io/v2/aggs/ticker/{}/prev?adjusted=true&apiKey={}", ticker_input.to_uppercase(), config.polygon_api_key);

    return url;
}





