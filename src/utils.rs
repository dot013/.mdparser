use std::cell::RefCell;

use comrak::nodes::NodeValue;

pub fn default_options() -> comrak::Options {
    let mut opts = comrak::Options::default();

    opts.render.width = 100;
    opts.render.hardbreaks = false;

    opts.extension.strikethrough = true;
    opts.extension.front_matter_delimiter = Some("---".to_owned());

    opts
}

pub fn iter_nodes<'a, F>(node: &'a comrak::nodes::AstNode<'a>, f: &F)
where
    F: Fn(&'a comrak::nodes::AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f)
    }
}

pub fn iter_nodes_err<'a, F, E>(node: &'a comrak::nodes::AstNode<'a>, f: &F) -> Result<(), E>
where
    F: Fn(&'a comrak::nodes::AstNode<'a>) -> Result<(), E>,
{
    f(node)?;
    for c in node.children() {
        iter_nodes_err(c, f)?
    }
    Ok(())
}

pub fn iter_nodes_shallow<'a, F, E>(node: &'a comrak::nodes::AstNode<'a>, f: &F) -> Result<(), E>
where
    F: Fn(&'a comrak::nodes::AstNode<'a>) -> Result<(), E>,
{
    for c in node.children() {
        f(c)?
    }
    Ok(())
}

pub fn iter_nodes_r<'a, F, T>(node: &'a comrak::nodes::AstNode<'a>, f: &F) -> Option<T>
where
    F: Fn(&'a comrak::nodes::AstNode<'a>) -> Option<T>,
{
    let result = f(node);
    if let Some(r) = result {
        return Some(r);
    }
    for c in node.children() {
        let result = iter_nodes_r(c, f);
        if let Some(r) = result {
            return Some(r);
        }
    }
    None
}

pub fn extract_text<'a>(node: &'a comrak::nodes::AstNode<'a>) -> String {
    let text = RefCell::new(String::new());
    iter_nodes(node, &|node| {
        if let NodeValue::Text(t) = &node.data.borrow().value {
            text.borrow_mut().push_str(&t);
        }
    });
    let r = text.borrow().to_string();
    r
}
