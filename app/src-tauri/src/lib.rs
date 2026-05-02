use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout, Instant};

// ─── FSM ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum FsmState {
    Disconnected,
    Connecting,
    Connected,
    Testing,
    Error(String),
}

impl FsmState {
    fn name(&self) -> &'static str {
        match self {
            FsmState::Disconnected => "disconnected",
            FsmState::Connecting => "connecting",
            FsmState::Connected => "connected",
            FsmState::Testing => "testing",
            FsmState::Error(_) => "error",
        }
    }
}

struct ZenohInner {
    fsm: FsmState,
    session: Option<zenoh::Session>,
}

pub struct ZenohState(Arc<Mutex<ZenohInner>>);

// ─── Event bus payloads ───────────────────────────────────────────────────────

#[derive(Clone, Serialize)]
struct ZenohStateEvent {
    state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Clone, Serialize)]
struct ZenohLogEvent {
    level: String,
    message: String,
    ts: f64,
}

#[derive(Clone, Serialize)]
struct ZenohTestDone {
    success: bool,
    count: u32,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn unix_ts() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

fn emit_state(app: &AppHandle, fsm: &FsmState) {
    let error = if let FsmState::Error(e) = fsm { Some(e.clone()) } else { None };
    app.emit("zenoh-state", ZenohStateEvent { state: fsm.name().to_string(), error }).ok();
}

fn emit_log(app: &AppHandle, level: &str, message: &str) {
    app.emit("zenoh-log", ZenohLogEvent {
        level: level.to_string(),
        message: message.to_string(),
        ts: unix_ts(),
    })
    .ok();
}

// ─── Tauri commands ───────────────────────────────────────────────────────────

#[tauri::command]
async fn zenoh_connect(
    app: AppHandle,
    state: State<'_, ZenohState>,
    endpoint: String,
) -> Result<(), String> {
    {
        let mut g = state.0.lock().await;
        match g.fsm {
            FsmState::Connected | FsmState::Connecting | FsmState::Testing => {
                return Err("Already connected or busy".to_string());
            }
            _ => {}
        }
        g.fsm = FsmState::Connecting;
        emit_state(&app, &g.fsm);
    }

    let config_json = format!(
        r#"{{"mode":"client","connect":{{"endpoints":["{endpoint}"]}}}}"#
    );
    let config = zenoh::Config::from_json5(&config_json).map_err(|e| e.to_string())?;

    match zenoh::open(config).await {
        Ok(session) => {
            let mut g = state.0.lock().await;
            g.session = Some(session);
            g.fsm = FsmState::Connected;
            emit_state(&app, &g.fsm);
            emit_log(&app, "INFO", &format!("Connected to {endpoint}"));
            Ok(())
        }
        Err(e) => {
            let msg = e.to_string();
            let mut g = state.0.lock().await;
            g.fsm = FsmState::Error(msg.clone());
            emit_state(&app, &g.fsm);
            emit_log(&app, "ERROR", &format!("Connection failed: {msg}"));
            Err(msg)
        }
    }
}

#[tauri::command]
async fn zenoh_disconnect(app: AppHandle, state: State<'_, ZenohState>) -> Result<(), String> {
    let mut g = state.0.lock().await;
    g.session = None;
    g.fsm = FsmState::Disconnected;
    emit_state(&app, &g.fsm);
    emit_log(&app, "INFO", "Disconnected");
    Ok(())
}

#[tauri::command]
async fn zenoh_get_state(state: State<'_, ZenohState>) -> Result<String, String> {
    let g = state.0.lock().await;
    Ok(g.fsm.name().to_string())
}

#[tauri::command]
async fn zenoh_run_test(
    app: AppHandle,
    state: State<'_, ZenohState>,
    robot_id: u32,
    duration_secs: u64,
) -> Result<(), String> {
    let session = {
        let mut g = state.0.lock().await;
        match &g.fsm {
            FsmState::Connected => {
                let s = g.session.as_ref().ok_or("No session")?.clone();
                g.fsm = FsmState::Testing;
                emit_state(&app, &g.fsm);
                s
            }
            FsmState::Testing => return Err("Test already running".to_string()),
            _ => return Err("Not connected".to_string()),
        }
    };

    let inner = Arc::clone(&state.0);
    tokio::spawn(async move {
        run_test(app, inner, session, robot_id, duration_secs).await;
    });

    Ok(())
}

// ─── Test sequence ────────────────────────────────────────────────────────────

async fn run_test(
    app: AppHandle,
    state: Arc<Mutex<ZenohInner>>,
    session: zenoh::Session,
    robot_id: u32,
    duration_secs: u64,
) {
    emit_log(&app, "INFO", &format!("Test start: robot {robot_id}, {duration_secs}s"));

    let cmd_topic = format!("robot/{robot_id}/cmd");
    let state_topic = format!("robot/{robot_id}/state");

    let subscriber = match session.declare_subscriber(&state_topic).await {
        Ok(s) => s,
        Err(e) => {
            emit_log(&app, "ERROR", &format!("Subscribe failed: {e}"));
            finish_test(&app, state, false, 0).await;
            return;
        }
    };

    if let Err(e) = publish_cmd(&session, &cmd_topic, "reset", None).await {
        emit_log(&app, "ERROR", &e);
        finish_test(&app, state, false, 0).await;
        return;
    }
    emit_log(&app, "SENT", &format!("{cmd_topic} — reset"));

    sleep(Duration::from_millis(500)).await;

    let servos = vec![45.0f64; 20];
    if let Err(e) = publish_cmd(&session, &cmd_topic, "move_servos", Some(servos)).await {
        emit_log(&app, "ERROR", &e);
        finish_test(&app, state, false, 0).await;
        return;
    }
    emit_log(&app, "SENT", &format!("{cmd_topic} — move_servos 45°×20"));

    let mut count = 0u32;
    let start = Instant::now();
    let deadline = Duration::from_secs(duration_secs);

    loop {
        let remaining = deadline.saturating_sub(start.elapsed());
        if remaining.is_zero() {
            break;
        }
        match timeout(remaining, subscriber.recv_async()).await {
            Ok(Ok(sample)) => {
                count += 1;
                let mut buf = Vec::new();
                use std::io::Read;
                sample.payload().reader().read_to_end(&mut buf).ok();
                let text = String::from_utf8_lossy(&buf).to_string();
                let truncated = if text.len() > 120 {
                    format!("{}…", &text[..120])
                } else {
                    text
                };
                emit_log(&app, "RECV", &format!("{state_topic} #{count}: {truncated}"));
            }
            _ => break,
        }
    }

    publish_cmd(&session, &cmd_topic, "reset", None).await.ok();
    emit_log(&app, "SENT", &format!("{cmd_topic} — reset (cleanup)"));

    emit_log(&app, "DONE", &format!("Test complete — {count} state messages received"));
    finish_test(&app, state, count > 0, count).await;
}

async fn publish_cmd(
    session: &zenoh::Session,
    topic: &str,
    action: &str,
    servos: Option<Vec<f64>>,
) -> Result<(), String> {
    let payload = match servos {
        Some(s) => serde_json::json!({ "action": action, "servos": s, "ts": unix_ts() }),
        None => serde_json::json!({ "action": action, "ts": unix_ts() }),
    };
    session
        .put(topic, payload.to_string())
        .await
        .map_err(|e| e.to_string())
}

async fn finish_test(
    app: &AppHandle,
    state: Arc<Mutex<ZenohInner>>,
    success: bool,
    count: u32,
) {
    let mut g = state.lock().await;
    if matches!(g.fsm, FsmState::Testing) {
        g.fsm = FsmState::Connected;
        emit_state(app, &g.fsm);
    }
    app.emit("zenoh-test-done", ZenohTestDone { success, count }).ok();
}

// ─── Swarm scenarios ──────────────────────────────────────────────────────────

struct ScenarioFrame {
    duration_ms: u64,
    servos: Vec<f64>,
}

fn scenario_frames(name: &str) -> Option<Vec<ScenarioFrame>> {
    fn n() -> Vec<f64> { vec![90.0; 20] }
    fn f(mut v: Vec<f64>, updates: &[(usize, f64)]) -> Vec<f64> {
        for &(i, val) in updates { v[i] = val; }
        v
    }
    Some(match name {
        "wave" => vec![
            ScenarioFrame { duration_ms: 400, servos: f(n(), &[(6,45.),(7,70.),(8,90.)]) },
            ScenarioFrame { duration_ms: 200, servos: f(n(), &[(6,45.),(7,70.),(8,130.)]) },
            ScenarioFrame { duration_ms: 200, servos: f(n(), &[(6,45.),(7,70.),(8,55.)]) },
            ScenarioFrame { duration_ms: 200, servos: f(n(), &[(6,45.),(7,70.),(8,130.)]) },
            ScenarioFrame { duration_ms: 200, servos: f(n(), &[(6,45.),(7,70.),(8,55.)]) },
            ScenarioFrame { duration_ms: 400, servos: n() },
        ],
        "walk" => vec![
            ScenarioFrame { duration_ms: 350, servos: f(n(), &[(10,65.),(12,115.),(13,110.),(15,115.),(17,65.),(18,70.),(2,115.),(6,65.)]) },
            ScenarioFrame { duration_ms: 350, servos: f(n(), &[(10,115.),(12,65.),(13,70.),(15,65.),(17,115.),(18,110.),(2,65.),(6,115.)]) },
            ScenarioFrame { duration_ms: 350, servos: f(n(), &[(10,65.),(12,115.),(13,110.),(15,115.),(17,65.),(18,70.),(2,115.),(6,65.)]) },
            ScenarioFrame { duration_ms: 350, servos: f(n(), &[(10,115.),(12,65.),(13,70.),(15,65.),(17,115.),(18,110.),(2,65.),(6,115.)]) },
            ScenarioFrame { duration_ms: 350, servos: n() },
        ],
        "jump" => vec![
            ScenarioFrame { duration_ms: 300, servos: f(n(), &[(10,120.),(12,140.),(13,65.),(15,120.),(17,140.),(18,65.)]) },
            ScenarioFrame { duration_ms: 150, servos: f(n(), &[(10,60.),(12,60.),(13,115.),(15,60.),(17,60.),(18,115.)]) },
            ScenarioFrame { duration_ms: 250, servos: n() },
            ScenarioFrame { duration_ms: 200, servos: f(n(), &[(10,110.),(12,120.),(13,75.),(15,110.),(17,120.),(18,75.)]) },
            ScenarioFrame { duration_ms: 300, servos: n() },
        ],
        _ => return None,
    })
}

#[tauri::command]
async fn zenoh_run_swarm_test(
    app: AppHandle,
    state: State<'_, ZenohState>,
    robot_count: u32,
    scenario: String,
) -> Result<(), String> {
    let frames = scenario_frames(&scenario)
        .ok_or_else(|| format!("Unknown scenario: {scenario}"))?;

    let session = {
        let mut g = state.0.lock().await;
        match &g.fsm {
            FsmState::Connected => {
                let s = g.session.as_ref().ok_or("No session")?.clone();
                g.fsm = FsmState::Testing;
                emit_state(&app, &g.fsm);
                s
            }
            FsmState::Testing => return Err("Test already running".to_string()),
            _ => return Err("Not connected".to_string()),
        }
    };

    let inner = Arc::clone(&state.0);
    tokio::spawn(async move {
        run_swarm_test(app, inner, session, robot_count, scenario, frames).await;
    });
    Ok(())
}

async fn run_swarm_test(
    app: AppHandle,
    state: Arc<Mutex<ZenohInner>>,
    session: zenoh::Session,
    robot_count: u32,
    scenario_name: String,
    frames: Vec<ScenarioFrame>,
) {
    let robot_ids: Vec<u32> = (1..=robot_count).collect();
    emit_log(&app, "INFO", &format!("Swarm test: scenario={scenario_name}, robots={robot_count}"));

    let subscriber = match session.declare_subscriber("robot/*/state").await {
        Ok(s) => s,
        Err(e) => {
            emit_log(&app, "ERROR", &format!("Subscribe failed: {e}"));
            finish_test(&app, state, false, 0).await;
            return;
        }
    };

    // Reset all robots
    for &id in &robot_ids {
        publish_cmd(&session, &format!("robot/{id}/cmd"), "reset", None).await.ok();
        emit_log(&app, "SENT", &format!("robot/{id}/cmd — reset"));
    }
    sleep(Duration::from_millis(500)).await;

    // Broadcast scenario to swarm topic (for 3D UI visualization)
    let swarm_payload = serde_json::json!({ "scenario": &scenario_name, "robot_count": robot_count, "ts": unix_ts() });
    session.put("swarm/scenario", swarm_payload.to_string()).await.ok();
    emit_log(&app, "SENT", &format!("swarm/scenario — {scenario_name} ×{robot_count}"));

    // Play frames
    let total = frames.len();
    for (fi, frame) in frames.iter().enumerate() {
        for &id in &robot_ids {
            publish_cmd(&session, &format!("robot/{id}/cmd"), "move_servos", Some(frame.servos.clone())).await.ok();
        }
        let frame_num = fi + 1;
        emit_log(&app, "SENT", &format!("frame {frame_num}/{total} → {robot_count} robots"));
        sleep(Duration::from_millis(frame.duration_ms)).await;
    }

    // Collect state responses (1s window)
    let mut counts: HashMap<u32, u32> = HashMap::new();
    let deadline = Duration::from_secs(1);
    let start = Instant::now();
    loop {
        let remaining = deadline.saturating_sub(start.elapsed());
        if remaining.is_zero() { break; }
        match timeout(remaining, subscriber.recv_async()).await {
            Ok(Ok(sample)) => {
                let mut buf = Vec::new();
                use std::io::Read;
                sample.payload().reader().read_to_end(&mut buf).ok();
                if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&buf) {
                    if let Some(id) = val.get("id").and_then(|v| v.as_u64()) {
                        let cnt = counts.entry(id as u32).or_insert(0);
                        *cnt += 1;
                        let cnt_val = *cnt;
                        emit_log(&app, "RECV", &format!("robot/{id}/state #{cnt_val}"));
                    }
                }
            }
            _ => break,
        }
    }

    // Cleanup
    for &id in &robot_ids {
        publish_cmd(&session, &format!("robot/{id}/cmd"), "reset", None).await.ok();
    }
    emit_log(&app, "SENT", "all robots — reset (cleanup)");

    let total_recv: u32 = counts.values().sum();
    let responded = counts.len() as u32;
    emit_log(&app, "DONE", &format!("Swarm complete — {responded}/{robot_count} robots, {total_recv} states total"));
    finish_test(&app, state, responded == robot_count, total_recv).await;
}

// ─── Entry point ──────────────────────────────────────────────────────────────

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! RoboForge is ready.")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ZenohState(Arc::new(Mutex::new(ZenohInner {
            fsm: FsmState::Disconnected,
            session: None,
        }))))
        .setup(|_app| Ok(()))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            zenoh_connect,
            zenoh_disconnect,
            zenoh_get_state,
            zenoh_run_test,
            zenoh_run_swarm_test,
        ])
        .run(tauri::generate_context!())
        .expect("error while running RoboForge")
}
