use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout, Instant};

use crate::events::{emit_log, emit_state, ZenohTestDone};
use crate::zenoh::{FsmState, ZenohInner, ZenohState};
use crate::zenoh::publisher::publish_cmd;
use crate::swarm::scenario::scenario_frames;
use crate::swarm::coordinator::run_swarm_test;

pub async fn finish_test(app: &AppHandle, state: Arc<Mutex<ZenohInner>>, success: bool, count: u32) {
    let mut g = state.lock().await;
    if matches!(g.fsm, FsmState::Testing) {
        g.fsm = FsmState::Connected;
        emit_state(app, &g.fsm);
    }
    app.emit("zenoh-test-done", ZenohTestDone { success, count }).ok();
}

#[tauri::command]
pub async fn zenoh_run_test(
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
        run_single_test(app, inner, session, robot_id, duration_secs).await;
    });
    Ok(())
}

async fn run_single_test(
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
        if remaining.is_zero() { break; }
        match timeout(remaining, subscriber.recv_async()).await {
            Ok(Ok(sample)) => {
                count += 1;
                let mut buf = Vec::new();
                sample.payload().reader().read_to_end(&mut buf).ok();
                let text = String::from_utf8_lossy(&buf).to_string();
                let truncated = if text.len() > 120 { format!("{}…", &text[..120]) } else { text };
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

#[tauri::command]
pub async fn zenoh_run_swarm_test(
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
