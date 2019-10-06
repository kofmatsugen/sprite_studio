use crate::{
    components::{AnimationTime, PlayAnimationKey},
    iter::AnimationNodes,
    resource::AnimationStore,
    traits::{collision_color::CollisionColor, AnimationKey, AnimationUser},
    SpriteAnimation,
};
use amethyst::{
    assets::AssetStorage,
    core::{
        math::{Matrix4, Point2},
        transform::Transform,
    },
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    renderer::{debug_drawing::DebugLinesComponent, palette::rgb::Srgba, ActiveCamera},
};
use std::{collections::BTreeMap, marker::PhantomData};

pub struct DebugCollisionSystem<K, U> {
    _key: PhantomData<K>,
    _user: PhantomData<U>,
}

impl<K, U> DebugCollisionSystem<K, U> {
    pub fn new() -> Self {
        DebugCollisionSystem {
            _key: PhantomData,
            _user: PhantomData,
        }
    }
}

impl<'s, K, U> System<'s> for DebugCollisionSystem<K, U>
where
    K: AnimationKey,
    U: AnimationUser + CollisionColor,
{
    type SystemData = (
        Entities<'s>,
        Read<'s, ActiveCamera>,
        Read<'s, AnimationStore<K, U>>,
        Read<'s, AssetStorage<SpriteAnimation<U>>>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, DebugLinesComponent>,
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, PlayAnimationKey<K>>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(
        &mut self,
        (
            entities,
            camera,
            animation_store,
            sprite_animation_storage,
            transforms,
            mut debug_lines,
            animation_times,
            animation_keys,
        ): Self::SystemData,
    ) {
        let camera_z = camera
            .entity
            .and_then(|entity| transforms.get(entity))
            .map(|transform| transform.translation().z - 1.);
        if camera_z.is_none() == true {
            return;
        }
        let position_z = camera_z.unwrap();

        for (e, transform, key, current) in
            (&*entities, &transforms, &animation_keys, &animation_times).join()
        {
            let debug = match debug_lines.entry(e) {
                Ok(entry) => entry.or_insert(DebugLinesComponent::new()),
                Err(err) => {
                    log::error!("{:?}", err);
                    continue;
                }
            };

            draw_collision(
                debug,
                *transform.global_matrix(),
                current,
                key,
                &animation_store,
                &sprite_animation_storage,
                position_z,
            );
        }
    }
}

fn draw_collision<K, U>(
    debug: &mut DebugLinesComponent,
    root_matrix: Matrix4<f32>,
    current: &AnimationTime,
    key: &PlayAnimationKey<K>,
    animation_store: &Read<AnimationStore<K, U>>,
    sprite_animation_storage: &Read<AssetStorage<SpriteAnimation<U>>>,
    position_z: f32,
) -> Option<()>
where
    K: AnimationKey,
    U: AnimationUser + CollisionColor,
{
    debug.clear();

    let nodes = AnimationNodes::new(
        key.key()?,
        current.current_time(),
        animation_store,
        sprite_animation_storage,
    )?;

    let mut global_matrixs = BTreeMap::new();
    for (_id, _info, key_frame, collision) in nodes
        .map(|(id, part_info, key_frame, _)| {
            let part_id = part_info.part_id();
            let parent_id = part_info.parent_id();

            // 親の位置からグローバル座標を算出．親がいなければルートが親
            let parent_matrix = parent_id
                .map(|parent_id| global_matrixs[&(id, parent_id)])
                .unwrap_or(root_matrix);

            // グローバル座標計算
            let global_matrix = parent_matrix * key_frame.transform().matrix();

            // 後ろのパーツの計算のために BTreeMap にセット
            global_matrixs.insert((id, part_id), global_matrix);

            (id, part_info, key_frame, global_matrix)
        })
        .filter(|(_, part_info, key_frame, _)| key_frame.visible() && part_info.bounds().is_some())
    {
        let collision: &[[f32; 4]; 4] = collision.as_ref();
        let width = collision[0][0];
        let height = collision[1][1];
        let x = collision[3][0];
        let y = collision[3][1];

        let min = Point2::new(x - width / 2., y - height / 2.);
        let max = Point2::new(x + width / 2., y + height / 2.);
        let color = key_frame.user().color();
        let color = Srgba::new(color[0], color[1], color[2], color[3]);
        debug.add_rectangle_2d(min, max, position_z, color);
    }
    Some(())
}
