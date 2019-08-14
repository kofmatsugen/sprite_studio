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

#[derive(Serialize, Deserialize, PrefabData)]
pub struct SpriteAnimation {
    sprite_hierarchy: Option<AnimationHierarchyPrefab<SpriteRender>>,
    transform_hierarchy: Option<AnimationHierarchyPrefab<Transform>>,
    cells: Option<SpriteSheetPrefab>,
    render: Option<SpriteRenderPrefab>,
    transform: Transform,
}

impl SpriteAnimation {
    pub fn new(
        cells: Option<SpriteSheetPrefab>,
        render: Option<SpriteRenderPrefab>,
        transform: Transform,
    ) -> Self {
        SpriteAnimation {
            sprite_hierarchy: None,
            transform_hierarchy: None,
            cells,
            render,
            transform,
        }
    }

    pub fn set_transform_hierarchy(&mut self, transform_hierarchy: AnimationHierarchyPrefab<Transform>){
        self.transform_hierarchy = transform_hierarchy.into();
    }
}
