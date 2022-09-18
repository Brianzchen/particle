mod test_utils;

use particle::run;

#[tokio::test]
async fn fails_if_no_config_file() {
    let cur_dir = test_utils::setup(Some("/project-no-config"));

    let res = run(
        &cur_dir,
        "run",
        &Some(String::from("hello")),
        &None,
        &None,
    ).await.unwrap_err();

    assert_eq!(
        res.to_string().contains("`particle.config.json` not found. You should add one to the root of your project to get started"),
        true,
    );

    test_utils::teardown();
}
