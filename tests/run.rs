mod test_utils;

use particle::run;

#[tokio::test]
async fn calls_run_script() {
    let cur_dir = test_utils::setup(None);

    run(
        &cur_dir,
        "run",
        &Some(String::from("hello")),
        &None,
        &None,
    ).await.unwrap();

    // TODO expect that hello was called

    test_utils::teardown();
}
