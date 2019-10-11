use crate::{
    system::{AnimationTimeIncrementSystem, DebugCollisionSystem},
    traits::{AnimationKey, AnimationUser, CollisionColor},
    SpriteAnimation,
};

use amethyst::{
    assets::Processor,
    core::{
        bundle::SystemBundle,
        ecs::{DispatcherBuilder, World},
    },
};
use std::marker::PhantomData;

pub struct WithoutDebugCollision;
pub struct WithDebugCollision;

pub struct SpriteStudioBundleBuilder;

impl SpriteStudioBundleBuilder {
    pub fn new<K, U>() -> SpriteStudioBundle<K, U, WithoutDebugCollision> {
        SpriteStudioBundle {
            _key: PhantomData,
            _user: PhantomData,
            _debug_collisiton: PhantomData,
        }
    }
    pub fn with_debug_collision<K, U>() -> SpriteStudioBundle<K, U, WithDebugCollision> {
        SpriteStudioBundle {
            _key: PhantomData,
            _user: PhantomData,
            _debug_collisiton: PhantomData,
        }
    }
}

pub struct SpriteStudioBundle<K, U, D> {
    _key: PhantomData<K>,
    _user: PhantomData<U>,
    _debug_collisiton: PhantomData<D>,
}

impl<'a, 'b, K, U> SystemBundle<'a, 'b> for SpriteStudioBundle<K, U, WithoutDebugCollision>
where
    K: AnimationKey,
    U: AnimationUser,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        builder.add(
            Processor::<SpriteAnimation<U>>::new(),
            "sprite_animation_processor",
            &[],
        );
        builder.add(
            AnimationTimeIncrementSystem::new(),
            "animation_time_increment",
            &[],
        );
        Ok(())
    }
}

impl<'a, 'b, K, U> SystemBundle<'a, 'b> for SpriteStudioBundle<K, U, WithDebugCollision>
where
    K: AnimationKey,
    U: AnimationUser + CollisionColor,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        log::info!("build start");
        builder.add(
            Processor::<SpriteAnimation<U>>::new(),
            "sprite_animation_processor",
            &[],
        );
        builder.add(
            AnimationTimeIncrementSystem::new(),
            "animation_time_increment",
            &[],
        );
        builder.add(DebugCollisionSystem::<K, U>::new(), "debug_collision", &[]);
        log::info!("build start");
        Ok(())
    }
}
