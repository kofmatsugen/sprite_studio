mod part;
use amethyst::animation::{AnimationHierarchyPrefab, AnimationPrefab, AnimationSetPrefab};
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
    sprite_animation: Option<AnimationSetPrefab<usize, SpriteRender>>,
    transform_animation: Option<AnimationSetPrefab<usize, Transform>>,
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
            sprite_animation: None,
            transform_animation: None,
            cells,
            render,
            transform,
        }
    }

    pub fn set_transform_hierarchy(
        &mut self,
        transform_hierarchy: AnimationHierarchyPrefab<Transform>,
    ) {
        self.transform_hierarchy = transform_hierarchy.into();
    }

    pub fn add_transform_animation(&mut self, animation: AnimationPrefab<Transform>) {
        if self.transform_animation.is_none() {
            self.transform_animation = AnimationSetPrefab::default().into();
        }
        let transform_animation = self.transform_animation.as_mut().unwrap();
        let id = transform_animation.animations.len();
        transform_animation.animations.push((id, animation));
    }
}
