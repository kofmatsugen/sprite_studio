mod bounds;
mod part_type;

pub use bounds::*;
pub use part_type::*;
use serde::*;

#[derive(Serialize, Deserialize)]
pub struct Part {
    name: String,
    parent_id: Option<u32>,
    part_type: PartType,
    bounds: BoundsType,
}

impl Part {
    pub fn new<N>(name: N, part_type: PartType, bounds: BoundsType, parent: i32) -> Self
    where
        N: Into<String>,
    {
        let name = name.into();
        let parent_id = if parent < 0 {
            None
        } else {
            Some(parent as u32)
        };
        Part {
            name,
            parent_id,
            part_type,
            bounds,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn parent(&self) -> Option<u32> {
        self.parent_id
    }

    pub fn part_type(&self) -> &PartType {
        &self.part_type
    }

    pub fn bounds(&self) -> &BoundsType {
        &self.bounds
    }
}
