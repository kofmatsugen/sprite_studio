use serde::{Deserialize, Serialize};

//----------------------------------------------------
// アニメーション情報の取得キー
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum AnimationName {
    FullName {
        pack: String,
        animation: String,
    },
    PackName(String),
    AnimName(String),
    WithPartId {
        pack: String,
        animation: String,
        part_id: u32,
    },
}
