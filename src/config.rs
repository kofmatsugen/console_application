use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub resource_path: std::path::PathBuf,
}
