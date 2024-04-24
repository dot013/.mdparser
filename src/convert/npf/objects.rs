use serde::{Deserialize, Serialize};

pub use super::objects_post::Post;

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlogInfo {
    pub uuid: String,
    pub name: Option<String>,
    pub url: Option<url::Url>,
}
impl BlogInfo {
    pub fn new(uuid: &str) -> Self {
        Self::from(uuid)
    }
    pub fn is_valid(&self) -> bool {
        !self.uuid.is_empty() || !self.uuid.chars().count() == 22 || !self.uuid.starts_with("t:")
    }
    fn default() -> Self {
        Self {
            uuid: String::new(),
            name: None,
            url: None,
        }
    }
}
impl From<String> for BlogInfo {
    fn from(value: String) -> Self {
        Self {
            uuid: value,
            ..Self::default()
        }
    }
}
impl From<&str> for BlogInfo {
    fn from(value: &str) -> Self {
        Self {
            uuid: String::from(value),
            ..Self::default()
        }
    }
}
impl From<url::Url> for BlogInfo {
    fn from(value: url::Url) -> Self {
        Self {
            url: Some(value),
            ..Self::default()
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReblogTrailPost {
    pub id: String,
    pub timestamp: Option<String>,
    pub is_commercial: Option<bool>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReblogTrail {
    pub post: Option<ReblogTrailPost>,
    pub blog: Option<BlogInfo>,
    pub content: Vec<super::content_blocks::BlockValue>,
    pub layout: Vec<super::layout_blocks::BlockValue>,
    pub broken_blog_name: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Avatar {
    pub width: u64,
    pub height: u64,
    pub url: url::Url,
    pub accessories: Vec<serde_json::Value>, // TODO: Find values for accessories
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Media {
    pub r#type: Option<String>,
    pub url: url::Url,
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub original_dimensions_missing: Option<bool>,
    pub cropped: Option<bool>,
    pub has_original_dimentions: Option<bool>,
}
impl Media {
    pub fn new(url: url::Url) -> Self {
        Self::from(url)
    }
    fn default() -> Self {
        Self {
            r#type: None,
            url: url::Url::parse("https://tumblr.com").unwrap(),
            width: None,
            height: None,
            original_dimensions_missing: None,
            cropped: None,
            has_original_dimentions: None,
        }
    }
}
impl From<url::Url> for Media {
    fn from(value: url::Url) -> Self {
        Self {
            url: value,
            ..Self::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedIframe {
    pub url: url::Url,
    pub width: u64,
    pub height: u64,
}
