use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CONFIG_FILE_NAME: &str = "particle.config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleConfig {
    workspaces: Vec<String>,
    scripts: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleDependencyLock {

}
