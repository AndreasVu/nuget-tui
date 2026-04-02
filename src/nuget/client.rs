#[derive(Debug, Default)]
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
        let url = format!("https://api.nuget.org/v3/search?q={}", query);
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let packages: Vec<Package> = serde_json::from_str(&body)?;
        Ok(packages)
    }

    pub async fn search_by_ids(&self, ids: Vec<String>) -> Result<Vec<Package>, anyhow::Error> {
        let url = format!(
            "https://api.nuget.org/v3/registration5-semver1/{}/index.json",
            ids.join("/")
        );
        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let packages: Vec<Package> = serde_json::from_str(&body)?;
        Ok(packages)
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub id: String,
    pub version: String,
    pub versions: Vec<PackageVersion>,
    pub package_types: Vec<PackageType>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "string_or_vec")]
    pub authors: Vec<String>,
    #[serde(default)]
    pub icon_url: Option<String>,
    #[serde(default)]
    pub license_url: Option<String>,
    #[serde(default, deserialize_with = "string_or_vec")]
    pub owners: Vec<String>,
    #[serde(default)]
    pub project_url: Option<String>,
    #[serde(default)]
    pub registration: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default, deserialize_with = "string_or_vec")]
    pub tags: Vec<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub total_downloads: Option<u64>,
    #[serde(default)]
    pub verified: Option<bool>,
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

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    match StringOrVec::deserialize(deserializer)? {
        StringOrVec::String(s) => Ok(vec![s]),
        StringOrVec::Vec(v) => Ok(v),
    }
}
