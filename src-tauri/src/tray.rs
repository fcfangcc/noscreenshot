use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub fn create_tray(app: &tauri::App) -> tauri::Result<()> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("toggle".to_string(), "Hide");

    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let handle = app.handle();
    let tray_id: String = "my-tray".to_string();

    SystemTray::new()
        .with_id(&tray_id)
        .with_menu(tray_menu)
        .with_tooltip("NoScreenshot")
        .on_event(move |event| {
            let tray_handle = handle.tray_handle_by_id(&tray_id).unwrap();
            match event {
                SystemTrayEvent::LeftClick { .. } => {
                    let window = handle.get_window("main").unwrap();
                    if let Ok(true) = window.is_minimized() {
                        window.unminimize().unwrap()
                    }
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    let item_handle = tray_handle.get_item(&id);
                    match id.as_str() {
                        "quit" => handle.exit(0),
                        "toggle" => {
                            let window = handle.get_window("main").unwrap();
                            let new_title = if window.is_visible().unwrap() {
                                window.hide().unwrap();
                                "Show"
                            } else {
                                window.show().unwrap();
                                "Hide"
                            };
                            item_handle.set_title(new_title).unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .build(app)
        .map(|_| ())
}
