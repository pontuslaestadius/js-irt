extern crate colored;
extern crate regex;

use regex::Regex;

use super::parser::*;
use std::fs::{self, DirEntry, File, OpenOptions};
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::result::Result;
use std::thread;
use std::time::Duration;

pub fn read(filename: &str) -> Result<(), Error> {
    let res = if !filename.ends_with(".js") {
        let fun = |de: &DirEntry| -> Result<(), Error> { test(de.path().to_str().unwrap()) };
        visit_dirs(&Path::new(filename), &fun)?
    } else {
        test(filename)?
    };
    Ok(res)
}

pub fn test(f: &str) -> std::result::Result<(), Error> {
    if f.ends_with(".js") && !f.ends_with(".min.js") {
        let contents = read_file(f)?;
        for block in generate_tests(f, &contents).into_iter() {
            create_test_file(&block)?;
            let test_results = block.resolve();

            thread::spawn(move || {
                for test_result in test_results.iter() {
                    test_result.output();
                    thread::sleep(Duration::from_millis(25));
                }
            });
        }
    }
    Ok(())
}

pub fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry) -> Result<(), Error>) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

pub fn generate_tests(filename: &str, contents: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut block: Block = Block::new(filename);
    let block_marker = "```";
    let mut in_block = false;
    let mut function_capture = false;
    for line in contents.split('\n') {
        if function_capture {
            block.function.cont.push_str(line);
            block.function.cont.push('\n');
            if line == "}" {
                function_capture = false;
                blocks.push(block);
                block = Block::new(filename);
            }
        } else if line.contains(block_marker) {
            in_block = !in_block;
            if in_block {
                if function_capture {
                    panic!("New code block captured while in function capture");
                }
            } else {
                function_capture = true;
            }
        } else if in_block {
            block.push_test_line(line);
        }
    }
    blocks
}

pub fn read_file(file_path: &str) -> Result<String, Error> {
    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn append_test_to_file(file_path: &str, append: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    if let Err(e) = writeln!(file, "\n// ##AUTOGEN##\n") {
        eprintln!("Couldn't write to file: {}", e);
    };
    if let Err(e) = writeln!(file, "{}", append) {
        eprintln!("Couldn't write to file: {}", e);
    };
}

pub fn retract_test_from_file(_file_path: &str) {
    // TODO
}

pub fn create_test_file(block: &Block) -> Result<(), Error> {
    let mut file = File::create("o.js")?;
    file.write_all(block.function.cont.as_bytes())?;
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
        let _ = file.write(&[b'\n'])?;
        file.write_all(fmt.as_bytes())?;
    }
    Ok(())
}
