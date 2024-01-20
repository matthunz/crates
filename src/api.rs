use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Version {
    pub num: String
}

#[derive(Debug, Deserialize)]
pub struct Crate {
    pub name: String,
    pub description: String,
    pub max_stable_version: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CrateData {
    #[serde(rename = "crate")]
    pub krate: Crate,
    pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct SearchData {
    pub crates: Vec<Crate>,
}

pub async fn get_crate(name: &str) -> CrateData {
    reqwest::get(format!("https://crates.io/api/v1/crates/{name}"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn search(query: &str) -> SearchData {
    reqwest::get(format!(
        " https://crates.io/api/v1/crates?page=1&per_page=10&q={query}"
    ))
    .await
    .unwrap()
    .json()
    .await
    .unwrap()
}

pub async fn get_readme(name: &str, latest: &str) -> String {
    reqwest::get(format!(
        "https://crates.io/api/v1/crates/{name}/{latest}/readme"
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap()
}
