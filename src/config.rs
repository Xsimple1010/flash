use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FlashConfig {
    pub dir: Vec<String>,
    pub observers: Vec<Observer>,
}


#[derive(Debug, serde::Deserialize, Clone)]
pub struct Observer {
    pub name: String,
    pub executable: String,
    pub deps: Vec<String>,
    pub url: String,
}


impl FlashConfig {
    pub fn new<'a>(origin: PathBuf) -> Result<FlashConfig, &'a str> {
        let dir = origin.read_dir().expect("Failed to read directory");

        for entry in dir {
            let path = entry.expect("Failed to read file entry").path();

            if path.is_file() && path.file_name().map_or(false, |name| name == "flash.json") {
                let content = fs::read_to_string(&path).expect("Could not read flash.json");

                if let Ok(config) = serde_json::from_str::<FlashConfig>(&content) {
                    return Ok(config);
                }
            }
        }

        Err("flash.json not found")
    }

}