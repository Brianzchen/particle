use std::env;

pub fn get_fixture_dir() -> String {
    let cur_dir = env::current_dir().unwrap();
    let cur_dir = cur_dir.to_str().unwrap();
    let mut cur_dir = String::from(cur_dir);
    cur_dir.push_str("/_fixtures_/project-a");

    cur_dir
}
