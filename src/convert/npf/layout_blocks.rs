use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum BlockValue {
    Rows(BlockRows),
    Ask(BlockAsk),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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
