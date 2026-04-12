const NATS_URL: &str = "nats://localhost:4222";
const SUBJECT: &str = "demo.events";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
struct Event {
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = Event {
            message: "Hello NATS!".to_string(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("Hello NATS!"));
    }

    #[test]
    fn test_event_serialization_empty_message() {
        let event = Event {
            message: String::new(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert_eq!(json, r#"{"message":""}"#);
    }

    #[test]
    fn test_event_roundtrip() {
        let original = Event {
            message: "Test message".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let decoded: Event = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let message = std::env::args().nth(1).unwrap_or_else(|| "Hello NATS!".to_string());
    
    let client = async_nats::connect(NATS_URL).await?;
    let event = Event { message: message.clone() };
    let payload = serde_json::to_string(&event)
        .map_err(|e| async_nats::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
    client.publish(SUBJECT, payload.into()).await?;
    client.flush().await?;
    println!("Published: {}", message);
    Ok(())
}
