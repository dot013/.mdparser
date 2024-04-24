use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
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
                    let mut content = Self::try_from(n)?.content;
                    let mut res = content.iter_mut().fold(
                        BlockText::new(&String::new()),
                        |mut acc, c| match c {
                            BlockValue::Text(t) => {
                                let text = &t.text.trim();
                                if let Some(ref mut f) = &mut t.formatting {
                                    let offset = acc.text.chars().count() as u64;
                                    f.iter_mut().for_each(|f| {
                                        f.offset(offset);
                                    });
                                    if let Some(ref mut af) = acc.formatting {
                                        af.append(f);
                                    } else {
                                        acc.formatting = Some(f.to_vec());
                                    }
                                }
                                acc.text.push_str(&format!("{} ", text));
                                acc
                            }
                            _ => acc,
                        },
                    );
                    res.text = res.text.trim().to_string();
                    let format = FormatValue::Bold(FormatTypeBold::from(&res.text));
                    if let Some(ref mut f) = res.formatting {
                        f.push(format);
                    } else {
                        res.formatting = Some(vec![format]);
                    }
                    post.content.push(BlockValue::Text(res));
                    Ok(())
                }
                NodeValue::Emph => {
                    let mut content = Self::try_from(n)?.content;
                    let mut res = content.iter_mut().fold(
                        BlockText::new(&String::new()),
                        |mut acc, c| match c {
                            BlockValue::Text(t) => {
                                let text = &t.text.trim();
                                if let Some(ref mut f) = &mut t.formatting {
                                    let offset = acc.text.chars().count() as u64;
                                    f.iter_mut().for_each(|f| {
                                        f.offset(offset);
                                    });
                                    if let Some(ref mut af) = acc.formatting {
                                        af.append(f);
                                    } else {
                                        acc.formatting = Some(f.to_vec());
                                    }
                                }
                                acc.text.push_str(&format!("{} ", text));
                                acc
                            }
                            _ => acc,
                        },
                    );
                    res.text = res.text.trim().to_string();
                    let format = FormatValue::Italic(FormatTypeItalic::from(&res.text));
                    if let Some(ref mut f) = res.formatting {
                        f.push(format);
                    } else {
                        res.formatting = Some(vec![format]);
                    }
                    // println!("italic {:#?}", res);
                    post.content.push(BlockValue::Text(res));
                    Ok(())
                }
                _ => Ok(()),
            })
            .collect();
        if let Err(e) = r {
            Err(e)
        } else {
            // println!("{:#?}", post);
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
