use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NPF<'a> {
    #[serde(borrow)]
    pub content: Vec<ContentType<'a>>,
}

impl<'a> NPF<'a> {
    pub fn new() -> NPF<'a> {
        NPF { content: vec![] }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ContentType<'a> {
    #[serde(borrow)]
    Text(content_types::Text<'a>),
    #[serde(borrow)]
    Link(content_types::Link<'a>),
    #[serde(borrow)]
    Audio(content_types::Audio<'a>),
    #[serde(borrow)]
    Image(content_types::Image<'a>),
    #[serde(borrow)]
    Video(content_types::Video<'a>),
}

pub mod content_types {
    use super::objects;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    pub mod text {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Deserialize, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub enum Subtypes {
            Heading1,
            Heading2,
            Quirky,
            Quote,
            Indented,
            Chat,
            OrderedListItem,
            UnorderedListItem,
        }

        #[serde_with::skip_serializing_none]
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct Formatting {
            pub start: usize,
            pub end: usize,
            pub r#type: FormattingType,
            pub url: Option<String>,
            pub blog: Option<super::objects::Blog>,
            pub color: Option<String>,
        }

        #[derive(Debug, Clone, Deserialize, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub enum FormattingType {
            Bold,
            Italic,
            Small,
            Strikethrough,
            Link,
            Mention,
        }
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Text<'a> {
        r#type: &'a str,
        pub text: String,
        pub subtype: Option<text::Subtypes>,
        pub indent_level: Option<u8>,
        pub formatting: Option<Vec<text::Formatting>>,
    }
    impl<'a> Text<'a> {
        pub fn new() -> Text<'a> {
            Text {
                r#type: "text",
                text: String::new(),
                subtype: None,
                indent_level: None,
                formatting: None,
            }
        }
        pub fn from(str: String) -> Text<'a> {
            Text {
                r#type: "text",
                text: str,
                subtype: None,
                indent_level: None,
                formatting: None,
            }
        }
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Image<'a> {
        r#type: &'a str,
        pub media: Vec<objects::Media>,
        pub alt_text: Option<String>,
        pub caption: Option<String>,
        pub feedback_token: Option<String>,
        pub colors: Option<HashMap<String, String>>,
        pub attribution: Option<objects::Attribution>,
    }
    impl<'a> Image<'a> {
        pub fn new() -> Image<'a> {
            Image {
                r#type: "image",
                media: vec![],
                alt_text: None,
                caption: None,
                feedback_token: None,
                colors: None,
                attribution: None,
            }
        }
        pub fn from(media: Vec<objects::Media>) -> Image<'a> {
            Image {
                media,
                r#type: "image",
                alt_text: None,
                caption: None,
                feedback_token: None,
                colors: None,
                attribution: None,
            }
        }
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Link<'a> {
        r#type: &'a str,
        pub url: String,
        pub title: Option<String>,
        pub description: Option<String>,
        pub author: Option<String>,
        pub site_name: Option<String>,
        pub display_url: Option<String>,
        pub poster: Option<Vec<objects::Media>>,
    }
    impl<'a> Link<'a> {
        pub fn new() -> Link<'a> {
            Link {
                r#type: "link",
                url: String::new(),
                title: None,
                description: None,
                author: None,
                site_name: None,
                display_url: None,
                poster: None,
            }
        }
        pub fn from(url: String) -> Link<'a> {
            Link {
                r#type: "link",
                url,
                title: None,
                description: None,
                author: None,
                site_name: None,
                display_url: None,
                poster: None,
            }
        }
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Audio<'a> {
        r#type: &'a str,
        pub url: Option<String>,
        pub media: Option<objects::Media>,
        pub provider: Option<String>,
        pub title: Option<String>,
        pub artist: Option<String>,
        pub album: Option<String>,
        pub poster: Option<Vec<objects::Media>>,
        pub embed_html: Option<String>,
        pub embed_url: Option<String>,
        pub metadata: Option<serde_json::Value>,
        pub attribution: Option<objects::Attribution>,
    }
    impl<'a> Audio<'a> {
        pub fn new() -> Audio<'a> {
            Audio {
                r#type: "audio",
                url: Some(String::new()),
                media: None,
                provider: None,
                title: None,
                artist: None,
                album: None,
                poster: None,
                embed_html: None,
                embed_url: None,
                metadata: None,
                attribution: None,
            }
        }
        pub fn from(url: String) -> Audio<'a> {
            Audio {
                r#type: "audio",
                url: Some(url),
                media: None,
                provider: None,
                title: None,
                artist: None,
                album: None,
                poster: None,
                embed_html: None,
                embed_url: None,
                metadata: None,
                attribution: None,
            }
        }
        pub fn from_media(media: objects::Media) -> Audio<'a> {
            Audio {
                r#type: "audio",
                url: None,
                media: Some(media),
                provider: None,
                title: None,
                artist: None,
                album: None,
                poster: None,
                embed_html: None,
                embed_url: None,
                metadata: None,
                attribution: None,
            }
        }
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Video<'a> {
        r#type: &'a str,
        pub media: Option<objects::Media>,
        pub url: Option<String>,
        pub provider: Option<String>,
        pub embed_html: Option<String>,
        pub embed_iframe: Option<String>,
        pub embed_url: Option<String>,
        pub poster: Option<Vec<objects::Media>>,
        pub filmstrip: Option<Vec<objects::Media>>,
        pub metadata: Option<serde_json::Value>,
        pub attribution: Option<objects::Attribution>,
        pub can_autoplay_on_cellular: Option<bool>,
    }
    impl<'a> Video<'a> {
        pub fn new() -> Video<'a> {
            Video {
                r#type: "video",
                url: Some(String::new()),
                media: None,
                provider: None,
                embed_html: None,
                embed_iframe: None,
                embed_url: None,
                poster: None,
                filmstrip: None,
                metadata: None,
                attribution: None,
                can_autoplay_on_cellular: None,
            }
        }
        pub fn from(url: String) -> Video<'a> {
            Video {
                r#type: "video",
                url: Some(url),
                media: None,
                provider: None,
                embed_html: None,
                embed_iframe: None,
                embed_url: None,
                poster: None,
                filmstrip: None,
                metadata: None,
                attribution: None,
                can_autoplay_on_cellular: None,
            }
        }
        pub fn from_media(media: objects::Media) -> Video<'a> {
            Video {
                r#type: "video",
                url: None,
                media: Some(media),
                provider: None,
                embed_html: None,
                embed_iframe: None,
                embed_url: None,
                poster: None,
                filmstrip: None,
                metadata: None,
                attribution: None,
                can_autoplay_on_cellular: None,
            }
        }
    }

    // TODO: Paywall type
}
pub mod objects {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Blog {
        pub uuid: String,
        pub name: String,
        pub url: String,
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Media {
        pub r#type: String,
        pub url: String,
        pub width: i32,
        pub height: i32,
        pub poster: Option<MediaPoster>,
    }

    #[serde_with::skip_serializing_none]
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct MediaPoster {
        pub r#type: String,
        pub url: String,
        pub width: Option<String>,
        pub height: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Attribution {
        pub r#type: String,
        pub url: String,
        pub post: Post,
        pub blug: Blog,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Post {
        pub id: u64,
    }
}
