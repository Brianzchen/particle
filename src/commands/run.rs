use crate::constants;
use crate::utils::{run_script_in_optional_scripts};

pub fn main(
    config: &constants::ParticleConfig,
    root_path: &String,
    script: &String,
) {
    let scripts = &config.scripts;

    run_script_in_optional_scripts(
        root_path,
        root_path,
        scripts,
        script,
    );
}
