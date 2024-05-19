// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent, Wry};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .system_tray(SystemTray::new())
        .on_system_tray_event(system_tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// system tray event fn
fn system_tray_event(app: &AppHandle<Wry>, e: SystemTrayEvent) {
    match e {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let windows = app.windows();
            for (key, value) in windows {
                if key == "login" || key == "home" {
                    value.show().unwrap();
                    value.unminimize().unwrap();
                    value.set_focus().unwrap();
                }
            }
        }
        SystemTrayEvent::RightClick {
            position: p,
            size: s,
            ..
        } => {
            app.emit_all("tray_menu", (p, s)).unwrap();
        }
        _ => {}
    }
}
