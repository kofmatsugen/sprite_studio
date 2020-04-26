use super::name::AnimationName;
use crate::{
    traits::AnimationKey,
    types::{bound_type::Bounds, part_type::PartType},
};
use serde::{Deserialize, Serialize};

//----------------------------------------------------
// アニメーションのパーツ情報
#[derive(Debug, Serialize, Deserialize)]
pub struct Part<P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<u32>,
    #[serde(
        bound(deserialize = "Option<AnimationName<P, A>>: Deserialize<'de>"),
        skip_serializing_if = "Option::is_none"
    )]
    refference_animation_name: Option<AnimationName<P, A>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refference_effect_index: Option<usize>,
    part_type: PartType,
    #[serde(skip_serializing_if = "Option::is_none")]
    bounds: Option<Bounds>,
}

impl<P, A> Part<P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent_id(&self) -> Option<u32> {
        self.parent_id
    }

    pub fn refference_animation_name(&self) -> Option<&AnimationName<P, A>> {
        self.refference_animation_name.as_ref()
    }
}

//----------------------------------------------------
// パーツの情報は取得のみしか許可しないためにビルダーパターン
#[cfg(feature = "builder")]
pub struct PartBuilder<P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    name: String,
    parent_id: Option<u32>,
    refference_animation_name: Option<AnimationName<P, A>>,
    refference_effect_index: Option<usize>,
    part_type: PartType,
    bounds: Option<Bounds>,
}

#[cfg(feature = "builder")]
impl<P, A> PartBuilder<P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    pub fn new<S: Into<String>>(name: S, part_type: PartType) -> Self {
        PartBuilder {
            name: name.into(),
            parent_id: None,
            refference_animation_name: None,
            refference_effect_index: None,
            part_type,
            bounds: None,
        }
    }

    pub fn parent_id(mut self, parent_id: u32) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn refference_animation_name(mut self, pack_name: P, anim_name: A) -> Self {
        self.refference_animation_name = Some(AnimationName::FullName {
            pack: pack_name,
            animation: anim_name,
        });
        self
    }

    pub fn refference_effect_index(mut self, index: usize) -> Self {
        self.refference_effect_index = Some(index);
        self
    }

    pub fn bounds(mut self, bounds: Option<Bounds>) -> Self {
        self.bounds = bounds;
        self
    }

    pub fn build(self) -> Part<P, A> {
        Part {
            name: self.name,
            parent_id: self.parent_id,
            refference_animation_name: self.refference_animation_name,
            refference_effect_index: self.refference_effect_index,
            part_type: self.part_type,
            bounds: self.bounds,
        }
    }
}
