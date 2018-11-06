extern crate clap;
extern crate colored;
extern crate regex;

use clap::{App, Arg};
use colored::*;
use regex::Regex;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let matches = App::new("Inline Rust Testing for Native Javascript")
        .version("0.1")
        .author("Pontus L. <pontus.laestadius@gmail.com>")
        .about("Executes Rust-like testing on JavaScript files.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    let contents = read_file(filename).unwrap();

    for block in generate_tests(filename, contents).into_iter() {
        block.consume();
    }
}

pub struct Assert {
    left: String,
    right: String,
}

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

impl Assert {
    pub fn new(line: &str) -> Self {
        let re = Regex::new(r"(?:assert_eq!)\((.+),[ ](.+)(?:\))").unwrap();
        let mut fmt = line.to_string();
        for caps in re.captures_iter(line) {
            return Assert {
                left: caps.get(1).unwrap().as_str().to_string(),
                right: caps.get(2).unwrap().as_str().to_string(),
            };
        }
        panic!("No assert pattern found!")
    }
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
    fn new(file: &str) -> Self {
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
        self.test.push_str(line_trim(line));
        self.test.push('\n');
    }

    pub fn consume(mut self) {
        create_test_file(&self);

        for line in self.test.split('\n') {
            if line == "" {
                continue;
            }

            if !line.starts_with("assert_eq!") {
                self.globals.push_str(line);
                self.globals.push('\n');
                continue;
            }
            let ass = Assert::new(line);
            let i = ass.left.find('(').unwrap();
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
            println!("{}", String::from_utf8_lossy(&proc.stderr).red());
            let res = String::from_utf8_lossy(&proc.stdout);
            if res != ass.right {
                println!("test {} - {} ... {}", line, self.file, "FAILED".red());
                panic!("test failed. left: '{}', right: '{}'", res, ass.right);
            }
            println!("test {} - {} ... {}", line, self.file, "ok".green());
        }
    }
}

fn line_trim(line: &str) -> &str {
    line.trim_start().trim_start_matches("/// ")
}

fn generate_tests(filename: &str, contents: String) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut block: Block = Block::new(filename);
    let BLOCK_MARKER = "```";
    let mut IN_BLOCK = false;
    let mut FUNCTION_CAPTURE = false;
    for line in contents.split('\n') {
        if FUNCTION_CAPTURE {
            println!("{}", line);
            block.function.cont.push_str(line);
            block.function.cont.push('\n');
            if line == "}" {
                FUNCTION_CAPTURE = false;
                blocks.push(block);
                block = Block::new(filename);
            }
        } else if line.contains(BLOCK_MARKER) {
            println!("{}", line.magenta());

            IN_BLOCK = !IN_BLOCK;
            if IN_BLOCK {
                if FUNCTION_CAPTURE {
                    panic!("New code block captured while in function capture");
                }
            } else {
                FUNCTION_CAPTURE = true;
            }
        } else if IN_BLOCK {
            println!("{}", line.green());
            block.push_test_line(line);
        }
    }
    blocks
}

fn read_file(file_path: &str) -> Result<String, io::Error> {
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn create_test_file(block: &Block) -> Result<(), io::Error> {
    let formatted = format!("module.exports={}", block.function.cont);
    let re = Regex::new(r"(return )([^;]+)(;.?)").unwrap();

    let mut file = File::create("o.js")?;

    for line in formatted.split('\n') {
        let mut fmt = line.to_string();
        for caps in re.captures_iter(line) {
            fmt = format!(
                "process.stdout.write(\"\" + ({})); return;",
                caps.get(2).unwrap().as_str()
            );
        }
        file.write(&[b'\n']);
        file.write_all(fmt.as_bytes());
    }
    Ok(())
}
