use comrak::{arena_tree::Node, nodes::Ast, nodes::NodeValue};
use std::{cell::RefCell, fmt::Error};

use crate::utils;

mod npf;
use npf::{
    content_types::{self, text::Formatting, text::FormattingType},
    ContentType, NPF,
};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Formats {
    TumblrNPF,
}

pub fn to_tumblr_npf<'a>(ast: &'a Node<'a, RefCell<Ast>>) -> Result<RefCell<NPF>, Error> {
    let npf = RefCell::new(NPF::<'a>::new());

    utils::iter_nodes_shallow(ast, &|node| match &node.data.borrow().value {
        NodeValue::Paragraph => {
            let text = RefCell::new(String::new());
            let formatting = RefCell::new(Vec::<Formatting>::new());

            utils::iter_nodes_shallow(node, &|node| {
                let mut text = text.borrow_mut();
                let mut formatting = formatting.borrow_mut();

                match &node.data.borrow().value {
                    NodeValue::Link(l) => {
                        let t = utils::extract_text(node);
                        formatting.push(Formatting {
                            r#type: FormattingType::Link,
                            start: text.chars().count(),
                            end: text.chars().count() + t.chars().count() + 1,
                            url: Some(String::from(&l.url)),
                            color: None,
                            blog: None,
                        });
                        text.push_str(&format!("{} ", &t))
                    }
                    NodeValue::Strong => {
                        let t = utils::extract_text(node);
                        formatting.push(Formatting {
                            r#type: FormattingType::Bold,
                            start: text.chars().count(),
                            end: text.chars().count() + t.chars().count() + 1,
                            url: None,
                            color: None,
                            blog: None,
                        });
                        text.push_str(&format!("{} ", &t))
                    }
                    NodeValue::Text(t) => text.push_str(&format!("{} ", &t)),
                    _ => (),
                }
            });

            let mut block =
                content_types::Text::from(text.borrow().trim().to_string().replace("  ", " "));
            block.formating = if formatting.borrow().len() > 0 {
                Some(formatting.borrow().to_vec())
            } else {
                None
            };

            npf.borrow_mut().content.push(ContentType::Text(block));
        }
        _ => (),
    });

    Ok(npf)
}
