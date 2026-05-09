use crate::service::opencode_service::{OpenCodeService};
use crate::nats::producer::NatsEvent;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    nats_client: async_nats::Client,
    opencode_service: Arc<OpenCodeService>,
}

#[derive(Deserialize)]
struct MessageRequest {
    message: String,
}

const NATS_URL: &str = "nats://localhost:4222";
const SUBJECT: &str = "demo.events";
const API_PORT: u16 = 8080;

async fn post_message(
    State(state): State<AppState>,
    Json(payload): Json<MessageRequest>,
) -> (StatusCode, Json<NatsEvent>) {
    let msg = payload.message.clone();

    let result = state
        .opencode_service
        .send_message(&state.nats_client, &msg)
        .await;

    match result {
        Ok(response) => (StatusCode::OK, Json(NatsEvent { message: response })),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(NatsEvent { message: String::new() })),
    }
}

async fn get_message(
    State(state): State<AppState>,
) -> (StatusCode, Json<NatsEvent>) {
    let response = state.opencode_service.get_last_response().await;
    if response.is_empty() {
        (StatusCode::NOT_FOUND, Json(NatsEvent { message: String::new() }))
    } else {
        (StatusCode::OK, Json(NatsEvent { message: response }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_request_deserialization() {
        let json = r#"{"message":"Hello World"}"#;
        let req: Result<MessageRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok());
        assert_eq!(req.unwrap().message, "Hello World");
    }

    #[test]
    fn test_message_request_empty() {
        let json = r#"{"message":""}"#;
        let req: Result<MessageRequest, _> = serde_json::from_str(json);
        assert!(req.is_ok());
        assert_eq!(req.unwrap().message, "");
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--random" {
        use rand::Rng;

        let names: &[&str] = &[
            "Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace", "Henry", "Iris", "Jack", "Karen",
            "Leo", "Mia", "Noah", "Olivia", "Peter", "Quinn", "Rose", "Sam", "Tina", "Uma", "Victor",
            "Wendy", "Xavier", "Yara", "Zack",
        ];

        let name = names[rand::thread_rng().gen_range(0..names.len())].to_string();
        let client = async_nats::connect(NATS_URL).await?;
        let event = NatsEvent { message: name.clone() };
        let payload = serde_json::to_string(&event)?;
        client.publish(SUBJECT, payload.into()).await?;
        client.flush().await?;
        println!("Published: {}", name);
        return Ok(());
    }

    if args.len() > 1 {
        let message = args[1].clone();
        let client = async_nats::connect(NATS_URL).await?;
        let event = NatsEvent { message: message.clone() };
        let payload = serde_json::to_string(&event)?;
        client.publish(SUBJECT, payload.into()).await?;
        client.flush().await?;
        println!("Published: {}", message);
        return Ok(());
    }

    let http_client = reqwest::Client::new();
    let nats_client = async_nats::connect(NATS_URL).await?;

    let opencode_service = Arc::new(OpenCodeService::new(http_client).await);

    let state = AppState {
        nats_client,
        opencode_service,
    };

    let app = Router::new()
        .route("/message", post(post_message))
        .route("/message", get(get_message))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", API_PORT);
    println!("REST API listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}