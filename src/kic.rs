// Jackson Coxson

use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize, Debug)]
pub struct KeyIndicator {
    #[serde(rename = "prosAreaId")]
    pub pros_area_id: u64, // Changed field to be snake_case to match JSON format
}

#[derive(Deserialize, Debug)]
pub struct KeyIndicatorsResponse {
    #[serde(rename = "areaKeyIndicators")]
    pub area_key_indicators: Vec<KeyIndicator>,
}

pub async fn extract_pros_area_ids(path: &str) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path).await?;
    let key_indicators: KeyIndicatorsResponse = serde_json::from_str(&data)?;

    let pros_area_ids = key_indicators
        .area_key_indicators
        .into_iter()
        .map(|indicator| indicator.pros_area_id)
        .collect();

    Ok(pros_area_ids)
}
