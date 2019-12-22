use crate::{
    system::AnimationTimeIncrementSystem,
    traits::{AnimationKey, AnimationUser},
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

pub struct SpriteStudioBundle<K, U> {
    _key: PhantomData<K>,
    _user: PhantomData<U>,
}

impl<K, U> SpriteStudioBundle<K, U> {
    pub fn new() -> Self {
        SpriteStudioBundle {
            _key: PhantomData,
            _user: PhantomData,
        }
    }
}

impl<'a, 'b, K, U> SystemBundle<'a, 'b> for SpriteStudioBundle<K, U>
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
