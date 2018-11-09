extern crate regex;
use regex::Regex;

pub fn line_trim(line: &str) -> &str {
    let re = Regex::new(r"(?:///)[ ](.+)[^[//]]").unwrap();
    let cap = re.captures(line).unwrap();
    cap.get(1).unwrap().as_str()
}

