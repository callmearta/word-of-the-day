// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{ActivationPolicy, CustomMenuItem, SystemTrayEvent, SystemTrayMenu};
use tauri::{AppHandle, Manager, SystemTray, Window};
extern crate time;
use chrono::Local;
use fix_path_env;
use std::{thread, time::Duration};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]

fn main() {
    let _ = fix_path_env::fix(); // <---- Add this
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![update_title])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            apply_vibrancy(
                &window,
                NSVisualEffectMaterial::HudWindow,
                Some(NSVisualEffectState::Active),
                Some(24.00),
            )
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            let _ = app.tray_handle().set_title("Word of the day");
            let _ = app.set_activation_policy(ActivationPolicy::Accessory);
            let main_window = app.get_window("main").unwrap();

            check_date_change(main_window);

            Ok(())
        })
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn check_date_change(main_window: Window) {
    thread::spawn(move || {
        let mut last_checked_date = Local::now().date_naive();

        loop {
            // Perform the check at the desired interval (e.g., every 60 seconds)
            thread::sleep(Duration::from_secs(60));
            // Check if the date has changed
            let current_date = Local::now().date_naive();
            if current_date != last_checked_date {
                // If the date has changed, emit an event to the frontend
                main_window
                    .emit_and_trigger("date-changed", current_date.format("%Y-%m-%d").to_string())
                    .expect("Failed to emit event");

                // Update the last checked date
                last_checked_date = current_date;
            }
        }
    });
}

#[tauri::command]
fn update_title(title: String, app_handle: AppHandle) {
    let _ = app_handle.tray_handle().set_title(&title);
}
