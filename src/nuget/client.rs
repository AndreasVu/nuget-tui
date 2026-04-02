#[derive(Debug, Default, Clone)]
pub struct NugetClient {
    client: reqwest::Client,
}

impl NugetClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Package>, anyhow::Error> {
        let url = format!("https://azuresearch-usnc.nuget.org/query?q={}", query);
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let search_response: SearchResponse = serde_json::from_str(&body)?;
        Ok(search_response.data)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub total_hits: u64,
    pub data: Vec<Package>,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    #[serde(rename = "@id")]
    pub url: String,
    #[serde(rename = "@type")]
    pub kind: String,
    pub registration: String,
    pub id: String,
    pub version: String,
    pub description: String,
    pub summary: String,
    pub title: String,
    pub icon_url: String,
    pub license_url: String,
    pub project_url: String,
    pub tags: Vec<String>,
    pub authors: Vec<String>,
    pub owners: Vec<String>,
    pub total_downloads: u64,
    pub verified: bool,
    pub package_types: Vec<PackageType>,
    pub versions: Vec<PackageVersion>,
    pub vulnerabilities: Vec<serde_json::Value>,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct PackageVersion {
    #[serde(rename = "@id")]
    pub id: String,
    pub version: String,
    pub downloads: u64,
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct PackageType {
    pub name: String,
}
