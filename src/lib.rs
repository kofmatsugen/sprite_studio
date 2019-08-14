mod part;
use amethyst::animation::AnimationHierarchyPrefab;
use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::core::Transform;
use amethyst::derive::*;
use amethyst::ecs::Entity;
use amethyst::error::Error;
use amethyst::renderer::{
    sprite::prefab::{SpriteRenderPrefab, SpriteSheetPrefab},
    SpriteRender,
};
pub use part::*;
use serde::*;
use rand::Rng;

#[derive(Serialize, Deserialize, PrefabData)]
pub struct SpriteAnimation {
    sprite_hierarchy: Option<AnimationHierarchyPrefab<SpriteRender>>,
    transform_hierarchy: Option<AnimationHierarchyPrefab<Transform>>,
    cells: Option<SpriteSheetPrefab>,
    render: Option<SpriteRenderPrefab>,
    transform: Transform,
}

impl SpriteAnimation {
    pub fn new(cells: Option<SpriteSheetPrefab>, render: Option<SpriteRenderPrefab>) -> Self {
        let mut transform = Transform::default();
        let mut range = rand::thread_rng();
        let x: f32 = range.gen_range(-100., 100.);
        let y: f32 = range.gen_range(-100., 100.);
        transform.set_translation_xyz(x, y, 1.0);
        SpriteAnimation {
            sprite_hierarchy: None,
            transform_hierarchy: None,
            cells,
            render,
            transform,
        }
    }
}
