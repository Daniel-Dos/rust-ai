use futures_util::StreamExt;

const NATS_URL: &str = "nats://localhost:4222";
const SUBJECT: &str = "demo.events";

#[derive(serde::Deserialize, Debug, PartialEq)]
struct Event {
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_deserialization() {
        let json = r#"{"message":"Hello NATS!"}"#;
        let event: Result<Event, _> = serde_json::from_str(json);
        assert!(event.is_ok());
        assert_eq!(event.unwrap().message, "Hello NATS!");
    }

    #[test]
    fn test_event_deserialization_empty_message() {
        let json = r#"{"message":""}"#;
        let event: Result<Event, _> = serde_json::from_str(json);
        assert!(event.is_ok());
        assert_eq!(event.unwrap().message, "");
    }

    #[test]
    fn test_event_deserialization_invalid_json() {
        let json = r#"not json"#;
        let result: Result<Event, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect(NATS_URL).await?;
    let mut subscriber = client.subscribe(SUBJECT).await?;

    println!("Listening on '{}'...", SUBJECT);

    while let Some(message) = subscriber.next().await {
        match serde_json::from_slice::<Event>(&message.payload) {
            Ok(event) => println!("Received: {}", event.message),
            Err(e) => eprintln!("Failed to parse message: {}", e),
        }
    }

    Ok(())
}
