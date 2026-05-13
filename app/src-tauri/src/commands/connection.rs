use std::io::Read;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

use crate::events::{emit_log, emit_state};
use crate::robot::{RobotDescriptor, RobotState};
use crate::zenoh::{FsmState, ZenohState};

#[tauri::command]
pub async fn zenoh_connect(
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

    let config_json = format!(r#"{{"mode":"client","connect":{{"endpoints":["{endpoint}"]}}}}"#);
    let config = zenoh::Config::from_json5(&config_json).map_err(|e| e.to_string())?;

    match zenoh::open(config).await {
        Ok(session) => {
            {
                let mut g = state.0.lock().await;
                g.fsm = FsmState::Connected;
                emit_state(&app, &g.fsm);
                emit_log(&app, "INFO", &format!("Connected to {endpoint}"));
            }

            // Subscriber: robot states
            let app_s = app.clone();
            let inner_s = Arc::clone(&state.0);
            let sess_s = session.clone();
            tokio::spawn(async move {
                if let Ok(sub) = sess_s.declare_subscriber("robot/*/state").await {
                    loop {
                        match sub.recv_async().await {
                            Ok(sample) => {
                                let mut buf = Vec::new();
                                sample.payload().reader().read_to_end(&mut buf).ok();
                                if let Ok(rs) = serde_json::from_slice::<RobotState>(&buf) {
                                    inner_s.lock().await.states.insert(rs.id, rs.clone());
                                    app_s.emit("robot-state", &rs).ok();
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
            });

            // Subscriber: robot descriptors
            let app_d = app.clone();
            let inner_d = Arc::clone(&state.0);
            let sess_d = session.clone();
            tokio::spawn(async move {
                if let Ok(sub) = sess_d.declare_subscriber("robot/*/descriptor").await {
                    loop {
                        match sub.recv_async().await {
                            Ok(sample) => {
                                let mut buf = Vec::new();
                                sample.payload().reader().read_to_end(&mut buf).ok();
                                if let Ok(desc) = serde_json::from_slice::<RobotDescriptor>(&buf) {
                                    inner_d.lock().await.descriptors.insert(desc.id, desc.clone());
                                    app_d.emit("robot-descriptor", &desc).ok();
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
            });

            state.0.lock().await.session = Some(session);
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
pub async fn zenoh_disconnect(app: AppHandle, state: State<'_, ZenohState>) -> Result<(), String> {
    let mut g = state.0.lock().await;
    g.session = None;
    g.states.clear();
    g.descriptors.clear();
    g.fsm = FsmState::Disconnected;
    emit_state(&app, &g.fsm);
    emit_log(&app, "INFO", "Disconnected");
    Ok(())
}

#[tauri::command]
pub async fn zenoh_get_state(state: State<'_, ZenohState>) -> Result<String, String> {
    let g = state.0.lock().await;
    Ok(g.fsm.name().to_string())
}
