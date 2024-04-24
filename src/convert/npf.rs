use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::VecDeque,
};

use comrak::{
    arena_tree::Node,
    nodes::{Ast, AstNode, NodeValue},
};

pub mod attributions;
pub mod content_blocks;
pub mod layout_blocks;
pub mod objects;
pub mod text_formatting;

mod objects_post;

use content_blocks::{BlockText, BlockValue};
use text_formatting::{FormatTypeBold, FormatTypeItalic, FormatValue};

use self::content_blocks::BlockImage;

#[derive(Debug)]
pub enum NPFConvertError {
    TODO,
}

fn extract_text(contents: &Vec<BlockValue>) -> String {
    contents
        .iter()
        .fold(String::new(), |mut a, c| {
            if let BlockValue::Text(block) = c {
                a.push_str(&format!(" {}", block.text));
            }
            a
        })
        .trim()
        .to_string()
}

impl<'a> TryFrom<&'a Node<'a, RefCell<Ast>>> for objects::Post {
    type Error = NPFConvertError;
    fn try_from(value: &'a Node<'a, RefCell<Ast>>) -> Result<Self, Self::Error> {
        let mut post = Self::new(0);

        let nodes = value.children().into_iter();
        let r: Result<Vec<_>, NPFConvertError> = nodes
            .map(|n| match &n.data.borrow().value {
                NodeValue::Paragraph => {
                    let mut paragraph_contents = Self::try_from(n)?.content;
                    post.content.append(&mut paragraph_contents);
                    Ok(())
                }
                NodeValue::Text(t) => {
                    let block_text = BlockText::from(String::from(t.clone()));
                    post.content.push(BlockValue::Text(block_text));
                    Ok(())
                }
                NodeValue::Strong => {
                    let mut content = Self::try_from(n)?
                        .fold_content()
                        .for_each_content(|c| {
                            if let BlockValue::Text(ref mut t) = c {
                                let format = FormatValue::Bold(FormatTypeBold::from(&t.text));
                                if let Some(ref mut f) = t.formatting {
                                    f.push(format);
                                } else {
                                    t.formatting = Some(vec![format]);
                                }
                                t.text = String::from(t.text.trim());
                            }
                        })
                        .content;
                    post.content.append(&mut content);
                    Ok(())
                }
                NodeValue::Emph => {
                    let mut content = Self::try_from(n)?
                        .fold_content()
                        .for_each_content(|c| {
                            if let BlockValue::Text(ref mut t) = c {
                                let format = FormatValue::Italic(FormatTypeItalic::from(&t.text));
                                if let Some(ref mut f) = t.formatting {
                                    f.push(format);
                                } else {
                                    t.formatting = Some(vec![format]);
                                }
                                t.text = String::from(t.text.trim());
                            }
                        })
                        .content;
                    post.content.append(&mut content);
                    // println!("{:#?}", post);

                    Ok(())
                }
                _ => Ok(()),
            })
            .collect();
        if let Err(e) = r {
            Err(e)
        } else {
            println!("{:#?}", post);
            Ok(post)
        }
    }
}

pub fn from<'a>(node: &'a Node<'a, RefCell<Ast>>) -> Result<objects::Post, NPFConvertError> {
    objects::Post::try_from(node)
}

/*
Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis
*/
