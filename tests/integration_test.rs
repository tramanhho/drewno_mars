use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_inputs() -> Result<(), Box<dyn std::error::Error>> {
    // let paths = fs::read_dir("./").unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
    let mut test = Command::cargo_bin("drewno_mars")?;

    test.args(&["./tests/input/test1.dm", "-u"]);
    test.assert()
        .success();

    Ok(())
}