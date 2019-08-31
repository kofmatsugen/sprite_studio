use crate::{
    components::{AnimationRoot, AnimationTime, PlayAnimationKey},
    resource::AnimationStore,
    timeline::{FromUser, SpriteAnimation},
};
use amethyst::{
    assets::AssetStorage,
    core::{Parent, ParentHierarchy, Transform},
    ecs::{
        storage::ComponentEvent, BitSet, Builder, Entities, Join, LazyUpdate, Read, ReadExpect,
        ReadStorage, ReaderId, System, SystemData, World, WorldExt, WriteStorage,
    },
    renderer::SpriteRender,
    utils::removal::{exec_removal, Removal},
};
use log::*;
use serde::Serialize;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub struct TimeLineApplySystem<K, U> {
    _key: PhantomData<K>,
    _user: PhantomData<U>,
    dirty: BitSet,
    reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<K, U> TimeLineApplySystem<K, U> {
    pub fn new() -> Self {
        TimeLineApplySystem {
            _key: PhantomData,
            _user: PhantomData,
            dirty: BitSet::new(),
            reader_id: None,
        }
    }
}

impl<'s, K, U> System<'s> for TimeLineApplySystem<K, U>
where
    K: 'static + Send + Sync + std::hash::Hash + PartialOrd + Ord + std::fmt::Debug,
    U: 'static + Send + Sync + FromUser + Serialize,
{
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, AnimationStore<K, U>>,
        Read<'s, AssetStorage<SpriteAnimation<U>>>,
        ReadExpect<'s, ParentHierarchy>,
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, PlayAnimationKey<K>>,
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Removal<AnimationRoot>>,
    );

    fn setup(&mut self, world: &mut World) {
        world.register::<AnimationRoot>();
        Self::SystemData::setup(world);
        self.reader_id = Some(WriteStorage::<PlayAnimationKey<K>>::fetch(world).register_reader());
    }

    fn run(
        &mut self,
        (
            entities,
            lazy,
            animation_store,
            sprite_animation_storage,
            parent_hierarchy,
            animation_times,
            animation_keys,
            sprite_renders,
            transforms,
            removal,
        ): Self::SystemData,
    ) {
        self.dirty.clear();

        let events = animation_keys
            .channel()
            .read(self.reader_id.as_mut().unwrap());

        for e in events {
            match e {
                ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) => {
                    self.dirty.add(*id);
                }
                ComponentEvent::Removed(_) => {}
            }
        }

        for (_, e, key) in (&self.dirty, &*entities, &animation_keys).join() {
            info!("{:?} key: {:?}", e, key.key());
            exec_removal(&*entities, &removal, AnimationRoot(e));
            if let Some((key, anim_id)) = key.key() {
                if let Some(anim_data) = animation_store.animation(key) {
                    if let Some(anim) = anim_data
                        .animation(*anim_id)
                        .and_then(|anim| sprite_animation_storage.get(anim))
                    {
                        let mut part_id_map = BTreeMap::new();
                        for (part_id, parent_id, sprite_info, transform) in
                            anim.timelines().map(|tl| {
                                (
                                    tl.part_id(),
                                    tl.parent_id(),
                                    tl.cells().nth(0).and_then(|c| c).and_then(
                                        |(map_id, sprite_index)| {
                                            anim_data
                                                .sprite_sheet(map_id)
                                                .map(|sheet| (sheet, sprite_index))
                                        },
                                    ),
                                    tl.transforms().nth(0).and_then(|t| t),
                                )
                            })
                        {
                            info!("id: {}, parent: {:?}", part_id, parent_id);
                            let parent = parent_id.map(|id| part_id_map[&id]);
                            let mut builder = lazy.create_entity(&*entities);

                            builder = match parent {
                                Some(entity) => builder.with(Parent { entity }),
                                None => builder,
                            };
                            builder = match transform {
                                Some(transform) => {
                                    info!("\ttransform: {:?}", transform);
                                    builder.with(transform.clone())
                                }
                                None => builder,
                            };
                            builder = match sprite_info {
                                Some((sprite_sheet, sprite_number)) => builder.with(SpriteRender {
                                    sprite_sheet: sprite_sheet.clone(),
                                    sprite_number,
                                }),
                                None => builder,
                            };

                            let created = builder.build();
                            part_id_map.insert(part_id, created);
                        }
                    }
                }
            }
        }
    }
}
