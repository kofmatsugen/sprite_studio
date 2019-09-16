use serde::*;

// 参照してるアニメーション用のID
#[derive(Serialize, Deserialize, Debug)]
pub struct RefferenceAnimation {
    pack_id: usize,
    animation_id: usize,
}

impl RefferenceAnimation {
    pub fn new(pack_id: usize, animation_id: usize) -> Self {
        RefferenceAnimation {
            pack_id,
            animation_id,
        }
    }

    pub fn pack_id(&self) -> usize {
        self.pack_id
    }

    pub fn animation_id(&self) -> usize {
        self.animation_id
    }
}
