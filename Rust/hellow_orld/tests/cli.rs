use assert_cmd::Command as Command_cargoBin;
use std::process::Command;

#[test]
fn atest_which_passes_when_program_fails() {
    let mut cmd = Command_cargoBin::cargo_bin("afailedtest").unwrap();
    cmd.assert().failure();
}

#[test]
fn a_test_where_a_program_aborts() {
    let mut cmd = Command_cargoBin::cargo_bin("abort").unwrap();
    cmd.assert().failure();
}

#[test]
fn a_test_without_any_assert() {
    println!("testing something which doen't assert shit");
}

#[test]
fn runs() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn runs_build() {
    let mut cmd = Command::new("./target/debug/hellow_orld");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn runs_check_in_bin() {
    let mut cmd = Command_cargoBin::cargo_bin("check").unwrap();
    cmd.assert().success();
}

#[test]
fn runs_with_new_cmd() {
    let mut cmd = Command_cargoBin::cargo_bin("hellow_orld").unwrap();
    cmd.assert().success();
}
