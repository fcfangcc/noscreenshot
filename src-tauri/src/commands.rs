use std::{fs, path::Path};

use screenshots::Screen;
use tauri::{Manager, Window};

pub struct ScreenshotTemp {
    id: String,
}

impl ScreenshotTemp {
    pub fn new(id: String) -> ScreenshotTemp {
        Self { id }
    }

    pub fn from_path(path: &str) -> Result<Self, String> {
        match path.starts_with("screenshots") {
            true => {
                let paths: Vec<_> = path.split('/').collect();
                if paths.len() < 2 {
                    Err("file path error.".to_string())
                } else {
                    Ok(Self {
                        id: paths[1].to_string(),
                    })
                }
            }
            false => Err("file path error.".to_string()),
        }
    }

    pub fn dirname(&self) -> String {
        format!("screenshots/{}", self.id)
    }
}

pub fn screenshots_fullscreens<P: AsRef<Path>>(tmp_dir: P, id: &str) -> Vec<String> {
    let screens = Screen::all().unwrap();

    screens
        .iter()
        .map(|screen| {
            let image = screen.capture().unwrap();
            let buffer = image.buffer();
            let temp = ScreenshotTemp::new(id.to_string());
            let dir_path = tmp_dir.as_ref().join(temp.dirname());
            if !Path::new(&dir_path).exists() {
                fs::create_dir_all(&dir_path).unwrap()
            }
            let filename = format!("{}.png", screen.display_info.id);

            fs::write(dir_path.join(&filename), buffer).unwrap();
            format!("{}/{}", temp.dirname(), filename)
        })
        .collect()
}

#[tauri::command]
pub fn screenshot(window: Window, id: String) -> (String, Vec<String>) {
    let tmp_dir = window.app_handle().path_resolver().app_data_dir().unwrap();

    (
        "fullscreen".to_string(),
        screenshots_fullscreens(tmp_dir, &id),
    )
}

#[tauri::command]
pub fn clear_temp(window: Window, id: String) -> Result<(), String> {
    let cache_dir = window.app_handle().path_resolver().app_data_dir().unwrap();
    let temp_dir = cache_dir.join(ScreenshotTemp::new(id).dirname());

    if temp_dir.exists() {
        info!("delete tmp file. {}", temp_dir.display());
        fs::remove_dir_all(temp_dir).map_err(|e| format!("{}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub fn logger(level: &str, message: &str) {
    match level {
        "info" => info!(target: "webapp", "{}", message),
        "error" => error!(target: "webapp", "{}", message),
        "warning" | "warn" => warn!(target: "webapp", "{}", message),
        _ => error!(target: "webapp", "level:{} message:{}", level, message),
    }
}
