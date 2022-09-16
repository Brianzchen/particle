mod test_utils;

use particle::run;

#[tokio::test]
async fn loops_through_every_workspace_to_call_script() {
    let cur_dir = test_utils::setup();

    run(
        &cur_dir,
        "workspace",
        &Some(String::from("hello")),
        &None,
        &None,
    ).await;

    // TODO expect all fake package hellos were called

    test_utils::teardown();
}
