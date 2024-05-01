use std::cell::RefCell;

use comrak::arena_tree::Node;
use comrak::nodes::{Ast, NodeLink, NodeValue};

use crate::utils;

pub fn iterate_links<'a, F>(ast: &'a Node<'a, RefCell<Ast>>, iterator: F)
where
    F: Fn(&mut NodeLink),
{
    utils::iter_nodes(ast, &|node| {
        if let NodeValue::Link(ref mut l) = &mut node.data.borrow_mut().value {
            iterator(l);
        };
    });
}

pub fn replace_links<'a>(ast: &'a Node<'a, RefCell<Ast>>, from: &'a str, to: &'a str) {
    iterate_links(ast, |l| {
        if l.url == from {
            l.url = String::from(to)
        }
    });
}

pub fn remove_link<'a>(ast: &'a Node<'a, RefCell<Ast>>, url: &'a str) {
    utils::iter_nodes(ast, &|node| {
        if let NodeValue::Link(ref mut l) = &mut node.data.borrow_mut().value {
            if l.url == url {
                node.children().for_each(|n| node.insert_before(n));
                node.detach();
            }
        }
    })
}

pub fn get_links<'a>(ast: &'a Node<'a, RefCell<Ast>>) -> Vec<String> {
    let links: RefCell<Vec<String>> = RefCell::new(vec![]);
    iterate_links(ast, |l| links.borrow_mut().push(l.url.clone()));
    let r = links.borrow().to_vec();
    r
}
