use std::env;
use std::fs;
use copy_dir::copy_dir;

fn get_fixture_dir() -> String {
    let cur_dir = env::current_dir().unwrap();
    let cur_dir = cur_dir.to_str().unwrap();
    let mut cur_dir = String::from(cur_dir);
    cur_dir.push_str("/_fixtures_");

    cur_dir
}

fn get_fixture_project() -> String {
    let mut cur_dir = get_fixture_dir();
    cur_dir.push_str("/project-integration-tests");

    cur_dir
}

fn create_fixture_project() {
    let fixture_dir = get_fixture_dir();
    let mut cur_dir = fixture_dir.clone();
    cur_dir.push_str("/project-a");

    let mut fixture_project_dir = fixture_dir.clone();
    fixture_project_dir.push_str("/project-integration-tests");

    copy_dir(cur_dir, fixture_project_dir)
        .expect("Copy failed");
}

fn delete_integration_tests_dir() {
    let mut fixture_dir = get_fixture_dir();
    fixture_dir.push_str("/project-integration-tests");

    fs::remove_dir_all(fixture_dir).unwrap_or_else(|_| {

    });
}

pub fn setup() -> String {
    delete_integration_tests_dir();
    create_fixture_project();
    get_fixture_project()
}

pub fn teardown() {
    delete_integration_tests_dir();
}
