use async_nats::Client;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::nats::producer::{NatsEvent, nats_producer};

const OPENCODE_URL: &str = "http://localhost:4096";

#[derive(Serialize)]
struct OpenCodeRequest {
    parts: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct MessageResponse {
    parts: Vec<serde_json::Value>,
}

#[derive(Clone)]
pub struct OpenCodeService {
    http_client: HttpClient,
    opencode_url: String,
    session_id: String,
    available: bool,
    last_response: Arc<Mutex<String>>,
}

impl OpenCodeService {
    pub async fn new(http_client: HttpClient) -> Self {
        let opencode_url = OPENCODE_URL.to_string();
        let mut session_id = String::new();
        let mut available = false;

        let url = format!("{}/session", opencode_url);
        match http_client.post(&url)
            .json(&serde_json::json!({ "title": "NATS API" }))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    available = true;
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

        if !available {
            println!("Running without OpenCode (NATS only)");
        }

        Self {
            http_client,
            opencode_url,
            session_id,
            available,
            last_response: Arc::new(Mutex::new(String::new())),
        }
    }

    pub async fn send_message(&self, nats_client: &Client, msg: &str) -> Result<String, ()> {
        if !self.available {
            return Ok("Message received successfully".to_string());
        }

        let request = OpenCodeRequest {
            parts: vec![serde_json::json!({
                "type": "text",
                "text": msg
            })],
        };

        let url = format!("{}/session/{}/message", self.opencode_url, self.session_id);

        match self.http_client.post(&url)
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
                                        let payload = match serde_json::to_string(&event) {
                                            Ok(p) => p,
                                            Err(e) => {
                                                eprintln!("Failed to serialize: {}", e);
                                                return Ok("Message received successfully".to_string());
                                            }
                                        };
                                        if let Err(e) = nats_producer(nats_client, payload).await {
                                            eprintln!("Failed to publish to NATS: {}", e);
                                        }
                                        if let Err(e) = nats_client.flush().await {
                                            eprintln!("Failed to publish to NATS: {}", e);
                                        }
                                        let mut guard = self.last_response.lock().await;
                                        *guard = text_content.to_string();
                                        println!("Published to NATS. {}", nats_client.server_info().server_name);
                                        return Ok("Message received successfully".to_string());
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

        Ok("Message received successfully".to_string())
    }

    pub async fn get_last_response(&self) -> String {
        let guard = self.last_response.lock().await;
        guard.clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = NatsEvent {
            message: "Hello NATS!".to_string(),
        };
        let json = serde_json::to_string(&event);
        assert!(json.is_ok());
        assert!(json.unwrap().contains("Hello NATS!"));
    }

    #[test]
    fn test_event_serialization_empty_message() {
        let event = NatsEvent {
            message: String::new(),
        };
        let json = serde_json::to_string(&event);
        assert!(json.is_ok());
        assert_eq!(json.unwrap(), r#"{"message":""}"#);
    }

    #[test]
    fn test_event_roundtrip() {
        let original = NatsEvent {
            message: "Test message".to_string(),
        };
        let json_result = serde_json::to_string(&original);
        assert!(json_result.is_ok());
        let json_str = json_result.unwrap();
        let decoded_result: Result<NatsEvent, _> = serde_json::from_str(&json_str);
        assert!(decoded_result.is_ok());
        assert_eq!(original.message, decoded_result.unwrap().message);
    }
}