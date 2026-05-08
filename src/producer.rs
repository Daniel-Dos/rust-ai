use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use rand::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

const NAMES: &[&str] = &[
    "Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace", "Henry", "Iris", "Jack", "Karen",
    "Leo", "Mia", "Noah", "Olivia", "Peter", "Quinn", "Rose", "Sam", "Tina", "Uma", "Victor",
    "Wendy", "Xavier", "Yara", "Zack",
];

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
struct NatsEvent {
    message: String,
}

#[derive(Clone)]
struct AppState {
    nats_client: async_nats::Client,
    http_client: reqwest::Client,
    opencode_url: String,
    session_id: String,
    last_response: Arc<Mutex<String>>,
    opencode_available: bool,
}

#[derive(serde::Deserialize)]
struct MessageRequest {
    message: String,
}

#[derive(serde::Serialize)]
struct OpenCodeRequest {
    parts: Vec<serde_json::Value>,
}

#[derive(serde::Deserialize)]
struct MessageResponse {
    parts: Vec<serde_json::Value>,
}

const NATS_URL: &str = "nats://localhost:4222";
const SUBJECT: &str = "demo.events";
const API_PORT: u16 = 8080;
const OPENCODE_URL: &str = "http://localhost:4096";

async fn post_message(
    State(state): State<AppState>,
    Json(payload): Json<MessageRequest>,
) -> (StatusCode, Json<NatsEvent>) {
    let msg = payload.message.clone();
    
    let event = NatsEvent {
        message: msg.clone(),
    };
    let payload_str = serde_json::to_string(&event).unwrap();
    if let Err(e) = state.nats_client.publish(SUBJECT, payload_str.into()).await {
        eprintln!("Failed to publish to NATS: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(NatsEvent { message: String::new() }));
    }
    if let Err(e) = state.nats_client.flush().await {
        eprintln!("Failed to flush NATS: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(NatsEvent { message: String::new() }));
    }
    println!("Published to NATS: {}", msg);
    
    if !state.opencode_available {
        return (StatusCode::OK, Json(NatsEvent { message: "Message received successfully".to_string() }));
    }
    
    let request = OpenCodeRequest {
        parts: vec![serde_json::json!({
            "type": "text",
            "text": msg.clone()
        })],
    };
    
    let opencode_url = format!("{}/session/{}/message", state.opencode_url, state.session_id);
    
    match state.http_client.post(&opencode_url)
        .json(&request)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                let text = resp.text().await.unwrap_or_default();
                
                if let Ok(msg_resp) = serde_json::from_str::<MessageResponse>(&text) {
                    for part in &msg_resp.parts {
                        if let Some(part_type) = part.get("type").and_then(|v| v.as_str()) {
                            if part_type == "text" {
                                if let Some(text_content) = part.get("text").and_then(|v| v.as_str()) {
                                    let event = NatsEvent {
                                        message: text_content.to_string(),
                                    };
                                    let payload_str = serde_json::to_string(&event).unwrap();
                                    if let Err(e) = state.nats_client.publish(SUBJECT, payload_str.into()).await {
                                        eprintln!("Failed to publish to NATS: {}", e);
                                    } 
                                    let mut guard = state.last_response.lock().await;
                                    *guard = text_content.to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to call OpenCode: {}", e);
        }
    }
    
    (StatusCode::OK, Json(NatsEvent { message: "Message received successfully".to_string() }))
}

async fn get_message(
    State(state): State<AppState>,
) -> (StatusCode, Json<NatsEvent>) {
    let guard = state.last_response.lock().await;
    if guard.is_empty() {
        (StatusCode::NOT_FOUND, Json(NatsEvent { message: String::new() }))
    } else {
        (StatusCode::OK, Json(NatsEvent { message: guard.clone() }))
    }
}

fn random_name() -> String {
    let mut rng = rand::thread_rng();
    NAMES[rng.gen_range(0..NAMES.len())].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = NatsEvent {
            message: "Hello NATS!".to_string(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("Hello NATS!"));
    }

    #[test]
    fn test_event_serialization_empty_message() {
        let event = NatsEvent {
            message: String::new(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert_eq!(json, r#"{"message":""}"#);
    }

    #[test]
    fn test_event_roundtrip() {
        let original = NatsEvent {
            message: "Test message".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let decoded: NatsEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--random" {
        let name = random_name();
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

    let opencode_url = OPENCODE_URL.to_string();
    let mut session_id = String::new();
    let mut opencode_available = false;
    
    let url = format!("{}/session", opencode_url);
    match http_client.post(&url)
        .json(&serde_json::json!({ "title": "NATS API" }))
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                opencode_available = true;
                let text = response.text().await.unwrap_or_default();
                
                if let Ok(session) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(id) = session.get("id").and_then(|v| v.as_str()) {
                        session_id = id.to_string();
                        println!("Created session: {}", id);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not connect to OpenCode: {}", e);
        }
    }
    
    if session_id.is_empty() {
        session_id = "first".to_string();
    }

    if !opencode_available {
        println!("Running without OpenCode (NATS only)");
    }

    let state = AppState {
        nats_client,
        http_client,
        opencode_url,
        session_id,
        last_response: Arc::new(Mutex::new(String::new())),
        opencode_available,
    };

    let app = Router::new()
        .route("/message", post(post_message))
        .route("/message", get(get_message))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", API_PORT);
    println!("REST API listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}