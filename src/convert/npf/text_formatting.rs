use std::{ops::Range, str::FromStr};

use serde::{Deserialize, Serialize};

use super::objects;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum FormatValue {
    Bold(FormatTypeBold),
    Italic(FormatTypeItalic),
    StrikeThrough(FormatTypeStrikeThrough),
    Small(FormatTypeSmall),
    Link(FormatTypeLink),
    Mention(FormatTypeMention),
    Color(FormatTypeColor),
}
impl FormatValue {
    pub fn offset(&mut self, offset: u64) {
        match self {
            FormatValue::Bold(ref mut f) => f.offset(offset),
            FormatValue::Italic(ref mut f) => f.offset(offset),
            FormatValue::Link(ref mut f) => f.offset(offset),
            FormatValue::Small(ref mut f) => f.offset(offset),
            FormatValue::Color(ref mut f) => f.offset(offset),
            FormatValue::Mention(ref mut f) => f.offset(offset),
            FormatValue::StrikeThrough(ref mut f) => f.offset(offset),
        }
    }
}

trait FormatType: From<Range<u64>> + From<String> {
    fn default() -> Self;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormatTypeLink {
    r#type: String,
    pub start: u64,
    pub end: u64,
    pub url: url::Url,
}
impl FormatTypeLink {
    pub fn new(range: Range<u64>, url: url::Url) -> Self {
        Self {
            url,
            ..Self::from(range)
        }
    }
}
impl FormatType for FormatTypeLink {
    fn default() -> Self {
        Self {
            r#type: String::from("link"),
            start: 0,
            end: 0,
            url: url::Url::from_str("https://tumblr.com").unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormatTypeMention {
    r#type: String,
    pub start: u64,
    pub end: u64,
    pub blog: objects::BlogInfo,
}
impl FormatTypeMention {
    pub fn new(range: Range<u64>, blog: objects::BlogInfo) -> Self {
        Self {
            blog,
            ..Self::from(range)
        }
    }
}
impl FormatType for FormatTypeMention {
    fn default() -> Self {
        Self {
            r#type: String::from("mention"),
            start: 0,
            end: 0,
            blog: objects::BlogInfo::new("t:0"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormatTypeColor {
    r#type: String,
    pub start: u64,
    pub end: u64,
    pub hex: String,
}
impl FormatTypeColor {
    pub fn new(range: Range<u64>, hex: color_art::Color) -> Self {
        Self {
            hex: hex.hex(),
            ..Self::from(range)
        }
    }
}
impl FormatType for FormatTypeColor {
    fn default() -> Self {
        Self {
            r#type: String::from("link"),
            start: 0,
            end: 0,
            hex: String::from("#ffffff"),
        }
    }
}

macro_rules! ImplInlines {
        // If passed token is already defined (is a type), implements
        // the FormattingInline traits
        (for $($t:ty),+) => {
            $(impl From<Range<u64>> for $t {
                fn from(value: Range<u64>) -> Self {
                    Self {
                        start: value.start,
                        end: value.end,
                        ..Self::default()
                    }
                }
            })*
            $(impl From<String> for $t {
                fn from(value: String) -> Self {
                    Self {
                        start: 0,
                        end: value.chars().count() as u64,
                        ..Self::default()
                    }
                }
            })*
            $(impl From<&String> for $t {
                fn from(value: &String) -> Self {
                    Self {
                        start: 0,
                        end: value.chars().count() as u64,
                        ..Self::default()
                    }
                }
            })*
            $(impl From<&str> for $t {
                fn from(value: &str) -> Self {
                    Self {
                        start: 0,
                        end: value.chars().count() as u64,
                        ..Self::default()
                    }
                }
            })*
            $(impl $t {
                pub fn offset(&mut self, offset: u64) {
                    self.start += offset;
                    self.end += offset;
                }
            })*
        };
        // Defines the struct and implements Default trait if the token is an
        // identifier and a literal
        (for $($t:ident $s:literal),+) => {
            $(#[derive(Debug, Deserialize, Serialize, Clone)]
            pub struct $t {
                r#type: String,
                pub start: u64,
                pub end: u64,
            })*
            $(impl FormatType for $t {
                fn default() -> Self {
                    Self {
                        r#type: String::from($s),
                        start: 0,
                        end: 0,
                    }
                }
            })*
            $(ImplInlines!(for $t);)*
        }
    }
ImplInlines!(for
    FormatTypeBold "bold",
    FormatTypeItalic "italic",
    FormatTypeStrikeThrough "strikethrough",
    FormatTypeSmall "small"
);
ImplInlines!(for
    FormatTypeLink,
    FormatTypeMention,
    FormatTypeColor
);
