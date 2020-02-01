use super::{animation::Animation, part::Part};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack<U, A>
where
    A: Ord + std::hash::Hash + Serialize,
{
    parts: Vec<Part>,
    #[serde(bound(deserialize = "BTreeMap<A, Animation<U>>: Deserialize<'de>"))]
    animations: BTreeMap<A, Animation<U>>,
}

impl<U, A> Pack<U, A>
where
    A: Ord + std::hash::Hash + Serialize,
{
    pub fn parts(&self) -> impl Iterator<Item = &Part> {
        self.parts.iter()
    }

    pub fn animation(&self, animation: &A) -> Option<&Animation<U>> {
        self.animations.get(animation)
    }
}

pub struct PackBuilder<U, A>
where
    A: Ord + std::hash::Hash + Serialize,
{
    parts: Vec<Part>,
    animations: BTreeMap<A, Animation<U>>,
}

impl<U, A> PackBuilder<U, A>
where
    A: Ord + std::hash::Hash + Serialize,
{
    pub fn new(parts: Vec<Part>, animations: BTreeMap<A, Animation<U>>) -> Self {
        PackBuilder { parts, animations }
    }

    pub fn build(self) -> Pack<U, A> {
        Pack {
            parts: self.parts,
            animations: self.animations,
        }
    }
}
