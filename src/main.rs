use crate::rest::rest_api;

mod rest;
mod service;
mod nats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("NATS Pub/Sub Demo");
    println!();
    println!("Terminal 1 - Consumer:");
    println!("  cargo run --bin consumer");
    println!();
    println!("Terminal 2 - Producer:");
    println!("  cargo run --bin producer <message>");

    rest_api::main().await
}
