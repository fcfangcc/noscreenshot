// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate tracing;

pub mod commands;
pub mod tracing_config;

use std::{fs, io::Read, process};

use tauri::{http::ResponseBuilder, Manager, Window};
use tauri::{CustomMenuItem, Menu, Submenu, WindowBuilder};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn screenshot(window: Window, id: String) -> (String, Vec<String>) {
    let tmp_dir = window.app_handle().path_resolver().app_data_dir().unwrap();

    (
        "fullscreen".to_string(),
        commands::screenshots_fullscreens(tmp_dir, &id),
    )
}

#[tauri::command]
fn clear_temp(window: Window, id: String) -> Result<(), String> {
    let cache_dir = window.app_handle().path_resolver().app_data_dir().unwrap();
    let temp_dir = cache_dir.join(commands::ScreenshotTemp::new(id).dirname());

    if temp_dir.exists() {
        info!("delete tmp file. {}", temp_dir.display());
        fs::remove_dir_all(temp_dir).map_err(|e| format!("{}", e))?;
    }
    Ok(())
}

#[tauri::command]
fn logger(level: &str, message: &str) {
    match level {
        "info" => info!(target: "webapp", "{}", message),
        "error" => error!(target: "webapp", "{}", message),
        "warning" | "warn" => warn!(target: "webapp", "{}", message),
        _ => error!(target: "webapp", "level:{} message:{}", level, message),
    }
}

fn menu() -> Menu {
    let quit = CustomMenuItem::new("appdir".to_string(), "AppDir");
    let submenu = Submenu::new("Tools", Menu::new().add_item(quit));
    Menu::new().add_submenu(submenu)
}

fn main() {
    // todo: system tray
    #[allow(clippy::single_match)]
    tauri::Builder::default()
        .on_menu_event(|event| match event.menu_item_id() {
            "appdir" => {
                let log_dir = event
                    .window()
                    .app_handle()
                    .path_resolver()
                    .app_data_dir()
                    .unwrap()
                    .display()
                    .to_string();

                #[cfg(target_os = "windows")]
                process::Command::new("explorer")
                    .args([&log_dir])
                    .spawn()
                    .unwrap();

                #[cfg(not(target_os = "windows"))]
                error!("Open logfile only in windows.")
            }
            _ => {}
        })
        .setup(|app| {
            // init log
            let log_dir = app.path_resolver().app_log_dir().unwrap();
            tracing_config::init(&log_dir.display().to_string());
            // init main window and meun
            WindowBuilder::new(
                app,
                "main".to_string(),
                tauri::WindowUrl::App("index.html".into()),
            )
            .title("NoScreenshot")
            .resizable(true)
            .inner_size(800.0, 600.0)
            .menu(menu())
            .build()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet, screenshot, clear_temp, logger
        ])
        .register_uri_scheme_protocol("fullscreen", move |app, request| {
            // prepare our response
            let response = ResponseBuilder::new().header("Access-Control-Allow-Origin", "*");
            // get the file path
            let path = request
                .uri()
                .strip_prefix("fullscreen://localhost/")
                .unwrap();
            let path = percent_encoding::percent_decode(path.as_bytes())
                .decode_utf8_lossy()
                .to_string();
            let tmp_dir = app.path_resolver().app_data_dir().unwrap();
            let file_path = tmp_dir.join(path);
            if !file_path.exists() {
                return response.mimetype("text/plain").status(404).body(Vec::new());
            }

            // read our file
            let mut content = std::fs::File::open(&file_path)?;
            let mut buf = Vec::new();

            // default status code
            content.read_to_end(&mut buf)?;

            response.mimetype("image/png").status(200).body(buf)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
