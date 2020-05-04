use crate::{components::AnimationNodes, traits::animation_file::AnimationFile};
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use movement_transform::components::Movement;
use std::marker::PhantomData;

// アニメーションのキーを変更したときにイベントを発行する
pub struct RootTranslateSystem<T>
where
    T: 'static + Send + Sync,
{
    _translation: PhantomData<T>,
}

impl<T> RootTranslateSystem<T>
where
    T: 'static + Send + Sync,
{
    pub fn new() -> Self {
        RootTranslateSystem {
            _translation: PhantomData,
        }
    }
}

impl<'s, T> System<'s> for RootTranslateSystem<T>
where
    T: AnimationFile,
{
    type SystemData = (
        ReadStorage<'s, AnimationNodes<T::UserData>>,
        WriteStorage<'s, Movement>,
    );

    fn run(&mut self, (nodes, mut movements): Self::SystemData) {
        for (node, movement) in (&nodes, &mut movements).join() {
            let (x, y) = node.root_translate();

            movement.transform_mut().translation_mut().x += x;
            movement.transform_mut().translation_mut().y += y;
        }
    }
}
