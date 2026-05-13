use tauri::State;
use crate::robot::{RobotDescriptor, RobotState};
use crate::zenoh::ZenohState;

#[derive(serde::Serialize)]
pub struct RobotInfo {
    pub state: RobotState,
    pub descriptor: Option<RobotDescriptor>,
}

#[tauri::command]
pub async fn get_robots(state: State<'_, ZenohState>) -> Result<Vec<RobotInfo>, String> {
    let g = state.0.lock().await;
    let mut list: Vec<RobotInfo> = g
        .states
        .values()
        .map(|rs| RobotInfo {
            state: rs.clone(),
            descriptor: g.descriptors.get(&rs.id).cloned(),
        })
        .collect();
    list.sort_by_key(|r| r.state.id);
    Ok(list)
}
