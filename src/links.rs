use std::cell::RefCell;
use std::path::PathBuf;

use comrak::arena_tree::Node;
use comrak::nodes::{Ast, NodeLink, NodeValue};

use crate::utils;

pub struct ParseOptions {
    pub alias_prop: Option<String>,
    pub path_root: PathBuf,
    pub to_complete_paths: bool,
    pub remove_unalised: bool,
    pub remove_unfound: bool,
}
impl Default for ParseOptions {
    fn default() -> Self {
        ParseOptions {
            alias_prop: None,
            path_root: PathBuf::new(),
            to_complete_paths: true,
            remove_unalised: true,
            remove_unfound: true,
        }
    }
}

pub fn iterate_links<'a, F>(ast: &'a Node<'a, RefCell<Ast>>, iterator: F)
where
    F: Fn(&mut NodeLink),
{
    let _ = utils::iter_nodes(ast, &|node| {
        if let NodeValue::Link(ref mut l) = &mut node.data.borrow_mut().value {
            iterator(l);
        };
        Ok::<(), ()>(())
    });
}

pub fn get_links<'a>(ast: &'a Node<'a, RefCell<Ast>>) -> Vec<String> {
    let links: RefCell<Vec<String>> = RefCell::new(vec![]);
    let _ = utils::iter_nodes(ast, &|node| {
        if let NodeValue::Link(l) = &node.data.borrow().value {
            links.borrow_mut().push(l.url.clone());
        }
        Ok::<(), ()>(())
    });
    let r = links.borrow().to_vec();
    r
}
