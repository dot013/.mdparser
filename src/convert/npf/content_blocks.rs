use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{attributions, objects};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockValue {
    Text(BlockText),
    Image(BlockImage),
    Link(BlockLink),
    Audio(BlockAudio),
    Video(BlockVideo),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockTextSubtype {
    Heading1,
    Heading2,
    Quirky,
    Quote,
    Indented,
    Chat,
    OrderedListItem,
    UnordoredListItem,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockText {
    r#type: String,
    pub subtype: Option<BlockTextSubtype>,
    pub text: String,
    pub formatting: Option<Vec<super::text_formatting::FormatValue>>,
    pub ident_level: Option<u8>,
}
impl BlockText {
    pub fn new(value: &str) -> Self {
        Self::from(value)
    }
    fn default() -> Self {
        Self {
            r#type: String::from("text"),
            subtype: None,
            text: String::new(),
            formatting: None,
            ident_level: None,
        }
    }
}
impl From<String> for BlockText {
    fn from(value: String) -> Self {
        Self {
            text: value,
            ..Self::default()
        }
    }
}
impl From<&str> for BlockText {
    fn from(value: &str) -> Self {
        Self {
            text: String::from(value),
            ..Self::default()
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockImage {
    r#type: String,
    pub media: Vec<objects::Media>,
    pub colors: Option<HashMap<String, String>>,
    pub feedback_token: Option<String>,
    pub poster: Option<objects::Media>,
    pub attribution: Option<attributions::AttributionValue>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
}
impl BlockImage {
    pub fn new(media: Vec<objects::Media>) -> Self {
        Self::from(media)
    }
    fn default() -> Self {
        Self {
            r#type: String::from("image"),
            media: vec![],
            colors: None,
            feedback_token: None,
            poster: None,
            attribution: None,
            alt_text: None,
            caption: None,
        }
    }
}
impl From<Vec<objects::Media>> for BlockImage {
    fn from(value: Vec<objects::Media>) -> Self {
        Self {
            media: value,
            ..Self::default()
        }
    }
}
impl From<objects::Media> for BlockImage {
    fn from(value: objects::Media) -> Self {
        Self::from(vec![value])
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockLink {
    r#type: String,
    pub url: url::Url,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub site_name: Option<String>,
    pub display_url: Option<url::Url>,
    pub poster: Option<objects::Media>,
}
impl BlockLink {
    pub fn new(url: url::Url) -> Self {
        Self::from(url)
    }
    fn default() -> Self {
        Self {
            r#type: String::from("link"),
            url: url::Url::from_str("https://tumblr.com").unwrap(),
            title: None,
            description: None,
            author: None,
            site_name: None,
            display_url: None,
            poster: None,
        }
    }
}
impl From<url::Url> for BlockLink {
    fn from(value: url::Url) -> Self {
        Self {
            url: value,
            ..Self::default()
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockAudio {
    r#type: String,
    pub media: Option<objects::Media>,
    pub url: Option<url::Url>,
    pub provider: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub poster: Option<objects::Media>,
    pub embed_html: Option<String>,
    pub embed_iframe: Option<objects::EmbedIframe>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub attribution: Option<attributions::AttributionValue>,
}
impl BlockAudio {
    pub fn new(url: url::Url) -> Self {
        Self::from(url)
    }
    pub fn is_valid(&self) -> bool {
        if self.url.is_some() || self.media.is_some() {
            true
        } else {
            false
        }
    }
    fn default() -> Self {
        Self {
            r#type: String::from("audio"),
            media: None,
            url: None,
            provider: None,
            title: None,
            artist: None,
            album: None,
            poster: None,
            embed_html: None,
            embed_iframe: None,
            metadata: None,
            attribution: None,
        }
    }
}
impl From<objects::Media> for BlockAudio {
    fn from(value: objects::Media) -> Self {
        Self {
            media: Some(value),
            ..Self::default()
        }
    }
}
impl From<url::Url> for BlockAudio {
    fn from(value: url::Url) -> Self {
        Self {
            url: Some(value),
            ..Self::default()
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockVideo {
    r#type: String,
    pub url: Option<url::Url>,
    pub media: Option<objects::Media>,
    pub provider: Option<String>,
    pub embed_html: Option<String>,
    pub embed_iframe: Option<objects::EmbedIframe>,
    pub embed_url: Option<url::Url>,
    pub poster: Option<objects::Media>,
    pub filmstrip: Option<objects::Media>,
    pub attribution: Option<attributions::AttributionValue>,
    pub can_autoplay_on_cellular: Option<bool>,
}
impl BlockVideo {
    pub fn new(url: url::Url) -> Self {
        Self::from(url)
    }
    pub fn is_valid(&self) -> bool {
        if self.url.is_some() || self.media.is_some() {
            true
        } else {
            false
        }
    }
    fn default() -> Self {
        Self {
            r#type: String::from("audio"),
            media: None,
            url: None,
            provider: None,
            embed_html: None,
            embed_iframe: None,
            embed_url: None,
            poster: None,
            filmstrip: None,
            attribution: None,
            can_autoplay_on_cellular: None,
        }
    }
}
impl From<objects::Media> for BlockVideo {
    fn from(value: objects::Media) -> Self {
        Self {
            media: Some(value),
            ..Self::default()
        }
    }
}
impl From<url::Url> for BlockVideo {
    fn from(value: url::Url) -> Self {
        Self {
            url: Some(value),
            ..Self::default()
        }
    }
}
