use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub screenshot_style: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            screenshot_style: "Gradient".to_string(),
        }
    }
}

impl AppSettings {
    fn settings_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "xiaoxu", "AudioCalculator")
            .map(|dirs| dirs.config_dir().join("settings.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::settings_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(settings) = serde_json::from_str(&content) {
                        return settings;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::settings_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, json);
            }
        }
    }
}
