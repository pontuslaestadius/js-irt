extern crate colored;
extern crate regex;

use regex::Regex;

use super::parser::*;
use std::fs::{self, DirEntry, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn read(filename: &str) {
    if !filename.ends_with(".js") {
        let fun = |de: &DirEntry| {
            test(de.path().to_str().unwrap());
        };
        visit_dirs(&Path::new(filename), &fun);
    } else {
        test(filename);
    }
}

pub fn test(f: &str) -> std::result::Result<(), std::io::Error> {
    if f.ends_with(".js") {
        let contents = read_file(f)?;
        for block in generate_tests(f, contents).into_iter() {
            create_test_file(&block);
            let test_results = block.consume();
            for test_result in test_results.iter() {
                test_result.output();
            }
        }
    }
    Ok(())
}

pub fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

pub fn generate_tests(filename: &str, contents: String) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut block: Block = Block::new(filename);
    let BLOCK_MARKER = "```";
    let mut IN_BLOCK = false;
    let mut FUNCTION_CAPTURE = false;
    for line in contents.split('\n') {
        if FUNCTION_CAPTURE {
            block.function.cont.push_str(line);
            block.function.cont.push('\n');
            if line == "}" {
                FUNCTION_CAPTURE = false;
                blocks.push(block);
                block = Block::new(filename);
            }
        } else if line.contains(BLOCK_MARKER) {
            IN_BLOCK = !IN_BLOCK;
            if IN_BLOCK {
                if FUNCTION_CAPTURE {
                    panic!("New code block captured while in function capture");
                }
            } else {
                FUNCTION_CAPTURE = true;
            }
        } else if IN_BLOCK {
            block.push_test_line(line);
        }
    }
    blocks
}

pub fn read_file(file_path: &str) -> Result<String, io::Error> {
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn create_test_file(block: &Block) -> Result<(), io::Error> {
    let mut file = File::create("o.js")?;
    file.write_all(block.function.cont.as_bytes());
    let formatted = format!("module.exports={0}", block.function.cont);
    let re = Regex::new(r"(return )([^;]+)(;.?)").unwrap();

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

