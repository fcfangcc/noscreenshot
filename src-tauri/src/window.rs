use tauri::{AppHandle, Manager};

const HELPER_NAME: &str = "AccessHelp";

pub fn open_helper_window(handle: AppHandle) {
    std::thread::spawn(move || {
        if let Some(window) = handle.get_window(HELPER_NAME) {
            window.set_focus().unwrap();
        } else {
            let _ = tauri::WindowBuilder::new(
                &handle,
                HELPER_NAME,
                tauri::WindowUrl::App("sc-help".into()),
            )
            .title("AccessHelp")
            .build()
            .unwrap();
        }
    });
}
