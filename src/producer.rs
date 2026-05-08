mod opencode_service;
mod rest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rest::main().await
}