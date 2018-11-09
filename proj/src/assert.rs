extern crate regex;
use regex::Regex;

#[derive(Debug)]
pub enum AssertType {
    Tr,
    Eq,
    Ne,
}

pub struct Assert {
    pub assert_type: AssertType,
    pub left: String,
    pub right: String,
}

impl Assert {
    fn new(line: &str, assert_type: AssertType) -> Self {
        let re = match assert_type {
            AssertType::Tr => Regex::new(r"!\((.+)(\))"),
            _ => Regex::new(r"_(?:eq|ne)!\((.+),(.+)\)"),
        };
        for caps in re.unwrap().captures_iter(line) {
            return Assert {
                assert_type,
                left: caps.get(1).unwrap().as_str().to_string(),
                right: caps.get(2).unwrap().as_str().to_string().trim().to_string(),
            };
        }
        panic!(
            "No assert pattern found for: {} type: {:?}",
            line, assert_type
        )
    }

    /// # Examples
    ///
    /// ```
    /// let line = "assert!(true)";
    /// let assert = Assert::parse(line);
    /// assert!(assert.resolve());
    /// ```
    /// ```
    /// let line = "assert_ne!(false, true)";
    /// # let assert = Assert::parse(line);
    /// # assert!(assert.resolve());
    /// ```
    /// ```
    /// let line = "assert_eq!(true, true)";
    /// # let assert = Assert::parse(line);
    /// # assert!(assert.resolve());
    /// ```
    pub fn resolve(&self) -> bool {
        match self.assert_type {
            AssertType::Tr => self.left == "true",
            AssertType::Eq => self.left == self.right,
            AssertType::Ne => self.left != self.right,
        }
    }

    pub fn parse(line: &str) -> Option<Self> {
        if line.starts_with("assert!") {
            Some(Assert::new(line, AssertType::Tr))
        } else if line.starts_with("assert_eq!") {
            Some(Assert::new(line, AssertType::Eq))
        } else if line.starts_with("assert_ne!") {
            Some(Assert::new(line, AssertType::Ne))
        } else {
            None
        }
    }
}
