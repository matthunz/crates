use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Clone, Deserialize, Serialize)]
pub struct PublishedBy {
    pub  avatar: String,
    pub  name: String,
    pub  login: String,
    pub  url: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Version {
    pub features: HashMap<String, Vec<String>>,
    pub num: String,
    pub readme_path: String,
    pub published_by: PublishedBy
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Category {
    pub category: String,
}

#[derive(Deserialize)]
pub struct CrateData {
    #[serde(rename = "crate")]
    pub krate: CrateItemData,
    pub versions: Vec<Version>,
    pub categories: Vec<Category>,
}

pub async fn get_crate(name: &str) -> reqwest::Result<CrateData> {
    let uri = format!("https://crates.io/api/v1/crates/{name}");
    let data = reqwest::get(uri).await?.json().await?;
    Ok(data)
}
