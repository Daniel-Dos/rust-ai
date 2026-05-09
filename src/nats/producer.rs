use async_nats::{Client, PublishError};
use async_nats::client::FlushError;
use serde::{Deserialize, Serialize};

const SUBJECT: &str = "demo.events";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NatsEvent {
    pub message: String,
}

pub async fn nats_producer(nats_client: &Client, message: String) -> Result<(), PublishError> {
    println!("producing message: {:?}",nats_client.server_info().server_id);
    nats_client.publish(SUBJECT,message.into()).await
}

pub async fn nats_flush(nats_client: &Client) -> Result<(), FlushError> {
    nats_client.flush().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nats_event_serialization() {
        let event = NatsEvent {
            message: "Test message".to_string(),
        };
        let json = serde_json::to_string(&event);
        assert!(json.is_ok());
        let json_str = json.unwrap();
        assert!(json_str.contains("Test message"));
    }

    #[test]
    fn test_nats_event_deserialization() {
        let json = r#"{"message":"Hello"}"#;
        let event: Result<NatsEvent, _> = serde_json::from_str(json);
        assert!(event.is_ok());
        assert_eq!(event.unwrap().message, "Hello");
    }

    #[test]
    fn test_nats_event_roundtrip() {
        let original = NatsEvent {
            message: "Roundtrip test".to_string(),
        };
        let json_result = serde_json::to_string(&original);
        assert!(json_result.is_ok());
        let json_str = json_result.unwrap();
        let decoded_result: Result<NatsEvent, _> = serde_json::from_str(&json_str);
        assert!(decoded_result.is_ok());
        assert_eq!(original.message, decoded_result.unwrap().message);
    }
}