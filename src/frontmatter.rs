use std::fmt::Display;

use serde_yaml as yaml;
use std::collections::HashMap;

use comrak::nodes::{AstNode, NodeValue};

#[derive(Debug)]
pub enum FrontmatterErr {
    InvalidFrontmatter,
    Parsing(yaml::Error),
}

#[derive(Debug)]
pub struct Frontmatter {
    map: HashMap<String, yaml::Value>,
}
impl<'a> Frontmatter {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn parse(string: &'a str) -> Result<HashMap<String, yaml::Value>, FrontmatterErr> {
        let mut string = string.trim();
        string = match string.strip_prefix("---") {
            Some(s) => s,
            None => return Err(FrontmatterErr::InvalidFrontmatter),
        };
        string = match string.strip_suffix("---") {
            Some(s) => s,
            None => return Err(FrontmatterErr::InvalidFrontmatter),
        };
        string = string.trim();
        yaml::from_str(string).map_err(|e| FrontmatterErr::Parsing(e))
    }
    pub fn insert(&mut self, key: String, value: yaml::Value) {
        self.map.insert(key, value);
    }
    pub fn remove(&mut self, key: String) -> Option<yaml::Value> {
        self.map.remove(&key)
    }
    pub fn get(&self, key: String) -> Option<&yaml::Value> {
        self.map.get(&key)
    }
    pub fn insert_ast(&self, ast: &'a AstNode<'a>) {
        if let NodeValue::FrontMatter(ref mut f) = &mut ast.data.borrow_mut().value {
            *f = self.to_string();
        } else {
            for c in ast.children() {
                self.insert_ast(c)
            }
        }
    }
}

impl<'a> TryFrom<&'a AstNode<'a>> for Frontmatter {
    type Error = FrontmatterErr;
    fn try_from(value: &'a AstNode<'a>) -> Result<Self, Self::Error> {
        if let NodeValue::FrontMatter(f) = &value.data.borrow().value {
            return Ok(Frontmatter {
                map: Frontmatter::parse(f)?,
            });
        }
        for node in value.children() {
            if let NodeValue::FrontMatter(f) = &value.data.borrow().value {
                return Ok(Frontmatter {
                    map: Frontmatter::parse(f)?,
                });
            } else {
                return Frontmatter::try_from(node);
            }
        }
        Ok(Frontmatter::new())
    }
}

impl Display for Frontmatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match yaml::to_string(&self.map) {
            Ok(s) => s,
            Err(_) => return Err(std::fmt::Error),
        };
        write!(f, "---\n{}---\n\n", string)
    }
}

#[cfg(test)]
mod test {
    use comrak::Arena;

    use crate::utils;

    use super::Frontmatter;

    #[test]
    fn frontmatter_manipulation() {
        let string = "---\n\
                    value1: hello world\n\
                    value2: 5\n\
                    value3: another value\n\
                    ---\n\
                    # Test string\n\
                    A small phrase for testing y'know";

        let arena = Arena::new();
        let ast = comrak::parse_document(&arena, &string, &utils::default_options());

        let mut frontmatter = Frontmatter::try_from(ast).unwrap();

        assert_eq!(
            frontmatter.get(String::from("value1")).unwrap(),
            &serde_yaml::to_value("hello world").unwrap()
        );

        frontmatter.insert(
            String::from("value_test"),
            serde_yaml::to_value("a inserted value").unwrap(),
        );

        assert_eq!(
            frontmatter.remove(String::from("value3")),
            Some(serde_yaml::to_value("another value").unwrap())
        );

        frontmatter.insert_ast(ast);

        let mut res = vec![];
        comrak::format_commonmark(ast, &utils::default_options(), &mut res).unwrap();
        let res = String::from_utf8(res).unwrap();

        let slices = res.split("---").into_iter().collect::<Vec<&str>>();
        let f = slices[1];

        assert_eq!(
            f.lines().find(|l| *l == "value1: hello world").unwrap(),
            "value1: hello world"
        );
        assert_eq!(f.lines().find(|l| *l == "value2: 5").unwrap(), "value2: 5");
        assert_eq!(
            f.lines()
                .find(|l| *l == "value_test: a inserted value")
                .unwrap(),
            "value_test: a inserted value"
        );

        assert_eq!(
            slices[2],
            "\n\n\
            # Test string\n\
            \n\
            A small phrase for testing y'know\n"
        );
    }
}
