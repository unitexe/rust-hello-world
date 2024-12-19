use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn hello_message_printed_to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-hello-world")?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello rustacean !"));

    Ok(())
}
