mod test_utils;

use particle::run;

#[tokio::test]
async fn can_call_script() {
    let cur_dir = &test_utils::get_fixture_dir();

    run(
        cur_dir,
        "run",
        &Some(String::from("hello")),
        &None,
        &None,
    ).await;
}
