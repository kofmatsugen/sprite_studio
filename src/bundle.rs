use crate::{
    resource::data::AnimationData, system::AnimationTimeIncrementSystem, traits::AnimationUser,
};

use amethyst::{
    assets::Processor,
    core::{
        bundle::SystemBundle,
        ecs::{DispatcherBuilder, World},
    },
};
use std::marker::PhantomData;

pub struct SpriteStudioBundle<U> {
    _user: PhantomData<U>,
}

impl<U> SpriteStudioBundle<U> {
    pub fn new() -> Self {
        SpriteStudioBundle { _user: PhantomData }
    }
}

impl<'a, 'b, U> SystemBundle<'a, 'b> for SpriteStudioBundle<U>
where
    U: AnimationUser,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        builder.add(
            Processor::<AnimationData<U>>::new(),
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
