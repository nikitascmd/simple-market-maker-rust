use std::env;
mod shared_state;

fn main() {
    println!("Hello, world!");

    dotenvy::dotenv().ok();

    let api_key = env::var("API_KEY").unwrap();
    let api_secret = env::var("API_SECRET").unwrap();
}
