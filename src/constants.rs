use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CONFIG_FILE_NAME: &str = "particle.config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleConfig {
    pub workspaces: Vec<String>,
    pub scripts: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleDependencyLock {

}

pub type Scripts = Option<HashMap<String, String>>;

#[derive(Deserialize, Debug)]
pub struct PkgJson {
  pub name: String,
  pub scripts: Scripts,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Workspace {
  /// The path to the package
  pub path: String,

  /// The package name
  pub name: String,

  /// scripts key on package.json
  pub scripts: Scripts,
}
