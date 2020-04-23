use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PartType {
    Null,
    Normal,
    Text,
    Instance,

    // ↓アニメーション的に複雑なので余裕のあるときに対応
    Mesh,
    Bone,
    Joint,
    Armature,
}
