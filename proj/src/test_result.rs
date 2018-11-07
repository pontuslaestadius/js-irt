
extern crate colored;
use colored::*;
use super::assert::*;

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
        print!("test {} - {} ... ", self.line, self.file);
        if !self.assert.resolve() {
            println!(
                "{} -> left: '{}', right: '{}'",
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

