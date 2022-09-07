use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CONFIG_FILE_NAME: &str = "particle.config.json";

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SyncDependencies {
    All(bool),
    Subset(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleConfigOptions {
    pub check_installs: Option<bool>,
    pub sync_dependencies: Option<SyncDependencies>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleConfig {
    pub workspaces: Vec<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub options: Option<ParticleConfigOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleDependencyLock {

}

pub type Scripts = Option<HashMap<String, String>>;
pub type Dependencies = Option<HashMap<String, String>>;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PkgJson {
    /// The package name
    pub name: String,

    /// scripts key on package.json
    pub scripts: Scripts,

    pub dependencies: Dependencies,

    pub dev_dependencies: Dependencies,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Workspace {
    /// The path to the package
    pub path: String,

    /// Fields in workspace's package.json
    pub package: PkgJson,
}
