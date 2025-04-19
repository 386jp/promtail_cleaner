use anyhow::Result;
use tokio::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawPromtailPositionData {
    pub positions: std::collections::HashMap<String, String>,
}
#[derive(Debug, Deserialize)]
pub struct PromtailPositionData {
    pub positions: std::collections::HashMap<String, u64>,
}

pub async fn read_positions_file(file_path: &str) -> Result<PromtailPositionData> {
    let content = fs::read_to_string(file_path).await?;
    let raw_data: RawPromtailPositionData = serde_yaml::from_str(&content)?;
    // Hashmap の value を u64 に変換
    let positions = raw_data
        .positions
        .into_iter()
        .map(|(k, v)| {
            let value = v.parse::<u64>().unwrap_or_default();
            (k, value)
        })
        .collect();
    let data = PromtailPositionData { positions };
    Ok(data)
}
