extern crate colored;
use super::assert::*;
use super::test_result::*;
use super::tools::*;
use std::process::Command;

const PROCESS_ERROR_MESSAGE: &str = "failed to execute process";

pub struct Function {
    pub params: Vec<String>,
    pub cont: String,
}

pub struct TestFile {
    pub blocks: Vec<Block>,
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

    pub fn new_test(&mut self, _line: &str) {
        // TODO
    }

    pub fn push_test_line(&mut self, line: &str) {
        match line_trim(line) {
            "" => (),
            l => {
                self.test.push_str(l);
                self.test.push('\n');
            }
        };
    }

    pub fn resolve(mut self) -> Vec<TestResult> {
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
            let i = ass.left.find('(');

            let eval = if i.is_some() {
                format!(
                    "require(\"./.irt.out.js\"){}",
                    ass.left.chars().skip(i.unwrap()).collect::<String>()
                )
            } else {
                format!("process.stdout.write(\"\" + {})", ass.left)
            };

            let node_cmd = format!(
                "node -e '{} {} {};'",
                self.function.cont, self.globals, eval,
            );

            println!("Executing: sh -c {}", node_cmd);

            let proc = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&["/C", node_cmd.as_str()])
                    .output()
                    .expect(PROCESS_ERROR_MESSAGE)
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(node_cmd.as_str())
                    .output()
                    .expect(PROCESS_ERROR_MESSAGE)
            };
            ass.left = String::from_utf8_lossy(&proc.stdout).to_string();
            test_results.push(TestResult::new(
                ass,
                self.file.clone(),
                line.to_string(),
                String::from_utf8_lossy(&proc.stdout).to_string(),
                String::from_utf8_lossy(&proc.stderr).to_string(),
            ));
        }
        test_results
    }
}
