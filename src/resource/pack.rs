use super::{animation::Animation, part::Part};
use crate::traits::AnimationKey;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack<U, P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    #[serde(bound(deserialize = "Vec<Part<P, A>>: Deserialize<'de>"))]
    parts: Vec<Part<P, A>>,
    #[serde(bound(deserialize = "BTreeMap<A, Animation<U>>: Deserialize<'de>"))]
    animations: BTreeMap<A, Animation<U>>,
}

impl<U, P, A> Pack<U, P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    pub fn parts(&self) -> impl Iterator<Item = &Part<P, A>> {
        self.parts.iter()
    }

    pub fn animation(&self, animation: &A) -> Option<&Animation<U>> {
        self.animations.get(animation)
    }
}

#[cfg(feature = "builder")]
pub struct PackBuilder<U, P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    parts: Vec<Part<P, A>>,
    animations: BTreeMap<A, Animation<U>>,
}

#[cfg(feature = "builder")]
impl<U, P, A> PackBuilder<U, P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    pub fn new(parts: Vec<Part<P, A>>, animations: BTreeMap<A, Animation<U>>) -> Self {
        PackBuilder { parts, animations }
    }

    pub fn build(self) -> Pack<U, P, A> {
        Pack {
            parts: self.parts,
            animations: self.animations,
        }
    }
}

#[cfg(feature = "debug")]
impl<U, P, A> std::ops::Drop for Pack<U, P, A>
where
    P: AnimationKey,
    A: AnimationKey,
{
    fn drop(&mut self) {
        log::debug!(
            "drop Pack: {:?}",
            self.parts().map(|p| p.name()).collect::<Vec<_>>()
        );
    }
}
