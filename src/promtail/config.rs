use anyhow::Result;
use tokio::fs;
use std::env;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PromtailConfig {
    pub positions: Positions,
}

#[derive(Debug, Deserialize)]
pub struct Positions {
    pub filename: String,
}

fn get_promtail_config_path() -> Result<String> {
    env::var("PROMTAIL_CONFIG_PATH").map_err(|_| anyhow::anyhow!("PROMTAIL_CONFIG_PATH environment variable is not set"))
}

pub async fn read_promtail_config() -> Result<PromtailConfig> {
    let path = get_promtail_config_path()?;
    let config_content = fs::read_to_string(&path).await?;
    let config: PromtailConfig = serde_yaml::from_str(&config_content)?;
    Ok(config)
}
