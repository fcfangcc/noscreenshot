use std::{fs, path::Path};

use screenshots::Screen;
use tauri::{Manager, Menu, Window, WindowBuilder};

pub struct ScreenshotTemp {
    id: String,
}

impl ScreenshotTemp {
    pub fn new(id: String) -> ScreenshotTemp {
        Self { id }
    }

    #[allow(dead_code)]
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

#[derive(serde::Deserialize, Debug)]
pub struct DisplayInfo {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    label: String,
    url: String,
}

#[tauri::command]
pub async fn show_screenshot(
    handle: tauri::AppHandle,
    info: DisplayInfo,
) -> Result<String, String> {
    println!("{:?}", info);
    let mut builder =
        WindowBuilder::new(&handle, info.label, tauri::WindowUrl::App(info.url.into()))
            .position(info.x, info.y)
            .inner_size(info.width, info.height)
            .skip_taskbar(true)
            .resizable(false)
            .focused(true)
            .always_on_top(true)
            .accept_first_mouse(true)
            .menu(Menu::new());

    #[cfg(target_os = "windows")]
    {
        builder = builder.fullscreen(true)
    }

    #[cfg(target_os = "macos")]
    {
        builder = builder
            .hidden_title(true)
            .decorations(false)
            .fullscreen(false)
    }

    let window = builder.build().map_err(|e| format!("{:?}", e))?;

    #[cfg(target_os = "macos")]
    window.hide_menubar();

    Ok(window.label().to_string())
}

pub trait WindowsExt {
    #[cfg(target_os = "macos")]
    fn hide_menubar(&self);
}

impl<R: tauri::Runtime> WindowsExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn hide_menubar(&self) {
        use cocoa::appkit::{NSMainMenuWindowLevel, NSWindow};
        use cocoa::base::id;

        unsafe {
            let ns_win = self.ns_window().unwrap() as id;
            ns_win.setLevel_(((NSMainMenuWindowLevel + 1) as u64).try_into().unwrap());
        }
    }
}
