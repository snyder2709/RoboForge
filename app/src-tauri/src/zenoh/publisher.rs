use std::time::{SystemTime, UNIX_EPOCH};

pub async fn publish_cmd(
    session: &zenoh::Session,
    topic: &str,
    action: &str,
    servos: Option<Vec<f64>>,
) -> Result<(), String> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    let payload = match servos {
        Some(s) => serde_json::json!({ "action": action, "servos": s, "ts": ts }),
        None => serde_json::json!({ "action": action, "ts": ts }),
    };
    session
        .put(topic, payload.to_string())
        .await
        .map_err(|e| e.to_string())
}
