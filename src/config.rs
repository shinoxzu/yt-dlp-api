use config::Config;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    pub server_url: String,
    pub max_file_size: String,
    pub yt_dlp_path: String,
}

pub fn load_config(path: &str) -> anyhow::Result<ApiConfig> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::with_prefix("YTDLP_API"))
        .build()?;

    Ok(config.try_deserialize::<ApiConfig>()?)
}
