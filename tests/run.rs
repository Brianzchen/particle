mod test_utils;

use particle::run;

#[tokio::test]
async fn calls_run_script() {
    let cur_dir = test_utils::setup();

    run(
        &cur_dir,
        "run",
        &Some(String::from("hello")),
        &None,
        &None,
    ).await;

    // TODO expect that hello was called

    test_utils::teardown();
}
