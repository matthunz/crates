use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CrateItemData {
    pub name: String,
    pub description: Option<String>,
    pub newest_version: String,
    pub downloads: u32,
    pub recent_downloads: u32,
    pub updated_at: String,
}

#[derive(Deserialize)]
struct Data {
    crates: Vec<CrateItemData>,
}

pub async fn get_crates(
    page: usize,
    per_page: usize,
    query: &str,
) -> reqwest::Result<Vec<CrateItemData>> {
    let uri = format!(
        "https://crates.io/api/v1/crates?page={page}&per_page={per_page}&sort=alpha&q={query}"
    );
    let data: Data = reqwest::get(uri).await?.json().await?;
    Ok(data.crates)
}

#[derive(Deserialize)]
pub struct CrateData {
    #[serde(rename = "crate")]
    pub krate: CrateItemData,
    pub versions: Vec<Version>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Version {
    pub features: HashMap<String, Vec<String>>,
    pub num: String,
}

pub async fn get_crate(name: &str) -> reqwest::Result<CrateData> {
    let uri = format!("https://crates.io/api/v1/crates/{name}");
    let data = reqwest::get(uri).await?.json().await?;
    Ok(data)
}
