#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

#[tauri::command]
fn backend_add(number: i32) -> i32 {
    // Note: these commands block the main thread and hang the UI until they return.
    // If you need to run a long-running task, use async command instead.
    println!("Backend was called with an argument: {}", number);
    number + 2
}

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![backend_add])
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let main_window = _app.get_window("main").unwrap();
                main_window.open_devtools();
            }
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
