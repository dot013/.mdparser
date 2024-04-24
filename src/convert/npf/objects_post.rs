use serde::{Deserialize, Serialize};
#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    object_type: String,
    pub id: u64,
    pub id_string: String,
    pub r#type: Option<String>,
    pub tumblelog_uuid: Option<String>,
    pub original_type: Option<String>,
    pub is_blocks_post_format: Option<bool>,
    pub blog_name: Option<String>,
    pub blog: Option<super::objects::BlogInfo>,
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
    pub author_blog: Option<super::objects::BlogInfo>,
    pub post_author_avatar: Option<super::objects::Avatar>,
    pub liked: Option<bool>,
    pub note_count: Option<u64>,
    pub content: Vec<super::content_blocks::BlockValue>,
    pub layout: Vec<super::layout_blocks::BlockValue>,
    pub trail: Vec<super::objects::ReblogTrail>,
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

