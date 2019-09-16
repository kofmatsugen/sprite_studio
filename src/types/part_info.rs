use crate::types::{animation_ref::RefferenceAnimation, bound_type::Bounds, part_type::PartType};
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartInfo {
    id: usize,
    parent: Option<usize>,
    part_type: PartType,
    bounds: Option<Bounds>,
    animation_ref: Option<RefferenceAnimation>,
}

impl PartInfo {
    pub fn part_id(&self) -> usize {
        self.id
    }

    pub fn parent_id(&self) -> Option<usize> {
        self.parent
    }

    pub fn part_type(&self) -> &PartType {
        &self.part_type
    }

    pub fn bounds(&self) -> Option<&Bounds> {
        self.bounds.as_ref()
    }

    pub fn refference_animation(&self) -> Option<&RefferenceAnimation> {
        self.animation_ref.as_ref()
    }
}

#[derive(Default)]
pub(crate) struct PartInfoBuilder {
    id: Option<usize>,
    parent: Option<usize>,
    part_type: Option<PartType>,
    bounds: Option<Bounds>,
    ref_pack_id: Option<usize>,
    ref_anim_id: Option<usize>,
}

impl PartInfoBuilder {
    pub(crate) fn id(&mut self, id: usize) {
        self.id = id.into();
    }

    pub(crate) fn parent(&mut self, parent: impl Into<Option<usize>>) {
        self.parent = parent.into();
    }

    pub(crate) fn part_type(&mut self, part_type: PartType) {
        self.part_type = part_type.into();
    }

    pub(crate) fn bounds(&mut self, bounds: impl Into<Option<Bounds>>) {
        self.bounds = bounds.into();
    }

    pub(crate) fn pack_id(&mut self, pack_id: impl Into<Option<usize>>) {
        self.ref_pack_id = pack_id.into();
    }

    pub(crate) fn animation_id(&mut self, animation_id: impl Into<Option<usize>>) {
        self.ref_anim_id = animation_id.into();
    }

    pub(crate) fn build(self) -> PartInfo {
        let animation_ref = match (self.ref_pack_id, self.ref_anim_id) {
            (Some(pack), Some(anim)) => RefferenceAnimation::new(pack, anim).into(),
            _ => None,
        };

        PartInfo {
            id: self.id.expect("not set part id"),
            parent: self.parent,
            part_type: self.part_type.expect("not set part type"),
            bounds: self.bounds,
            animation_ref,
        }
    }
}
