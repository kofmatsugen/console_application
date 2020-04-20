use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub resource_path: std::path::PathBuf,
    pub convert_animation_files: Vec<std::path::PathBuf>,
}
