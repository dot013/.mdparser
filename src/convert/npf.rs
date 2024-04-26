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

use self::{objects::BlogInfo, text_formatting::FormatTypeMention};

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
            NodeValue::Link(link) => {
                let content = Self::try_from(node.children())?.fold_content();

                #[cfg(feature = "uuid-link-to-mention")]
                if link.url.starts_with("t:") {
                    return Ok(content.for_each_content(|c| {
                        if let BlockValue::Text(ref mut t) = c {
                            let blog = BlogInfo::new(&link.url);
                            let format =
                                FormatTypeMention::new(0..t.text.chars().count() as u64, blog);
                            t.push_formatting(FormatValue::Mention(format));
                        }
                    }));
                }

                match url::Url::parse(&link.url) {
                    Ok(url) => Ok(content.for_each_content(|c| {
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
                }
            }
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

#[cfg(test)]
mod tests {

    use super::content_blocks::BlockValue;
    use crate::convert::npf;
    use crate::convert::npf::objects::BlogInfo;
    use crate::convert::npf::text_formatting::{
        FormatTypeBold, FormatTypeItalic, FormatTypeLink, FormatTypeMention,
        FormatTypeStrikeThrough, FormatValue,
    };
    use crate::utils;
    use comrak::Arena;

    macro_rules! assert_eq_text {
        ($b:expr, $s:tt) => {
            match ($b, $s) {
                (left, right) => {
                    if let BlockValue::Text(b) = left {
                        assert_eq!(b.text, right);
                    } else {
                        panic!("Given block doesn't match BlockValue::Text\n{:#?}", left);
                    }
                }
            }
        };
    }

    macro_rules! extrac_formatting {
        ($b:expr) => {{
            let block = $b;
            if let BlockValue::Text(b) = block {
                b.formatting
                    .clone()
                    .unwrap_or_else(|| {
                        panic!("Given block doesn't have a formatting vector {:#?}", block)
                    })
                    .to_vec()
            } else {
                panic!("Given block doesn't match BlockValue::Text {:#?}", block);
            }
        }};
    }

    macro_rules! assert_eq_formatting {
        ($a:ident, $b:ident) => {
            let vec_a = $a;
            $b.iter().enumerate().for_each(|(i, f)| match f {
                FormatValue::Bold(f) => {
                    if let FormatValue::Bold(f2) = &vec_a[i] {
                        assert_eq!(f.start, f2.start);
                        assert_eq!(f.end, f2.end);
                    } else {
                        panic!(
                            "Formatting value aren't the same on {f:#?} and {:#?}",
                            &vec_a[i]
                        );
                    }
                }
                FormatValue::Italic(f) => {
                    if let FormatValue::Italic(f2) = &vec_a[i] {
                        assert_eq!(f.start, f2.start);
                        assert_eq!(f.end, f2.end);
                    } else {
                        panic!(
                            "Formatting value aren't the same on {f:#?} and {:#?}",
                            &vec_a[i]
                        );
                    }
                }
                FormatValue::StrikeThrough(f) => {
                    if let FormatValue::StrikeThrough(f2) = &vec_a[i] {
                        assert_eq!(f.start, f2.start);
                        assert_eq!(f.end, f2.end);
                    } else {
                        panic!(
                            "Formatting value aren't the same on {f:#?} and {:#?}",
                            &vec_a[i]
                        );
                    }
                }
                FormatValue::Small(f) => {
                    if let FormatValue::Small(f2) = &vec_a[i] {
                        assert_eq!(f.start, f2.start);
                        assert_eq!(f.end, f2.end);
                    } else {
                        panic!(
                            "Formatting value aren't the same on {f:#?} and {:#?}",
                            &vec_a[i]
                        );
                    }
                }
                FormatValue::Link(f) => {
                    if let FormatValue::Link(f2) = &vec_a[i] {
                        assert_eq!(f.start, f2.start);
                        assert_eq!(f.end, f2.end);
                        assert_eq!(f.url, f2.url);
                    } else {
                        panic!(
                            "Formatting value aren't the same on {f:#?} and {:#?}",
                            &vec_a[i]
                        );
                    }
                }
                FormatValue::Mention(f) => {
                    if let FormatValue::Mention(f2) = &vec_a[i] {
                        assert_eq!(f.start, f2.start);
                        assert_eq!(f.end, f2.end);
                        assert_eq!(f.blog.uuid, f2.blog.uuid);
                        assert_eq!(f.blog.name, f2.blog.name);
                        assert_eq!(f.blog.url, f2.blog.url);
                    } else {
                        panic!(
                            "Formatting value aren't the same on {f:#?} and {:#?}",
                            &vec_a[i]
                        );
                    }
                }
                FormatValue::Color(f) => {
                    if let FormatValue::Color(f2) = &vec_a[i] {
                        assert_eq!(f.start, f2.start);
                        assert_eq!(f.end, f2.end);
                        assert_eq!(f.hex, f2.hex);
                    } else {
                        panic!(
                            "Formatting value aren't the same on {f:#?} and {:#?}",
                            &vec_a[i]
                        );
                    }
                }
            });
        };
    }

    #[test]
    fn text_block_plain() {
        let markdown = "Hello world, this is a test of markdown.";
        let arena = Arena::new();
        let ast = comrak::parse_document(&arena, &markdown, &utils::default_options());

        let npf = npf::from(&ast).unwrap();

        assert_eq_text!(&npf.content[0], "Hello world, this is a test of markdown.");
    }

    #[test]
    fn text_block_formatting() {
        let markdown = "Hello world, **this is a test of markdown**.";
        let arena = Arena::new();
        let ast = comrak::parse_document(&arena, &markdown, &utils::default_options());

        let npf = npf::from(&ast).unwrap();

        let formatting = vec![FormatValue::Bold(FormatTypeBold::from(13..39))];
        let npf_formatting = extrac_formatting!(&npf.content[0]);

        assert_eq_formatting!(formatting, npf_formatting);
        assert_eq_text!(&npf.content[0], "Hello world, this is a test of markdown.");
    }

    #[test]
    fn text_block_formatting_nested() {
        let markdown = "Hello world, **this [is a test of](https://guz.one) markdown**.";
        let arena = Arena::new();
        let ast = comrak::parse_document(&arena, &markdown, &utils::default_options());

        let npf = npf::from(&ast).unwrap();

        let formatting = vec![
            FormatValue::Link(FormatTypeLink::new(
                18..30,
                url::Url::parse("https://guz.one").unwrap(),
            )),
            FormatValue::Bold(FormatTypeBold::from(13..39)),
        ];
        let npf_formatting = extrac_formatting!(&npf.content[0]);

        assert_eq_formatting!(formatting, npf_formatting);
        assert_eq_text!(&npf.content[0], "Hello world, this is a test of markdown.");
    }

    #[test]
    fn text_block_formatting_broken() {
        // This isn't "valid" markdown, so the conversion should reflect that
        let markdown = "Hello [world, *this is](https://guz.one) a test of markdown*.";
        let arena = Arena::new();
        let ast = comrak::parse_document(&arena, &markdown, &utils::default_options());

        let npf = npf::from(&ast).unwrap();

        let formatting = vec![FormatValue::Link(FormatTypeLink::new(
            6..21,
            url::Url::parse("https://guz.one").unwrap(),
        ))];
        let npf_formatting = extrac_formatting!(&npf.content[0]);

        assert_eq_formatting!(formatting, npf_formatting);
        assert_eq_text!(
            &npf.content[0],
            "Hello world, *this is a test of markdown*."
        );
    }

    #[test]
    fn text_block_formatting_complex() {
        let markdown = "Hello [world, *this is*](https://guz.one) ~~a test of markdown~~.";
        let arena = Arena::new();
        let ast = comrak::parse_document(&arena, &markdown, &utils::default_options());

        let npf = npf::from(&ast).unwrap();

        let formatting = vec![
            FormatValue::Italic(FormatTypeItalic::from(13..20)),
            FormatValue::Link(FormatTypeLink::new(
                6..20,
                url::Url::parse("https://guz.one").unwrap(),
            )),
            FormatValue::StrikeThrough(FormatTypeStrikeThrough::from(21..39)),
        ];
        let npf_formatting = extrac_formatting!(&npf.content[0]);

        assert_eq_formatting!(formatting, npf_formatting);
        assert_eq_text!(&npf.content[0], "Hello world, this is a test of markdown.");
    }

    #[test]
    fn text_block_paragraph() {
        #[cfg(not(feature = "uuid-link-to-mention"))]
        let markdown = "If **you** are reading this, thanks for giving a look\n\
                        and checking the ~~ugly~~ source code of this *little\n\
                        **personal** project*. It is heart warming to know that *at least*\n\
                        someone found this interesting and maybe useful, even knowing\n\
                        how niched this whole project is.\\
                        - [Gustavo \"Guz\" L. de Mello](https://guz.one), Apr 16, 12.2024";

        #[cfg(feature = "uuid-link-to-mention")]
        let markdown = "If **you** are reading this, thanks for giving a look\n\
                        and checking the ~~ugly~~ source code of this *little\n\
                        **personal** project*. It is heart warming to know that *at least*\n\
                        [someone](t:_YENQUPzd_oPpmVDqZQ-yw) found this interesting and maybe useful, even knowing\n\
                        how niched this whole project is.\\
                        - [Gustavo \"Guz\" L. de Mello](https://guz.one), Apr 16, 12.2024";

        let arena = Arena::new();
        let ast = comrak::parse_document(&arena, &markdown, &utils::default_options());

        let npf = npf::from(&ast).unwrap();
        let formatting = vec![
            FormatValue::Bold(FormatTypeBold::from(3..6)),
            FormatValue::StrikeThrough(FormatTypeStrikeThrough::from(67..71)),
            FormatValue::Bold(FormatTypeBold::from(99..107)),
            FormatValue::Italic(FormatTypeItalic::from(92..115)),
            FormatValue::Italic(FormatTypeItalic::from(150..158)),
            #[cfg(feature = "uuid-link-to-mention")]
            FormatValue::Mention(FormatTypeMention::new(
                159..166,
                BlogInfo::new("t:_YENQUPzd_oPpmVDqZQ-yw"),
            )),
            FormatValue::Link(FormatTypeLink::new(
                257..282,
                url::Url::parse("https://guz.one").unwrap(),
            )),
        ];

        let npf_formatting = extrac_formatting!(&npf.content[0]);

        assert_eq_formatting!(formatting, npf_formatting);
        assert_eq_text!(
            &npf.content[0],
            "If you are reading this, thanks for giving a look \
            and checking the ugly source code of this little personal project. \
            It is heart warming to know that at least someone found this \
            interesting and maybe useful, even knowing how niched this \
            whole project is.\n\
            - Gustavo \"Guz\" L. de Mello, Apr 16, 12.2024"
        );
    }
}
