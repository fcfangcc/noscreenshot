use std::{
    borrow::Cow,
    fs::OpenOptions,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use tauri::State;

fn default_screenshot() -> String {
    "CommandOrControl+Shift+Z".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shortcut {
    #[serde(default = "default_screenshot")]
    screenshot: String,
}

impl Default for Shortcut {
    fn default() -> Self {
        serde_json::from_str("{}").unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigData {
    #[serde(default)]
    shortcut: Shortcut,
}

pub struct Config {
    data: Arc<Mutex<ConfigData>>,
    path: String,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let data: ConfigData = match std::path::Path::new(path).exists() {
            true => {
                let f = std::fs::File::open(path).expect("read config failed.");
                let obj = serde_json::from_reader(f).expect("serde config failed.");
                info!("Load config from {} successfully", path);
                obj
            }
            false => {
                info!("Load config from {} failed. With default config", path);
                serde_json::from_str("{}").unwrap()
            }
        };

        Self {
            data: Arc::new(Mutex::new(data)),
            path: path.to_string(),
        }
    }

    fn set_shortcut_screenshot(&self, key: &str) {
        let mut data = self.data.lock().unwrap();
        data.shortcut.screenshot = key.to_string();
        drop(data);
        self.persistence();
    }

    fn persistence(&self) {
        let data = self.data.lock().unwrap();
        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(&self.path)
            .expect("open config failed.");
        serde_json::to_writer(f, &*data).unwrap();
        info!("Persistence config {} successfully.", &self.path);
    }

    pub fn data(&self) -> ConfigData {
        let data = self.data.lock().unwrap();
        (*data).clone()
    }
}

#[tauri::command]
pub fn get_config(config: State<'_, Config>) -> ConfigData {
    config.data()
}

#[tauri::command]
pub fn set_shortcut_screenshot(config: State<'_, Config>, key: Cow<'_, &str>) -> ConfigData {
    config.set_shortcut_screenshot(&key);
    config.data()
}
