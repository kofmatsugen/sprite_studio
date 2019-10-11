use crate::{
    resource::{animation_store::AnimationData, AnimationStore},
    traits::{AnimationKey, AnimationUser},
    types::animation_instance::InstanceKey,
    types::node::Node,
    utils::{convert_time_to_frame, convert_time_to_frame_range},
    SpriteAnimation,
};
use amethyst::assets::AssetStorage;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct AnimationNodes<'a, U>
where
    U: AnimationUser,
{
    nodes: Vec<Node<'a, U>>,
    storage: &'a AssetStorage<SpriteAnimation<U>>,
    animation_data: &'a AnimationData<U>,
}

impl<'a, U> AnimationNodes<'a, U>
where
    U: AnimationUser,
{
    pub fn new<K>(
        data_key: (&'a K, usize, usize),
        animation_time: f32,
        animation_store: &'a AnimationStore<K, U>,
        storage: &'a AssetStorage<SpriteAnimation<U>>,
    ) -> Option<Self>
    where
        K: AnimationKey,
    {
        let (key, pack_id, anim_id) = data_key;
        let animation_data = animation_store.animation(key)?;
        Some(AnimationNodes {
            nodes: make_nodes((pack_id, anim_id), animation_time, animation_data, storage)?,
            storage,
            animation_data,
        })
    }

    pub(crate) fn range<K>(
        data_key: (&'a K, usize, usize),
        start: f32,
        end: f32,
        animation_store: &'a AnimationStore<K, U>,
        storage: &'a AssetStorage<SpriteAnimation<U>>,
    ) -> Option<Vec<Self>>
    where
        K: AnimationKey,
    {
        let (key, pack_id, anim_id) = data_key;
        let mut hasher = DefaultHasher::new();
        (pack_id, anim_id).hash(&mut hasher);
        let animation_data = animation_store.animation(key)?;

        let root_animation = animation_data
            .animation(pack_id, anim_id)
            .and_then(|handle| storage.get(handle))?;

        Some(
            convert_time_to_frame_range(start, end, root_animation)
                .map(|frame| AnimationNodes {
                    nodes: make_nodes_from_frame(
                        hasher.finish(),
                        frame,
                        root_animation,
                        animation_data,
                    ),
                    storage,
                    animation_data,
                })
                .rev()
                .collect(),
        )
    }
}

impl<'a, U> Iterator for AnimationNodes<'a, U>
where
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
                    let anim_key = (ref_anim.pack_id(), ref_anim.animation_id());
                    make_instance_nodes(anim_key, instance_key, self.animation_data, self.storage)
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

fn make_nodes_from_frame<'a, U>(
    hash_id: u64,
    frame: usize,
    data: &'a SpriteAnimation<U>,
    anim_data: &'a AnimationData<U>,
) -> Vec<Node<'a, U>>
where
    U: AnimationUser,
{
    data.timelines()
        .map(|tl| (hash_id, tl.part_info(), tl.key_frame(frame), anim_data))
        .rev()
        .collect()
}

fn make_nodes<'a, U>(
    key: (usize, usize),
    animation_time: f32,
    animation_data: &'a AnimationData<U>,
    storage: &'a AssetStorage<SpriteAnimation<U>>,
) -> Option<Vec<Node<'a, U>>>
where
    U: AnimationUser,
{
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);

    let (pack_id, anim_id) = key;

    let root_animation = animation_data
        .animation(pack_id, anim_id)
        .and_then(|handle| storage.get(handle))?;

    let frame = convert_time_to_frame(animation_time, root_animation);

    Some(make_nodes_from_frame(
        hasher.finish(),
        frame,
        root_animation,
        animation_data,
    ))
}

fn make_instance_nodes<'a, U>(
    key: (usize, usize),
    instance_key: &InstanceKey,
    animation_data: &'a AnimationData<U>,
    storage: &'a AssetStorage<SpriteAnimation<U>>,
) -> Option<Vec<Node<'a, U>>>
where
    U: AnimationUser,
{
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);

    let (pack_id, anim_id) = key;

    let root_animation = animation_data
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

    Some(make_nodes_from_frame(
        hasher.finish(),
        current,
        root_animation,
        animation_data,
    ))
}
