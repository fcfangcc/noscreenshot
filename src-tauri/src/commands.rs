use std::{fs, path::Path};

use screenshots::Screen;

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
