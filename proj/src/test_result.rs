extern crate colored;
use super::assert::*;
use colored::*;

const PREFIX: &str = "test";


pub struct TestResult {
    assert: Assert,
    file: String,
    line: String,
    stdout: String,
    stderr: String,
}

impl TestResult {
    pub fn new(assert: Assert, file: String, line: String, stdout: String, stderr: String) -> Self {
        TestResult {
            assert,
            file,
            line,
            stdout,
            stderr,
        }
    }

    pub fn output(&self) {

        // Wait for test results before we print a new line.
        print!("{} {} - {} ... ", PREFIX, self.line, self.file);

        if !self.assert.resolve() {

            println!(
                "{} -> expected '{}' but got '{}'",
                "FAILED".red(),
                self.stdout,
                self.assert.right
            );

            if self.stderr != "" {
                println!("\n{}", self.stderr.red());
            }

        } else {
            println!("{}", "ok".green());
        }
    }
}
