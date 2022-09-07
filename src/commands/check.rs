use std::{fs, collections::HashMap};
use serde_json::from_str;

use crate::constants;
use crate::utils::{get_workspaces_data};

pub fn main(config: &constants::ParticleConfig, root_path: &String) {
    // Pull lock file data
    let lock_file = fs::read_to_string(format!("{}/particle.lock.json", root_path));
    let lock_file = match lock_file {
        Ok(content) => {
            let config: constants::ParticleDependencyLock = from_str(&content)
                .expect("lock file is malformed");
            config
        },
        Err(_) => {
            println!("No lock file found");
            constants::ParticleDependencyLock {}
        }
    };
    println!("the lock contents are {:?}", lock_file);

    let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
    let workspaces = get_workspaces_data(&config, &root_path);

    for workspace in workspaces {
        let deps = workspace.package.dependencies;
        let dev_deps = workspace.package.dev_dependencies;

        if let Some(map) = deps {
            for (key, value) in map.into_iter() {
                let dep = dependencies.entry(key).or_insert(vec![]);
                dep.push(value);
            }
        }

        if let Some(map) = dev_deps {
            for (key, value) in map.into_iter() {
                let dep = dependencies.entry(key).or_insert(vec![]);
                dep.push(value);
            }
        }
    }

    for (_, dep_versions) in &mut dependencies {
        dep_versions.sort_unstable();
        dep_versions.dedup();
    }

    println!("{:?}", dependencies);

    // look at the dependencies/peerDependencies of each
    // create a list of what needs to be installed
    // compare with lock file
    // update lock file
    // and query .npmrc file to figure out where to look for packages
    // then install the remaining uninstall packages
    // what if we lazily installed dependencies
    // Then particle would also need to be a runtime but we want to be un opinionated
    // cleanup remaining packages
    // only write new lockfile now after successful install
}
