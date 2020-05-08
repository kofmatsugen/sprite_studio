use crate::{
    resource::data::AnimationData,
    system::{AnimationTimeIncrementSystem, AnimationTransitionSystem, RootTranslateSystem},
    traits::translate_animation::TranslateAnimation,
};

use amethyst::{
    assets::Processor,
    core::{
        bundle::SystemBundle,
        ecs::{DispatcherBuilder, World},
    },
};
use std::marker::PhantomData;

pub struct SpriteStudioBundle<T> {
    _marker: PhantomData<T>,
}

impl<T> SpriteStudioBundle<T> {
    pub fn new() -> Self {
        SpriteStudioBundle {
            _marker: PhantomData,
        }
    }
}

impl<'a, 'b, T> SystemBundle<'a, 'b> for SpriteStudioBundle<T>
where
    T: for<'c> TranslateAnimation<'c> + 'a,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        builder.add(
            Processor::<AnimationData<T>>::new(),
            "sprite_animation_processor",
            &[],
        );

        builder.add(
            AnimationTimeIncrementSystem::new(),
            "animation_time_increment",
            &[],
        );

        builder.add_barrier();

        builder.add(
            AnimationTransitionSystem::<T>::new(),
            "animation_translate",
            &[],
        );

        builder.add_barrier();

        builder.add(RootTranslateSystem::<T>::new(), "root_translate", &[]);

        Ok(())
    }
}
