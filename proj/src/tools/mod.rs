extern crate regex;
use regex::Regex;

pub fn line_trim(line: &str) -> &str {
    let re = Regex::new(r"(?:///)[ ](.+)[^[//]]").unwrap();
    for caps in re.captures_iter(line) {
        return caps.get(1).unwrap().as_str();
    }
    line
}
