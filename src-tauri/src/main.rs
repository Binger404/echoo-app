#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
use tauri::Manager;

use crate::ui::app::{handle_run_events, register_shortcut};
use crate::ui::menu::main_menu_builder;

mod commands;
mod events;
mod libs;
mod ui;

fn main() {
    let builder = tauri::Builder::default();
    let app = builder
        .menu(main_menu_builder())
        .setup(|app| {
            app.get_window("main").and_then(|win| {
                let pkg_info = app.package_info();
                let window_title = format!("{} - v{}", pkg_info.name, pkg_info.version);
                win.set_title(window_title.as_str()).ok()
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::fs::read_binary_file,
            commands::fs::write_binary_file,
            commands::os::get_system
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    register_shortcut(&app);
    app.run(handle_run_events);
}
