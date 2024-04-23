use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NPF {
    pub content: Vec<blocks::BlockValue>,
}

pub mod blocks {
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
}

pub mod text_formatting {
    use std::{ops::Range, str::FromStr};

    use serde::{Deserialize, Serialize};

    use super::objects;

    #[derive(Debug, Deserialize, Serialize)]
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

    trait FormatType: From<Range<u64>> + From<String> {
        fn default() -> Self;
    }

    #[derive(Debug, Deserialize, Serialize)]
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

    #[derive(Debug, Deserialize, Serialize)]
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

    #[derive(Debug, Deserialize, Serialize)]
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
            $(impl From<&str> for $t {
                fn from(value: &str) -> Self {
                    Self {
                        start: 0,
                        end: value.chars().count() as u64,
                        ..Self::default()
                    }
                }
            })*
        };
        // Defines the struct and implements Default trait if the token is an
        // identifier and a literal
        (for $($t:ident $s:literal),+) => {
            $(#[derive(Debug, Deserialize, Serialize)]
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
}

pub mod objects {
    use serde::{Deserialize, Serialize};

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize)]
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
            !self.uuid.is_empty()
                || !self.uuid.chars().count() == 22
                || !self.uuid.starts_with("t:")
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Post {
        pub id: u64,
    }
    impl Post {
        pub fn new(id: u64) -> Self {
            Self::from(id)
        }
    }
    impl Default for Post {
        fn default() -> Self {
            Self { id: 0 }
        }
    }
    impl From<u64> for Post {
        fn from(value: u64) -> Self {
            Self {
                id: value,
                id_string: value.to_string(),
                ..Self::default()
            }
        }
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize)]
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

    #[derive(Debug, Deserialize, Serialize)]
    pub struct EmbedIframe {
        pub url: url::Url,
        pub width: u64,
        pub height: u64,
    }
}

pub mod attributions {
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};

    use super::objects;

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum AttributionValue {
        Post(AttributionPost),
        Link(AttributionLink),
        Blog(AttributionBlog),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AttributionPost {
        r#type: String,
        pub url: url::Url,
        pub post: objects::Post,
        pub blog: objects::BlogInfo,
    }
    impl AttributionPost {
        pub fn new(url: url::Url, post: objects::Post, blog: objects::BlogInfo) -> Self {
            Self {
                url,
                post,
                blog,
                ..Self::default()
            }
        }
        pub fn is_valid(&self) -> bool {
            self.blog.is_valid() && self.post.is_valid()
        }
        fn default() -> Self {
            Self {
                r#type: String::from("post"),
                url: url::Url::from_str("https://tumblr.com").unwrap(),
                post: objects::Post::new(0),
                blog: objects::BlogInfo::new("t:0"),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AttributionLink {
        r#type: String,
        pub url: url::Url,
    }
    impl AttributionLink {
        pub fn new(url: url::Url) -> Self {
            Self::from(url)
        }
        fn default() -> Self {
            Self {
                r#type: String::from("link"),
                url: url::Url::from_str("https://tumblr.com").unwrap(),
            }
        }
    }
    impl From<url::Url> for AttributionLink {
        fn from(value: url::Url) -> Self {
            Self {
                url: value,
                ..Self::default()
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AttributionBlog {
        r#type: String,
        pub url: Option<url::Url>,
        pub blog: objects::BlogInfo,
    }
    impl AttributionBlog {
        pub fn new(blog: objects::BlogInfo) -> Self {
            Self::from(blog)
        }
        pub fn is_valid(&self) -> bool {
            self.blog.is_valid()
        }
        fn default() -> Self {
            Self {
                r#type: String::from("blog"),
                url: None,
                blog: objects::BlogInfo::new("t:0"),
            }
        }
    }
    impl From<objects::BlogInfo> for AttributionBlog {
        fn from(value: objects::BlogInfo) -> Self {
            Self {
                blog: value,
                ..Self::default()
            }
        }
    }
    /* TODO: Add TryFrom which checks if given BlogInfo is valid
    impl TryFrom<objects::BlogInfo> for AttributionBlog {
        fn try_from(value: objects::BlogInfo) -> Result<Self, Self::Error> {
            todo!()
        }
    }
    */

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AttributionApp {
        r#type: String,
        pub url: url::Url,
        pub app_name: Option<String>,
        pub display_text: Option<String>,
        pub logo: Option<objects::Media>,
    }
    impl AttributionApp {
        pub fn new(url: url::Url) -> Self {
            Self::from(url)
        }
        fn default() -> Self {
            Self {
                r#type: String::from("blog"),
                url: url::Url::from_str("https://tumblr.com").unwrap(),
                app_name: None,
                display_text: None,
                logo: None,
            }
        }
    }
    impl From<url::Url> for AttributionApp {
        fn from(value: url::Url) -> Self {
            Self {
                url: value,
                ..Self::default()
            }
        }
    }
}
