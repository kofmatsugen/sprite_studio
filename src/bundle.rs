use crate::{
    resource::data::AnimationData,
    system::{
        AnimationTimeIncrementSystem, AnimationTransitionSystem, BuildNodesSystem,
        KeyChangeEventSystem,
    },
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
            &["animation_time_increment"],
        );

        builder.add_barrier();

        builder.add(
            BuildNodesSystem::<T>::new(),
            "build_animation_node",
            &["animation_translate"],
        );

        builder.add(
            KeyChangeEventSystem::<T>::new(),
            "key_change_event",
            &["animation_translate"],
        );
        Ok(())
    }
}
