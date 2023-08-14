use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub remote: String,
    pub stable_version: Option<Version>,
    pub latest_version: Option<Version>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub commit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct D17Manifest {
    pub plugin: D17ManifestPlugin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct D17ManifestPlugin {
    pub repository: String,
    pub commit: String,
    pub owners: Vec<String>,
    pub project_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginManifest {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "AssemblyVersion")]
    pub assembly_version: String,

    #[serde(rename = "IconUrl")]
    pub icon_url: Option<String>,
}
