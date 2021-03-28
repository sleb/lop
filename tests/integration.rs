use assert_cmd::prelude::*;
use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;


struct LopTest {
    name: String,
    args: Vec<String>,
    input: String,
    output: String,
}

fn tests() -> Vec<LopTest> {
    vec![LopTest {
        name: String::from("1"),
        args: vec![String::from("-d:"), String::from("-f1,3-")],
        input: String::from("a:b:c\n"),
        output: String::from("a:c\n"),
    }]
}

#[test]
fn test_stdin() -> Result<(), Box<dyn Error>> {
    for test in &tests() {
        let want = predicate::str::is_match(&test.output)?;
        Command::cargo_bin("lop")?
            .args(&test.args)
            .write_stdin(test.input.as_bytes())
            .assert()
            .success()
            .stdout(want);
    }

    Ok(())
}
