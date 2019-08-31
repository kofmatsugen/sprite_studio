use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct AnimationPart(pub usize);

impl Component for AnimationPart {
    type Storage = DenseVecStorage<Self>;
}
