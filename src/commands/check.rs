mod get_highest_compatible_version;

use std::time::Instant;
use std::{fs, collections::HashMap};
use reqwest::{Client, Error};
use serde_json::from_str;

use crate::constants::{ParticleConfig, ParticleLock, Dependencies, PkgJson, SyncDependencies, PackageRegistry, ParticleLockDependencyVersion};
use crate::utils::{get_workspaces_data, highlight};

use get_highest_compatible_version::main as get_highest_compatible_version;

fn extract_dependencies(dependencies: &mut HashMap<String, Vec<String>>, deps: Option<Dependencies>) {
    if let Some(map) = deps {
        for (key, value) in map.into_iter() {
            let dep = dependencies.entry(key).or_insert(vec![]);
            dep.push(value);
        }
    }
}

pub async fn main(config: &ParticleConfig, root_path: &String) {
    // Pull lock file data
    let lock_file = fs::read_to_string(format!("{}/particle.lock.json", root_path));
    let _lock_file = match lock_file {
        Ok(content) => {
            let config: ParticleLock = from_str(&content)
                .expect("lock file is malformed");
            config
        },
        Err(_) => {
            println!("No lock file found");
            ParticleLock::new()
        }
    };

    let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
    let workspaces = get_workspaces_data(&config, &root_path);

    if let Ok(root_pkg_json) = fs::read_to_string(format!("{}/package.json", root_path)) {
        if let Ok(root_pkg_json) = from_str::<PkgJson>(&root_pkg_json) {
            extract_dependencies(&mut dependencies, root_pkg_json.dependencies);
            extract_dependencies(&mut dependencies, root_pkg_json.dev_dependencies);
        }
    }

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
                        panic!("Found dependency {} with mismatched dependency versions", highlight(key));
                    },
                    None => {},
                }
            }
        },
        SyncDependencies::Subset(list) => {
            let unlisted_deps_more_than_one_version = &dependencies
                .iter()
                .find(|deps| {
                    let (key, version_list) = deps;
                    !list.contains(key) && version_list.len() > 1
                });
            if let Some((key, _value)) = unlisted_deps_more_than_one_version {
                println!("An unlisted dependency {} is not synced across your repo.", highlight(key));
                println!("If this is intentional you should add it to {} in your {}",
                    highlight(&String::from("sync_dependencies")),
                    highlight(&String::from("particle.config.json")));
                panic!("Otherwise ensure it's sync across the project to continue");
            }
        },
    }

    println!("{:?}", dependencies);

    // TODO: read npmrc for different registry per scope

    let now = Instant::now();
    let client = Client::new();

    let mut packages_registry = vec![];

    for (dep, _version) in &dependencies {
        // TODO: Need to make the calls async
        let url = format!("https://registry.npmjs.org/{}", dep);
        let res: Result<PackageRegistry, Error> = client.get(url)
            .send()
            .await
            .expect(format!("Unable to query registry for package {}", dep).as_str())
            .json()
            .await;

        packages_registry.push(res);
    }

    let mut new_lock = ParticleLock::new();

    for package_registry in &packages_registry {
        match package_registry {
            Ok(registry_payload) => {
                let dependency_version_map = new_lock.dependencies
                    .entry(registry_payload.name.clone())
                    .or_insert(HashMap::new());

                let versions = dependencies.get(&registry_payload.name);
                let registry_available_versions = &registry_payload.versions;

                if let Some(versions) = versions {
                    for version in versions {
                        let available_versions: Vec<&String> = registry_available_versions.keys().collect();
                        let lock_version = get_highest_compatible_version(available_versions, version);

                        let version_data = registry_available_versions.get(&lock_version)
                            .expect("Cannot get package registry data for version queried by version");

                        let dependency_version_data = dependency_version_map.entry(lock_version).or_insert(ParticleLockDependencyVersion::new(
                            version_data,
                        ));

                        dependency_version_data.add_workspace(&String::from("@foo/bar"), &version);

                        // Go through every version and
                        // get the most highest valid version payload from the
                        // package registry
                        // and save it as a particle lock
                    }
                }
            },
            Err(e) => {
                panic!("{e}");
            },
        }
    }

    println!("{:?}", new_lock);

    println!("{}", now.elapsed().as_millis());

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
