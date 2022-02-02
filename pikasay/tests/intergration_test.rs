use std::error::Error;
use assert_cmd::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_defaults()  {
    Command::cargo_bin("pikasay")
        .expect("binary does not exist")
        .args(["-a","alt.txt"])
        .assert()
        .stdout(predicate::str::contains("Justice"))
        .success();

}