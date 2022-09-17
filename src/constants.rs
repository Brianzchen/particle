use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

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

/// Sample struct
/// react: {
///     /// locked exact version
///     [version: string]: {
///         /// tarball#shasum
///         resolved "https://registry.yarnpkg.com/@ampproject/remapping/-/remapping-2.1.2.tgz#4edca94973ded9630d20101cd8559cedb8d8bd34"
///         integrity sha512-hoyByceqwKirw7w3Z7gnIIZC3Wx3J484Y3L/cMpXFbr7d9ZQj2mODrirNzcJa+SM3UlpWXYvKV4RlRpFXlWgXg==
///         dependencies: {
///             "@jridgewell/trace-mapping" "^0.3.0"
///         }
///         workspaces: [['root', '1.0.0']], ['@particle/foo', '^1.0.0], ...]
///     }
/// }
#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleLockDependencyVersion {
    pub resolved: String,
    pub integrity: String,
    pub dependencies: Option<Dependencies>,
    pub workspaces: Vec<(String, String)>,
}

impl ParticleLockDependencyVersion {
    pub fn new(
        package_registry_data: &PackageRegistryVersion,
    ) -> Self {
        ParticleLockDependencyVersion {
            resolved: format!("{}#{}", package_registry_data.dist.tarball, package_registry_data.dist.shasum),
            integrity: package_registry_data.dist.integrity.clone(),
            dependencies: package_registry_data.dependencies.clone(),
            workspaces: vec![],
        }
    }

    pub fn add_workspace(&mut self, workspace: &String, version: &String) {
        self.workspaces.push((workspace.clone(), version.clone()));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleLock {
    pub dependencies: HashMap<String, HashMap<String, ParticleLockDependencyVersion>>,
}

impl ParticleLock {
    pub fn new() -> Self {
        ParticleLock {
            dependencies: HashMap::new(),
        }
    }
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
    /// Path to workspace directory
    pub workspace_path: String,

    /// Path to the package.json
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
