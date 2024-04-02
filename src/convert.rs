use comrak::{arena_tree::Node, nodes::Ast, nodes::NodeValue};
use std::{cell::RefCell, fmt::Error};

use crate::utils;

mod npf;
use npf::content_types::text::{Formatting, FormattingType, Subtypes};
use npf::{content_types, ContentType, NPF};

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
                    NodeValue::Emph => {
                        let t = utils::extract_text(node);
                        formatting.push(Formatting {
                            r#type: FormattingType::Italic,
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
                };

                Ok::<(), Error>(())
            })?;

            let text = text.borrow().trim().to_string().replace("  ", " ");
            let mut block = content_types::Text::from(text);

            block.formatting = if formatting.borrow().len() > 0 {
                Some(formatting.borrow().to_vec())
            } else {
                None
            };

            npf.borrow_mut().content.push(ContentType::Text(block));

            Ok::<(), Error>(())
        }
        NodeValue::BlockQuote => {
            let block_quote = to_tumblr_npf(node)?;
            let final_block = RefCell::new(content_types::Text::new());

            block_quote.borrow_mut().content.iter_mut().for_each(|b| {
                if let ContentType::Text(t) = b {
                    let mut fb = final_block.borrow_mut();
                    let text_len = fb.text.chars().count();

                    if let Some(formattings) = &t.formatting {
                        for formatting in formattings {
                            match fb.formatting {
                                Some(ref mut v) => v.push(Formatting {
                                    start: text_len + formatting.start,
                                    end: text_len + formatting.end,
                                    ..formatting.clone()
                                }),
                                None => {
                                    fb.formatting = Some(vec![Formatting {
                                        start: text_len + formatting.start,
                                        end: text_len + formatting.end,
                                        ..formatting.clone()
                                    }]);
                                }
                            }
                        }
                    }
                    fb.text.push_str(&format!("{}\n\n", &t.text));
                } else {
                }
            });

            Ok(())
        }
        NodeValue::Heading(h) => {
            let mut text = content_types::Text::from(utils::extract_text(node));

            if h.level == 1 {
                text.subtype = Some(Subtypes::Heading1);
            } else if h.level == 2 {
                text.subtype = Some(Subtypes::Heading2);
            } else if h.level == 3 {
                text.subtype = Some(Subtypes::UnorderedListItem);
                let formatting = Formatting {
                    r#type: FormattingType::Bold,
                    start: 0,
                    end: text.text.chars().count() + 1,
                    url: None,
                    color: None,
                    blog: None,
                };
                match text.formatting {
                    Some(ref mut f) => f.push(formatting),
                    None => text.formatting = Some(vec![formatting]),
                };
            } else {
                text.subtype = Some(Subtypes::UnorderedListItem);
            }

            npf.borrow_mut().content.push(ContentType::Text(text));

            Ok(())
        }
        _ => Ok(()),
    })?;

    Ok(npf)
}
