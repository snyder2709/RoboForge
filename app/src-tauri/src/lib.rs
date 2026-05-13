mod events;
mod robot;
mod zenoh;
mod swarm;
mod commands;

use crate::zenoh::ZenohState;
use crate::commands::{connection, robots, test, urdf};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! RoboForge is ready.")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ZenohState::new())
        .setup(|_app| Ok(()))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            connection::zenoh_connect,
            connection::zenoh_disconnect,
            connection::zenoh_get_state,
            test::zenoh_run_test,
            test::zenoh_run_swarm_test,
            robots::get_robots,
            urdf::read_urdf_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running RoboForge")
}
