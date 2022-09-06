use run_script::{run_script};

use crate::constants;

pub fn main(
  config: &constants::ParticleConfig,
  _root_path: &String,
  script: &String,
) {
  let scripts = &config.scripts;

  if let Some(s) = scripts {
    let script_value = s.get(script);
    if let Some(run) = script_value {
      let (_, output, error) = run_script!(run).unwrap();
      if error.len() == 0 {
        if output.len() > 0 {
          print!("{}", output);
        }
        println!("Done ✨");
      } else {
        println!("{}", error);
      }
    } else {
      println!("Script {:?} does not exist!", script_value);
    }
  }
}
