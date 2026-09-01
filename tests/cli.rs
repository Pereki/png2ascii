use assert_cmd::Command;

#[test]
fn test_no_arguments_given() {
    let mut cmd = Command::cargo_bin("png2ascii").unwrap();
    cmd.assert().stdout(b"Please provide a path to the image file using the --path argument or run in interactive mode using the --interactive flag.\n" as &[u8]).success();
}

#[test]
fn test_no_valid_path() {
    let mut cmd = Command::cargo_bin("png2ascii").unwrap();

    cmd.arg("--path")
        .arg("./does/not/exist.png")
        .assert()
        .failure();
}
