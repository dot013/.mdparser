use std::cell::RefCell;

use serde_yaml as yaml;
use std::collections::HashMap;

use comrak::arena_tree::Node;
use comrak::nodes::{Ast, NodeValue};

use crate::utils;

#[derive(Debug)]
pub struct Frontmatter {
    map: HashMap<String, yaml::Value>,
}

impl<'a> Frontmatter {
    pub fn new() -> Frontmatter {
        Frontmatter {
            map: HashMap::new(),
        }
    }
    pub fn get(&self, key: &str) -> Option<&yaml::Value> {
        self.map.get(key)
    }
    pub fn set(&mut self, key: &str, value: yaml::Value) {
        self.map.insert(String::from(key), value);
    }
    pub fn rename_prop(&mut self, from: &str, to: &str) {
        if let Some(value) = self.map.get(from) {
            self.map.insert(String::from(to), value.clone());
            self.map.remove(from);
        }
    }
    pub fn place_on_ast(
        self,
        ast: &'a Node<'a, RefCell<Ast>>,
    ) -> RefCell<Result<Option<String>, yaml::Error>> {
        let result = RefCell::new(Ok(None));
        utils::iter_nodes(ast, &|node| {
            if let NodeValue::FrontMatter(ref mut f) = &mut node.data.borrow_mut().value {
                match String::try_from(&self) {
                    Ok(s) => {
                        // I'm sure there's a way to not use clone here, something in the lines
                        // of passing the pointer of f to the result, and then changing the pointer
                        // of f to be equals to s. But premature optimisation is the root of all
                        // evil, and this is a not a "serious" project. When I better learn Rust,
                        // I hopefully learn how to do this.
                        //
                        // - Gustavo "Guz" L. de Mello (2024-04-18)
                        *result.borrow_mut() = Ok(Some(f.clone()));
                        f.replace_range(.., &s);
                    }
                    Err(e) => {
                        *result.borrow_mut() = Err(e);
                    }
                };
            }
        });
        result
    }
}

impl<'a> TryFrom<&'a Node<'a, RefCell<Ast>>> for Frontmatter {
    type Error = yaml::Error;
    fn try_from(value: &'a Node<'a, RefCell<Ast>>) -> Result<Self, Self::Error> {
        let str = RefCell::new(String::new());
        utils::iter_nodes(value, &|node| {
            if let NodeValue::FrontMatter(f) = &node.data.borrow_mut().value {
                *str.borrow_mut() = f.to_string();
            };
        });
        let s = str.borrow().clone();
        Frontmatter::try_from(s)
    }
}

impl TryFrom<&str> for Frontmatter {
    type Error = yaml::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = if value.trim().starts_with("---") {
            value.split("---").collect::<Vec<&str>>()[1]
        } else {
            value
        };
        Ok(Frontmatter {
            map: yaml::from_str::<HashMap<String, yaml::Value>>(&value)?,
        })
    }
}

impl TryFrom<String> for Frontmatter {
    type Error = yaml::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Frontmatter::try_from(value.as_str())
    }
}

impl TryFrom<&String> for Frontmatter {
    type Error = yaml::Error;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Frontmatter::try_from(value.as_str())
    }
}

impl AsRef<Frontmatter> for Frontmatter {
    fn as_ref(&self) -> &Frontmatter {
        self
    }
}

impl AsMut<Frontmatter> for Frontmatter {
    fn as_mut(&mut self) -> &mut Frontmatter {
        self
    }
}

impl TryFrom<Frontmatter> for String {
    type Error = yaml::Error;
    fn try_from(value: Frontmatter) -> Result<Self, Self::Error> {
        String::try_from(&value)
    }
}

impl TryFrom<&Frontmatter> for String {
    type Error = yaml::Error;
    fn try_from(value: &Frontmatter) -> Result<Self, Self::Error> {
        Ok(format!("---\n{}---\n\n", yaml::to_string(&value.map)?))
    }
}

impl From<Frontmatter> for HashMap<String, yaml::Value> {
    fn from(value: Frontmatter) -> Self {
        value.map
    }
}

impl From<&Frontmatter> for HashMap<String, yaml::Value> {
    fn from(value: &Frontmatter) -> Self {
        value.map.clone()
    }
}
