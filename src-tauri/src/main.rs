// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate tracing;

mod commands;
pub mod tracing_config;
mod tray;

use std::{io::Read, process};

use tauri::http::{Request, Response};
use tauri::{http::ResponseBuilder, Manager};
use tauri::{
    AppHandle, CustomMenuItem, Menu, RunEvent, Submenu, WindowBuilder, WindowEvent, WindowMenuEvent,
};

fn menu() -> Menu {
    let quit = CustomMenuItem::new("appdir".to_string(), "AppDir");
    let submenu = Submenu::new("Tools", Menu::new().add_item(quit));
    Menu::new().add_submenu(submenu)
}

fn bind_menu_event(event: WindowMenuEvent) {
    #[allow(clippy::single_match)]
    match event.menu_item_id() {
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
    }
}

fn bind_fullscreen_protocol(
    app: &AppHandle,
    request: &Request,
) -> Result<Response, std::boxed::Box<(dyn std::error::Error + 'static)>> {
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
}

fn main() {
    let mut builder = tauri::Builder::default()
        .on_menu_event(bind_menu_event)
        .setup(|app| {
            tray::create_tray(app)?;
            let log_dir = app.path_resolver().app_log_dir().unwrap();

            tracing_config::init(&log_dir.display().to_string());

            let mut window_builder = WindowBuilder::new(
                app,
                "main".to_string(),
                tauri::WindowUrl::App("index.html".into()),
            )
            .title("NoScreenshot")
            .resizable(true)
            .inner_size(800.0, 600.0)
            .menu(menu());

            #[cfg(target_os = "windows")]
            {
                window_builder = window_builder.transparent(true).decorations(true);
            }

            let _ = window_builder.build()?;

            Ok(())
        });

    builder = builder.register_uri_scheme_protocol("fullscreen", bind_fullscreen_protocol);
    builder = builder.invoke_handler(tauri::generate_handler![
        commands::screenshot,
        commands::clear_temp,
        commands::logger
    ]);

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(tauri::Menu::os_default("NoScreenshot"));
    }

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Regular);

    app.run(move |app_handle, e| match e {
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            if label == "main" {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window(&label).unwrap();
                window.hide().unwrap();
                api.prevent_close();
            }
        }
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => (),
    })
}
