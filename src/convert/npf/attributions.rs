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
