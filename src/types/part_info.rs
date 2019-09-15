use crate::types::{bound_type::Bounds, part_type::PartType};
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartInfo {
    id: usize,
    parent: Option<usize>,
    part_type: PartType,
    bounds: Option<Bounds>,
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
}

#[derive(Default)]
pub(crate) struct PartInfoBuilder {
    id: Option<usize>,
    parent: Option<usize>,
    part_type: Option<PartType>,
    bounds: Option<Bounds>,
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

    pub(crate) fn build(self) -> PartInfo {
        PartInfo {
            id: self.id.expect("not set part id"),
            parent: self.parent,
            part_type: self.part_type.expect("not set part type"),
            bounds: self.bounds,
        }
    }
}
