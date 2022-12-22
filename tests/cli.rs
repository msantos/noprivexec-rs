use assert_cmd::Command;

#[test]
fn setuid() {
    let mut cmd = Command::cargo_bin("noprivexec").unwrap();
    let fail = cmd.args(["sudo", "true"]).assert();
    fail.failure();
}

#[test]
fn nosetuid() {
    let mut cmd = Command::cargo_bin("noprivexec").unwrap();
    let ok = cmd.args(["true"]).assert();
    ok.success();
}
