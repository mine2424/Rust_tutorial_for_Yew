use serde::Deserialize;

// deriveは、structを拡張するためのものです。
// trait(like interface)を複数指定できる！！
#[derive(PartialEq, Clone, Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}
