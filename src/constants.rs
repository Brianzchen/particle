use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CONFIG_FILE_NAME: &str = "particle.config.json";

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SyncDependencies {
    All(bool),
    Subset(Vec<String>)
}

fn sync_dependencies_default() -> SyncDependencies { SyncDependencies::All(true) }
fn check_installs_default() -> bool { false }
fn options_default() -> ParticleConfigOptions {
    ParticleConfigOptions {
        check_installs: check_installs_default(),
        sync_dependencies: sync_dependencies_default(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleConfigOptions {
    #[serde(default = "check_installs_default")]
    pub check_installs: bool,

    #[serde(default = "sync_dependencies_default")]
    pub sync_dependencies: SyncDependencies
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleConfig {
    pub workspaces: Vec<String>,

    pub scripts: Option<HashMap<String, String>>,

    #[serde(default = "options_default")]
    pub options: ParticleConfigOptions,
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
