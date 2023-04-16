use cargo_test_support::{compare::assert_ui, curr_dir, paths, prelude::*, Project};

#[cargo_test]
fn case() {
    let cwd = &paths::root().join("foo");
    std::fs::create_dir(cwd).unwrap();

    snapbox::cmd::Command::cargo_ui()
        .arg("init")
        .arg_line("--verbose --lib foo") // Note the `--verbose`
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches_path(curr_dir!().join("stdout-verbose.log"))
        .stderr_matches_path(curr_dir!().join("stderr-verbose.log"));

    // Check without `--verbose`

    std::fs::remove_dir_all(cwd).unwrap();
    std::fs::create_dir(cwd).unwrap();

    snapbox::cmd::Command::cargo_ui()
        .arg("init")
        .arg_line("--lib foo") // Note the lack of `--verbose`
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));
}
