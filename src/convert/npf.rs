use std::cell::RefCell;

use comrak::{
    arena_tree::{Children, Node},
    nodes::{Ast, NodeValue},
};

pub mod attributions;
pub mod content_blocks;
pub mod layout_blocks;
pub mod objects;
pub mod text_formatting;

mod objects_post;

use content_blocks::{BlockText, BlockValue};
use text_formatting::{FormatTypeBold, FormatTypeItalic, FormatValue};

use text_formatting::{FormatTypeLink, FormatTypeStrikeThrough};

#[derive(Debug)]
pub enum NPFConvertError {
    TODO,
    InvalidURL { url: String, err: url::ParseError },
}

impl<'a> TryFrom<Children<'a, RefCell<Ast>>> for objects::Post {
    type Error = NPFConvertError;
    fn try_from(mut nodes: Children<'a, RefCell<Ast>>) -> Result<Self, Self::Error> {
        nodes.try_fold(Self::new(0), |mut acc, n| {
            acc.content.append(&mut Self::try_from(n)?.content);
            Ok(acc)
        })
    }
}

impl<'a> TryFrom<&'a Node<'a, RefCell<Ast>>> for objects::Post {
    type Error = NPFConvertError;
    fn try_from(node: &'a Node<'a, RefCell<Ast>>) -> Result<Self, Self::Error> {
        match &node.data.borrow().value {
            NodeValue::Document => Self::try_from(node.children()),
            NodeValue::Paragraph => {
                let p = Self::try_from(node.children())?.fold_content();
                println!("{p:#?}");
                Ok(p)
            }
            NodeValue::Text(t) => {
                let mut post = Self::new(0);
                let block_text = BlockText::from(String::from(t.clone()));
                post.content.push(BlockValue::Text(block_text));
                Ok(post)
            }
            NodeValue::Strong => Ok(Self::try_from(node.children())?
                .fold_content()
                .for_each_content(|c| {
                    if let BlockValue::Text(ref mut t) = c {
                        let format = FormatValue::Bold(FormatTypeBold::from(&t.text));
                        t.push_formatting(format);
                        // t.text = String::from(t.text.trim());
                    }
                })),
            NodeValue::Emph => Ok(Self::try_from(node.children())?
                .fold_content()
                .for_each_content(|c| {
                    if let BlockValue::Text(ref mut t) = c {
                        let format = FormatValue::Italic(FormatTypeItalic::from(&t.text));
                        t.push_formatting(format);
                        // t.text = String::from(t.text.trim());
                    }
                })),
            NodeValue::Strikethrough => Ok(Self::try_from(node.children())?
                .fold_content()
                .for_each_content(|c| {
                    if let BlockValue::Text(ref mut t) = c {
                        let format =
                            FormatValue::StrikeThrough(FormatTypeStrikeThrough::from(&t.text));
                        t.push_formatting(format);
                        // t.text = String::from(t.text.trim());
                    }
                })),
            NodeValue::Link(link) => match url::Url::parse(&link.url) {
                Ok(url) => Ok(Self::try_from(node.children())?
                    .fold_content()
                    .for_each_content(|c| {
                        if let BlockValue::Text(ref mut t) = c {
                            let mut format = FormatTypeLink::from(&t.text);
                            format.url = url.clone();
                            t.push_formatting(FormatValue::Link(format));
                            // t.text = String::from(t.text.trim());
                        }
                    })),
                Err(err) => Err(NPFConvertError::InvalidURL {
                    url: link.url.clone(),
                    err,
                }),
            },
            NodeValue::SoftBreak => {
                let mut post = Self::new(0);
                post.content.push(BlockValue::Text(BlockText::from(" ")));
                Ok(post)
            }
            NodeValue::LineBreak => {
                let mut post = Self::new(0);
                post.content.push(BlockValue::Text(BlockText::from("\n")));
                Ok(post)
            }
            _ => Ok(Self::new(0)),
        }
    }
}

pub fn from<'a>(node: &'a Node<'a, RefCell<Ast>>) -> Result<objects::Post, NPFConvertError> {
    objects::Post::try_from(node)
}

/*
Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis
*/
