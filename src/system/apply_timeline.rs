use crate::{
    components::{AnimationPart, AnimationRoot, AnimationTime, PlayAnimationKey},
    resource::AnimationStore,
    timeline::{FromUser, SpriteAnimation},
};
use amethyst::{
    assets::AssetStorage,
    core::{Hidden, Parent, ParentHierarchy},
    ecs::{
        storage::ComponentEvent, BitSet, Builder, Entities, Join, LazyUpdate, Read, ReadExpect,
        ReadStorage, ReaderId, System, SystemData, World, WorldExt, WriteStorage,
    },
    renderer::{pallet::rgb::Srgba, resources::Tint, SpriteRender},
    utils::removal::{exec_removal, Removal},
};
use itertools::izip;
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
        ReadStorage<'s, AnimationPart>,
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
            animation_parts,
            removal,
        ): Self::SystemData,
    ) {
        self.dirty.clear();

        let events = animation_keys
            .channel()
            .read(self.reader_id.as_mut().unwrap());

        for (e, anim_key, anim_time) in (&*entities, &animation_keys, &animation_times).join() {
            let current = anim_time.current_time();

            if let Some((anim_data, animation)) = anim_key.key().and_then(|(key, anim_id)| {
                animation_store
                    .animation(key)
                    .and_then(|anim_data| {
                        izip!(Some(anim_data), anim_data.animation(*anim_id)).next()
                    })
                    .and_then(|(anim_data, handle)| {
                        izip!(Some(anim_data), sprite_animation_storage.get(handle)).next()
                    })
            }) {
                let fps = animation.fps();
                let current_frame =
                    ((current * (fps as f32)).floor() as usize) % animation.total_frame();

                let children = parent_hierarchy.all_children(e);
                for (_, child, anim_part) in (children, &*entities, &animation_parts).join() {
                    if let Some((transform, visible, sprite_info, color)) = animation
                        .timelines()
                        .find(|tl| tl.part_id() == anim_part.0)
                        .map(|tl| {
                            (
                                tl.transforms().nth(current_frame).and_then(|t| t),
                                tl.visibles().nth(current_frame).and_then(|v| v),
                                tl.cells().nth(current_frame).and_then(|c| c).and_then(
                                    |(map_id, sprite_index)| {
                                        anim_data
                                            .sprite_sheet(map_id)
                                            .map(|sheet| (sheet, sprite_index))
                                    },
                                ),
                                tl.colors().nth(current_frame).and_then(|c| c),
                            )
                        })
                    {
                        if let Some(transform) = transform {
                            lazy.insert(child, transform.clone());
                        }

                        match visible {
                            Some(true) => {
                                lazy.remove::<Hidden>(child);
                            }
                            Some(false) => {
                                lazy.insert(child, Hidden);
                            }
                            None => {}
                        }

                        match sprite_info {
                            Some((sheet, sprite_number)) => lazy.insert(
                                child,
                                SpriteRender {
                                    sprite_sheet: sheet.clone(),
                                    sprite_number,
                                },
                            ),
                            None => {}
                        }

                        if let Some(color) = color {
                            lazy.insert(child, color.clone());
                        }
                    }
                }
            }
        }

        for e in events {
            match e {
                ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) => {
                    self.dirty.add(*id);
                }
                ComponentEvent::Removed(_) => {}
            }
        }

        for (_, e, anim_key) in (&self.dirty, &*entities, &animation_keys).join() {
            info!("{:?} key: {:?}", e, anim_key.key());
            exec_removal(&*entities, &removal, AnimationRoot(e));
            if let Some((anim_data, animation)) = anim_key.key().and_then(|(key, anim_id)| {
                animation_store
                    .animation(key)
                    .and_then(|anim_data| {
                        izip!(Some(anim_data), anim_data.animation(*anim_id)).next()
                    })
                    .and_then(|(anim_data, handle)| {
                        izip!(Some(anim_data), sprite_animation_storage.get(handle)).next()
                    })
            }) {
                let mut part_id_map = BTreeMap::new();
                for (part_id, parent_id, sprite_info, transform) in
                    animation.timelines().map(|tl| {
                        (
                            tl.part_id(),
                            tl.parent_id(),
                            tl.cells()
                                .nth(0)
                                .and_then(|c| c)
                                .and_then(|(map_id, sprite_index)| {
                                    anim_data
                                        .sprite_sheet(map_id)
                                        .map(|sheet| (sheet, sprite_index))
                                }),
                            tl.transforms().nth(0).and_then(|t| t),
                        )
                    })
                {
                    let parent = parent_id.map(|id| part_id_map[&id]);
                    let mut builder = lazy.create_entity(&*entities).with(AnimationPart(part_id));

                    builder = match parent {
                        Some(entity) => builder.with(Parent { entity }),
                        None => builder.with(Parent { entity: e }),
                    };
                    builder = match transform {
                        Some(transform) => builder.with(transform.clone()),
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
                    info!("id: {}, parent: {:?}, {:?}", part_id, parent_id, created);
                    part_id_map.insert(part_id, created);
                }
            }
        }
    }
}
