use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::zenoh::FsmState;

#[derive(Clone, Serialize)]
pub struct ZenohStateEvent {
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct ZenohLogEvent {
    pub level: String,
    pub message: String,
    pub ts: f64,
}

#[derive(Clone, Serialize)]
pub struct ZenohTestDone {
    pub success: bool,
    pub count: u32,
}

pub fn unix_ts() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

pub fn emit_state(app: &AppHandle, fsm: &FsmState) {
    let error = if let FsmState::Error(e) = fsm { Some(e.clone()) } else { None };
    app.emit("zenoh-state", ZenohStateEvent {
        state: fsm.name().to_string(),
        error,
    }).ok();
}

pub fn emit_log(app: &AppHandle, level: &str, message: &str) {
    app.emit("zenoh-log", ZenohLogEvent {
        level: level.to_string(),
        message: message.to_string(),
        ts: unix_ts(),
    }).ok();
}
