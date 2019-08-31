use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationRoot(pub Entity);

impl Component for AnimationRoot {
    type Storage = DenseVecStorage<Self>;
}
