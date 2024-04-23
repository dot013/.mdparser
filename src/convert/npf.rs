pub mod content_blocks {
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

pub mod layout_blocks {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockValue {
        Rows(BlockRows),
        Ask(BlockAsk),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockRows {
        r#type: String,
        pub display: Vec<DisplayBlocks>,
        pub truncate_after: Option<u32>,
    }
    impl BlockRows {
        pub fn new(blocks: Vec<DisplayBlocks>) -> Self {
            Self::from(blocks)
        }
        fn default() -> Self {
            Self {
                r#type: String::from("rows"),
                display: vec![],
                truncate_after: None,
            }
        }
    }
    impl From<Vec<DisplayBlocks>> for BlockRows {
        fn from(value: Vec<DisplayBlocks>) -> Self {
            Self {
                display: value,
                ..Self::default()
            }
        }
    }
    impl From<DisplayBlocks> for BlockRows {
        fn from(value: DisplayBlocks) -> Self {
            Self {
                display: vec![value],
                ..Self::default()
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DisplayBlocks {
        pub blocks: Vec<u64>,
        pub mode: Option<String>,
    }
    impl DisplayBlocks {
        pub fn new(blocks: Vec<u64>) -> Self {
            Self::from(blocks)
        }
        fn default() -> Self {
            Self {
                blocks: vec![],
                mode: None,
            }
        }
    }
    impl From<Vec<u64>> for DisplayBlocks {
        fn from(value: Vec<u64>) -> Self {
            Self {
                blocks: value,
                ..Self::default()
            }
        }
    }
    impl From<u64> for DisplayBlocks {
        fn from(value: u64) -> Self {
            Self {
                blocks: vec![value],
                ..Self::default()
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct BlockAsk {
        r#type: String,
        blocks: Vec<u64>,
        attribution: Option<super::attributions::AttributionBlog>,
    }
    impl BlockAsk {
        pub fn new(blocks: Vec<u64>) -> Self {
            Self::from(blocks)
        }
        fn default() -> Self {
            Self {
                r#type: String::from("ask"),
                blocks: vec![],
                attribution: None,
            }
        }
    }
    impl From<Vec<u64>> for BlockAsk {
        fn from(value: Vec<u64>) -> Self {
            Self {
                blocks: value,
                ..Self::default()
            }
        }
    }
    impl From<u64> for BlockAsk {
        fn from(value: u64) -> Self {
            Self {
                blocks: vec![value],
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

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ReblogTrailPost {
        pub id: String,
        pub timestamp: Option<String>,
        pub is_commercial: Option<bool>,
    }
    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct ReblogTrail {
        pub post: Option<ReblogTrailPost>,
        pub blog: Option<BlogInfo>,
        pub content: Vec<super::content_blocks::BlockValue>,
        pub layout: Vec<super::layout_blocks::BlockValue>,
        pub broken_blog_name: Option<bool>,
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Avatar {
        pub width: u64,
        pub height: u64,
        pub url: url::Url,
        pub accessories: Vec<serde_json::Value>, // TODO: Find values for accessories
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Post {
        object_type: String,
        pub id: u64,
        pub id_string: String,
        pub r#type: Option<String>,
        pub tumblelog_uuid: Option<String>,
        pub original_type: Option<String>,
        pub is_blocks_post_format: Option<bool>,
        pub blog_name: Option<String>,
        pub blog: Option<BlogInfo>,
        pub is_blazed: Option<bool>,
        pub is_bale_pending: Option<bool>,
        pub can_ignite: Option<bool>,
        pub can_blaze: Option<bool>,
        pub post_url: Option<url::Url>,
        pub slug: Option<String>,
        pub date: Option<String>,
        pub timestamp: Option<String>,
        pub state: Option<String>,
        pub reblog_key: Option<String>,
        pub tags: Option<Vec<String>>,
        pub short_url: Option<String>,
        pub summary: Option<String>,
        pub should_open_in_legacy: Option<bool>,
        pub recommended_source: Option<String>,
        pub recommended_color: Option<String>,
        pub followed: Option<String>,
        pub post_author: Option<String>,
        pub author_blog: Option<BlogInfo>,
        pub post_author_avatar: Option<Avatar>,
        pub liked: Option<bool>,
        pub note_count: Option<u64>,
        pub content: Vec<super::content_blocks::BlockValue>,
        pub layout: Vec<super::layout_blocks::BlockValue>,
        pub trail: Vec<ReblogTrail>,
        pub can_line: Option<bool>,
        pub interactability_reblog: Option<String>,
        pub interactability_blaze: Option<String>,
        pub can_reblog: Option<bool>,
        pub can_send_in_message: Option<bool>,
        pub muted: Option<bool>,
        pub mute_end_timestamp: Option<u64>,
        pub can_mute: Option<bool>,
    }
    impl Post {
        pub fn new(id: u64) -> Self {
            Self::from(id)
        }
        pub fn is_valid(&self) -> bool {
            if let Ok(i) = self.id_string.parse::<u64>() {
                self.id == i
            } else {
                false
            }
        }
        fn default() -> Self {
            Self {
                object_type: String::from("post"),
                r#type: None,
                id: 0,
                id_string: String::from("0"),
                tumblelog_uuid: None,
                original_type: None,
                is_blocks_post_format: None,
                blog_name: None,
                blog: None,
                is_blazed: None,
                is_bale_pending: None,
                can_ignite: None,
                can_blaze: None,
                post_url: None,
                slug: None,
                date: None,
                timestamp: None,
                state: None,
                reblog_key: None,
                tags: None,
                short_url: None,
                summary: None,
                should_open_in_legacy: None,
                recommended_source: None,
                recommended_color: None,
                followed: None,
                post_author: None,
                author_blog: None,
                post_author_avatar: None,
                liked: None,
                note_count: None,
                content: vec![],
                layout: vec![],
                trail: vec![],
                can_line: None,
                interactability_reblog: None,
                interactability_blaze: None,
                can_reblog: None,
                can_send_in_message: None,
                muted: None,
                mute_end_timestamp: None,
                can_mute: None,
            }
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
