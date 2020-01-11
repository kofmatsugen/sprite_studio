use super::{animation::Animation, part::Part};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack<U> {
    parts: Vec<Part>,
    animations: BTreeMap<String, Animation<U>>,
}

impl<U> Pack<U> {
    pub fn parts(&self) -> impl Iterator<Item = &Part> {
        self.parts.iter()
    }

    pub fn animation(&self, animation_name: &str) -> Option<&Animation<U>> {
        self.animations.get(animation_name)
    }
}

pub struct PackBuilder<U> {
    parts: Vec<Part>,
    animations: BTreeMap<String, Animation<U>>,
}

impl<U> PackBuilder<U> {
    pub fn new(parts: Vec<Part>, animations: BTreeMap<String, Animation<U>>) -> Self {
        PackBuilder { parts, animations }
    }

    pub fn build(self) -> Pack<U> {
        Pack {
            parts: self.parts,
            animations: self.animations,
        }
    }
}
