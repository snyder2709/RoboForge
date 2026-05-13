use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout, Instant};

use crate::events::{emit_log, emit_state, unix_ts, ZenohTestDone};
use crate::zenoh::{FsmState, ZenohInner};
use crate::zenoh::publisher::publish_cmd;
use super::scenario::ScenarioFrame;

pub async fn finish_test(app: &AppHandle, state: Arc<Mutex<ZenohInner>>, success: bool, count: u32) {
    let mut g = state.lock().await;
    if matches!(g.fsm, FsmState::Testing) {
        g.fsm = FsmState::Connected;
        emit_state(app, &g.fsm);
    }
    app.emit("zenoh-test-done", ZenohTestDone { success, count }).ok();
}

pub async fn run_swarm_test(
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

    for &id in &robot_ids {
        publish_cmd(&session, &format!("robot/{id}/cmd"), "reset", None).await.ok();
        emit_log(&app, "SENT", &format!("robot/{id}/cmd — reset"));
    }
    sleep(Duration::from_millis(500)).await;

    let swarm_payload = serde_json::json!({
        "scenario": &scenario_name,
        "robot_count": robot_count,
        "ts": unix_ts()
    });
    session.put("swarm/scenario", swarm_payload.to_string()).await.ok();
    emit_log(&app, "SENT", &format!("swarm/scenario — {scenario_name} ×{robot_count}"));

    let total = frames.len();
    for (fi, frame) in frames.iter().enumerate() {
        for &id in &robot_ids {
            publish_cmd(&session, &format!("robot/{id}/cmd"), "move_servos", Some(frame.servos.clone()))
                .await
                .ok();
        }
        emit_log(&app, "SENT", &format!("frame {}/{total} → {robot_count} robots", fi + 1));
        sleep(Duration::from_millis(frame.duration_ms)).await;
    }

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
                        emit_log(&app, "RECV", &format!("robot/{id}/state #{}", *cnt));
                    }
                }
            }
            _ => break,
        }
    }

    for &id in &robot_ids {
        publish_cmd(&session, &format!("robot/{id}/cmd"), "reset", None).await.ok();
    }
    emit_log(&app, "SENT", "all robots — reset (cleanup)");

    let total_recv: u32 = counts.values().sum();
    let responded = counts.len() as u32;
    emit_log(&app, "DONE", &format!("Swarm complete — {responded}/{robot_count} robots, {total_recv} states total"));
    finish_test(&app, state, responded == robot_count, total_recv).await;
}
