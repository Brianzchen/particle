use std::{fs, collections::HashMap};
use serde_json::from_str;

use crate::constants::{ParticleConfig, ParticleDependencyLock, Dependencies, PkgJson, SyncDependencies};
use crate::utils::{get_workspaces_data, highlight};

fn extract_dependencies(dependencies: &mut HashMap<String, Vec<String>>, deps: Option<Dependencies>) {
    if let Some(map) = deps {
        for (key, value) in map.into_iter() {
            let dep = dependencies.entry(key).or_insert(vec![]);
            dep.push(value);
        }
    }
}

pub fn main(config: &ParticleConfig, root_path: &String) {
    // Pull lock file data
    let lock_file = fs::read_to_string(format!("{}/particle.lock.json", root_path));
    let lock_file = match lock_file {
        Ok(content) => {
            let config: ParticleDependencyLock = from_str(&content)
                .expect("lock file is malformed");
            config
        },
        Err(_) => {
            println!("No lock file found");
            ParticleDependencyLock {}
        }
    };
    println!("the lock contents are {:?}", lock_file);

    let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
    let workspaces = get_workspaces_data(&config, &root_path);

    if let Ok(root_pkg_json) = fs::read_to_string(format!("{}/package.json", root_path)) {
        if let Ok(root_pkg_json) = from_str::<PkgJson>(&root_pkg_json) {
            extract_dependencies(&mut dependencies, root_pkg_json.dependencies);
            extract_dependencies(&mut dependencies, root_pkg_json.dev_dependencies);
        }
    }
    println!("{:?}", dependencies);

    for workspace in workspaces {
        let deps = workspace.package.dependencies;
        let dev_deps = workspace.package.dev_dependencies;

        extract_dependencies(&mut dependencies, deps);
        extract_dependencies(&mut dependencies, dev_deps);
    }

    for (_, dep_versions) in &mut dependencies {
        dep_versions.sort_unstable();
        dep_versions.dedup();
    }

    match &config.options.sync_dependencies {
        SyncDependencies::All(syn_deps) => {
            if *syn_deps {
                let all_deps_single_version = &dependencies
                    .iter()
                    .find(|deps| {
                        let (_key, version_list) = deps;
                        version_list.len() > 1
                    });
                match all_deps_single_version {
                    Some(dep) => {
                        let (key, _) = dep;
                        println!("{} is enabled, all dependencies across the project must use the same version.",
                            highlight(&String::from("sync_dependencies")));
                        panic!("Found {} with mismatched dependency versions", highlight(key));
                    },
                    None => {},
                }
            }
        },
        SyncDependencies::Subset(_list) => {

        },
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
