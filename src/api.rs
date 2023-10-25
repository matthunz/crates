use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrateData {
    pub name: String,
    pub description: Option<String>,
    pub newest_version: String,
    pub downloads: u32,
    pub recent_downloads: u32,
    pub updated_at: String,
}

#[derive(Deserialize)]
struct Data {
    crates: Vec<CrateData>,
}

pub async fn crates(page: usize, per_page: usize) -> reqwest::Result<Vec<CrateData>> {
    let uri = format!("https://crates.io/api/v1/crates?page={page}&per_page={per_page}&sort=alpha");
    let data: Data = reqwest::get(uri).await?.json().await?;
    Ok(data.crates)
}
