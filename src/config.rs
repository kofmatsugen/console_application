use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub resource_path: std::path::PathBuf,
    pub convert_fight_animations: Vec<std::path::PathBuf>,
    pub convert_splash_animations: Vec<std::path::PathBuf>,
}
