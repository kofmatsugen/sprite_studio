use crate::{
    resource::{animation_store::AnimationData, AnimationStore},
    types::{animation_instance::InstanceKey, key_frame::KeyFrame, part_info::PartInfo},
    SpriteAnimation,
};
use amethyst::assets::AssetStorage;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::traits::{AnimationKey, AnimationUser};

type Node<'a, U> = (u64, &'a PartInfo, &'a KeyFrame<U>, &'a AnimationData<U>);

pub struct AnimationNodes<'a, K, U>
where
    K: AnimationKey,
    U: AnimationUser,
{
    key: &'a K,
    nodes: Vec<Node<'a, U>>,
    storage: &'a AssetStorage<SpriteAnimation<U>>,
    animation_store: &'a AnimationStore<K, U>,
}

impl<'a, K, U> AnimationNodes<'a, K, U>
where
    K: AnimationKey,
    U: AnimationUser,
{
    pub fn new(
        data_key: (&'a K, usize, usize),
        animation_time: f32,
        animation_store: &'a AnimationStore<K, U>,
        storage: &'a AssetStorage<SpriteAnimation<U>>,
    ) -> Option<Self> {
        Some(AnimationNodes {
            key: data_key.0,
            nodes: make_nodes(data_key, animation_time, animation_store, storage)?,
            storage,
            animation_store,
        })
    }
}

impl<'a, K, U> Iterator for AnimationNodes<'a, K, U>
where
    K: AnimationKey,
    U: AnimationUser,
{
    type Item = Node<'a, U>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.nodes.pop()?;

        let (_, part_info, key_frame, _) = item;

        match (
            key_frame.visible(),
            part_info.refference_animation(),
            key_frame.instance_key(),
        ) {
            (true, Some(ref_anim), Some(instance_key)) => {
                if instance_key.independent() == false {
                    let anim_key = (self.key, ref_anim.pack_id(), ref_anim.animation_id());
                    make_instance_nodes(anim_key, instance_key, self.animation_store, self.storage)
                        .map(|mut nodes| {
                            self.nodes.append(&mut nodes);
                        });
                }
            }
            _ => {}
        }

        Some(item)
    }
}

fn make_nodes<'a, K, U>(
    key: (&K, usize, usize),
    animation_time: f32,
    animation_store: &'a AnimationStore<K, U>,
    storage: &'a AssetStorage<SpriteAnimation<U>>,
) -> Option<Vec<Node<'a, U>>>
where
    K: AnimationKey,
    U: AnimationUser,
{
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);

    let (key, pack_id, anim_id) = key;

    let anim_data = animation_store.animation(key)?;
    let root_animation = anim_data
        .animation(pack_id, anim_id)
        .and_then(|handle| storage.get(handle))?;

    let fps = root_animation.fps();
    let current = (animation_time * (fps as f32)).floor() as usize;
    let current = current % root_animation.total_frame();

    Some(
        root_animation
            .timelines()
            .map(|tl| {
                (
                    hasher.finish(),
                    tl.part_info(),
                    tl.key_frame(current),
                    anim_data,
                )
            })
            .rev()
            .collect(),
    )
}

fn make_instance_nodes<'a, K, U>(
    key: (&K, usize, usize),
    instance_key: &InstanceKey,
    animation_store: &'a AnimationStore<K, U>,
    storage: &'a AssetStorage<SpriteAnimation<U>>,
) -> Option<Vec<Node<'a, U>>>
where
    K: AnimationKey,
    U: AnimationUser,
{
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);

    let (key, pack_id, anim_id) = key;

    let anim_data = animation_store.animation(key)?;
    let root_animation = anim_data
        .animation(pack_id, anim_id)
        .and_then(|handle| storage.get(handle))?;
    // 経過時間とアニメーションFPSからフレーム数算出
    let speed_rate = instance_key.speed_rate();
    let play_frame = ((instance_key.play_frame() as f32) * speed_rate).floor() as usize;
    let start_offset = instance_key.start_offset();
    let end_offset = instance_key.end_offset();
    let _reverse = instance_key.reverse();
    // 終端オフセットと開始オフセットから長さを計算
    let total_length = root_animation.total_frame() - end_offset - start_offset;

    // ループ数は往復再生なら二倍
    let loop_num = instance_key.loop_num().map(|num| {
        if instance_key.pingpong() {
            num * 2
        } else {
            num
        }
    });

    // 現在のループ回数を計算
    let current_loop_num = play_frame / total_length;

    let current = match loop_num {
        Some(loop_num) => {
            if loop_num > current_loop_num {
                (play_frame % total_length) + start_offset
            } else {
                total_length + start_offset - 1
            }
        }
        None => (play_frame % total_length) + start_offset,
    };

    log::debug!("[{:?}_{}_{}]: {} F", key, pack_id, anim_id, current);

    Some(
        root_animation
            .timelines()
            .map(|tl| {
                (
                    hasher.finish(),
                    tl.part_info(),
                    tl.key_frame(current),
                    anim_data,
                )
            })
            .rev()
            .collect(),
    )
}
