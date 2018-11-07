
extern crate colored;
use colored::*;
use super::test_result::*;
use super::assert::*;
use super::util::*;
use std::process::Command;

pub struct Function {
    pub params: Vec<String>,
    pub cont: String,
}

pub struct Block {
    pub file: String,
    pub function: Function,
    pub globals: String,
    pub test: String,
}

impl Function {
    fn new() -> Self {
        Function {
            params: Vec::new(),
            cont: String::new(),
        }
    }
}

impl Block {
    pub fn new(file: &str) -> Self {
        Block {
            file: file.to_string(),
            function: Function::new(),
            globals: String::new(),
            test: String::new(),
        }
    }

    pub fn new_test(&mut self, line: &str) {
        // TODO
    }

    pub fn push_test_line(&mut self, line: &str) {
        let line = line_trim(line);
        if line == "" {
            return;
        }
        self.test.push_str(line);
        self.test.push('\n');
    }

    pub fn consume(mut self) -> Vec<TestResult> {
        let mut test_results = Vec::new();
        for line in self.test.split('\n') {
            if line == "" {
                continue;
            }
            let opt = Assert::parse(line);
            if opt.is_none() {
                self.globals.push_str(line);
                self.globals.push('\n');
                continue;
            }
            let mut ass = opt.unwrap();
            let i = ass.left.find('(').unwrap_or(0);
            let node_cmd = format!(
                "node -e '{} require(\"./o.js\"){}'",
                self.globals,
                ass.left.chars().skip(i).collect::<String>()
            );
            let proc = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&["/C", node_cmd.as_str()])
                    .output()
                    .expect("failed to execute process")
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(node_cmd.as_str())
                    .output()
                    .expect("failed to execute process")
            };
            let stdout = String::from_utf8_lossy(&proc.stdout).to_string();
            let stderr = String::from_utf8_lossy(&proc.stderr).to_string();

            ass.left = stdout.to_string();
            let test_result = TestResult::new(ass, self.file.clone(), line.to_string(), stdout, stderr);
            test_results.push(test_result);
        }
        test_results
    }
}

