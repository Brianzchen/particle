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

pub type Scripts = HashMap<String, String>;
pub type Dependencies = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleConfig {
    pub workspaces: Vec<String>,

    pub scripts: Option<Scripts>,

    #[serde(default = "options_default")]
    pub options: ParticleConfigOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleDependencyLock {

}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PkgJson {
    /// The package name
    pub name: Option<String>,

    /// scripts key on package.json
    pub scripts: Option<Scripts>,

    pub dependencies: Option<Dependencies>,

    pub dev_dependencies: Option<Dependencies>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Workspace {
    /// The path to the package
    pub path: String,

    /// Fields in workspace's package.json
    pub package: PkgJson,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct PackageRegistryVersionDist {
    shasum: String,

    /// Link to the actual package code
    tarball: String,

    integrity: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageRegistryVersion {
    name: String,
    version: String,
    description: String,
    main: String,
    dependencies: Option<Dependencies>,
    peer_dependencies: Option<Dependencies>,
    dev_dependencies: Option<Dependencies>,
    dist: PackageRegistryVersionDist,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageRegistry {
    pub name: String,

    pub versions: HashMap<String, PackageRegistryVersion>,
}
