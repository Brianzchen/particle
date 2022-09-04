use std::fs;
use serde_json::from_str;

use crate::constants;

pub fn check(_config: &constants::ParticleConfig, root_path: &String) {
  println!("install deps I guess");

  // Pull lock file data
  let mut lock_file = root_path.clone();
  lock_file.push_str("/particle.lock.json");
  let lock_file = fs::read_to_string(lock_file);
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
