use super::name::AnimationName;
use crate::types::{bound_type::Bounds, part_type::PartType};
use serde::{Deserialize, Serialize};

//----------------------------------------------------
// アニメーションのパーツ情報
#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    name: String,
    parent_id: Option<u32>,
    refference_animation_name: Option<AnimationName>,
    part_type: PartType,
    bounds: Option<Bounds>,
}

impl Part {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent_id(&self) -> Option<u32> {
        self.parent_id
    }

    pub fn refference_animation_name(&self) -> Option<&AnimationName> {
        self.refference_animation_name.as_ref()
    }
}

//----------------------------------------------------
// パーツの情報は取得のみしか許可しないためにビルダーパターン
pub struct PartBuilder {
    name: String,
    parent_id: Option<u32>,
    refference_animation_name: Option<AnimationName>,
    part_type: PartType,
    bounds: Option<Bounds>,
}

impl PartBuilder {
    pub fn new<S: Into<String>>(name: S, part_type: PartType) -> Self {
        PartBuilder {
            name: name.into(),
            parent_id: None,
            refference_animation_name: None,
            part_type,
            bounds: None,
        }
    }

    pub fn parent_id(mut self, parent_id: u32) -> Self {
        self.parent_id = Some(parent_id);
        self
    }
    pub fn refference_animation_name<S: Into<String>>(
        mut self,
        pack_name: S,
        anim_name: S,
    ) -> Self {
        self.refference_animation_name = Some(AnimationName::FullName {
            pack: pack_name.into(),
            animation: anim_name.into(),
        });
        self
    }

    pub fn bounds(mut self, bounds: Option<Bounds>) -> Self {
        self.bounds = bounds;
        self
    }

    pub fn build(self) -> Part {
        Part {
            name: self.name,
            parent_id: self.parent_id,
            refference_animation_name: self.refference_animation_name,
            part_type: self.part_type,
            bounds: self.bounds,
        }
    }
}
