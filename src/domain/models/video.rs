use serde::Deserialize;

// deriveは、structを拡張するためのものです。
#[derive(PartialEq, Clone, Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}
