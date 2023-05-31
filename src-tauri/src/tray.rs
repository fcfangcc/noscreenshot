use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use crate::window::open_helper_window;

pub fn create_tray(app: &tauri::App) -> tauri::Result<()> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("mainWindow".to_string(), "MainWindow");

    let mut tray_menu = SystemTrayMenu::new().add_item(hide);

    #[cfg(target_os = "macos")]
    {
        let access_help = CustomMenuItem::new("access-help".to_string(), "AccessHelp");
        tray_menu = tray_menu.add_item(access_help);
    }

    tray_menu = tray_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let handle = app.handle();
    let tray_id: String = "my-tray".to_string();

    #[allow(clippy::single_match)]
    SystemTray::new()
        .with_id(tray_id)
        .with_menu(tray_menu)
        .with_tooltip("NoScreenshot")
        .on_event(move |event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => handle.exit(0),
                "mainWindow" => {
                    let window = handle.get_window("main").unwrap();
                    if let Ok(true) = window.is_minimized() {
                        window.unminimize().unwrap()
                    }
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "access-help" => {
                    let handle = handle.clone();
                    open_helper_window(handle)
                }
                _ => {}
            },
            _ => {}
        })
        .build(app)
        .map(|_| ())
}
