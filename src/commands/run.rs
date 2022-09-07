use crate::constants;
use crate::utils::{run_script_in_optional_scripts};

pub fn main(
    config: &constants::ParticleConfig,
    _root_path: &String,
    script: &String,
) {
    let scripts = &config.scripts;

    run_script_in_optional_scripts(scripts, script);
}
