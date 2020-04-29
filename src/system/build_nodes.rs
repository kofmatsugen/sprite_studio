use crate::{
    components::{AnimationNodes, AnimationTime, Node, PlayAnimationKey},
    resource::{
        animation::Animation, data::AnimationData, name::AnimationName, pack::Pack, AnimationStore,
    },
    traits::translate_animation::TranslateAnimation,
    types::{
        event::{AnimationEvent, AnimationEventChannel},
        InstanceKey,
    },
};
use amethyst::{
    assets::AssetStorage,
    core::{math::Matrix4, Transform},
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    renderer::resources::Tint,
};
use std::marker::PhantomData;

pub struct BuildNodesSystem<T> {
    user: PhantomData<T>,
}

impl<T> BuildNodesSystem<T> {
    pub fn new() -> Self {
        BuildNodesSystem { user: PhantomData }
    }
}

impl<'s, T> System<'s> for BuildNodesSystem<T>
where
    T: TranslateAnimation<'s>,
{
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, AnimationNodes<T::UserData>>,
        ReadStorage<'s, AnimationTime>,
        ReadStorage<'s, PlayAnimationKey<T>>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Tint>,
        Write<'s, AnimationEventChannel<T>>,
        Read<'s, AssetStorage<AnimationData<T>>>,
        Read<'s, AnimationStore<T>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut nodes,
            animation_times,
            animation_keys,
            transforms,
            tints,
            mut channel,
            animation_storage,
            animation_store,
        ): Self::SystemData,
    ) {
        // 座標系計算をここに集約して，AnimationNodes にキャッシュする
        for (e, time, key, transform, tint) in (
            &*entities,
            &animation_times,
            &animation_keys,
            &transforms,
            tints.maybe(),
        )
            .join()
        {
            let root_color = tint
                .map(|tint| {
                    let (r, g, b, a) = tint.0.into_components();
                    [r, g, b, a]
                })
                .unwrap_or([1.0; 4]);
            if let Some(node) = make_node(
                time.current_time(),
                key.play_key(),
                transform,
                transform.global_matrix(),
                &root_color,
                &animation_store,
                &animation_storage,
                e,
                &mut channel,
            ) {
                // 生成したノードをセットする
                match nodes.insert(e, node) {
                    Ok(_) => {
                        log::trace!(
                            "node root position = ({:.3}, {:.3})",
                            transform.translation().x,
                            transform.translation().y,
                        );
                    }
                    Err(err) => log::error!("{:?}", err),
                }
            }
        }
    }
}

// 実時間でノードを作成
fn make_node<'s, T>(
    time: f32,
    key: Option<(&T::FileId, &T::PackKey, &T::AnimationKey)>,
    root_transform: &Transform,
    root_matrix: &Matrix4<f32>,
    root_color: &[f32; 4],
    store: &AnimationStore<T>,
    animation_storage: &AssetStorage<AnimationData<T>>,
    entity: Entity,
    channel: &mut AnimationEventChannel<T>,
) -> Option<AnimationNodes<T::UserData>>
where
    T: TranslateAnimation<'s>,
{
    log::debug!("make node start: {:?}", key?);
    let (id, pack_id, animation_id) = key?;

    let handle = store.get_animation_handle(id)?;
    let pack = animation_storage.get(handle)?.pack(pack_id)?;
    let animation = pack.animation(animation_id)?;

    let frame = animation.sec_to_frame(time);

    if frame >= animation.total_frame() {
        // エンティティに直接付随してるアニメーションが終わったのでイベントを登録
        channel.single_write(AnimationEvent::End {
            entity,
            file_id: *id,
            pack: *pack_id,
            animation: *animation_id,
        });
        return None;
    }

    make_animation_nodes::<T>(
        frame,
        root_transform,
        root_matrix,
        root_color,
        pack,
        animation,
        id,
        store,
        animation_storage,
    )
}

// インスタンスパーツのノード作成
fn make_instance_nodes<'s, T>(
    key_set_frame: usize,
    current_frame: usize,
    instance: &InstanceKey,
    key: Option<(&T::FileId, &T::PackKey, &T::AnimationKey)>,
    root_transform: &Transform,
    root_matrix: &Matrix4<f32>,
    root_color: &[f32; 4],
    store: &AnimationStore<T>,
    animation_storage: &AssetStorage<AnimationData<T>>,
) -> Option<AnimationNodes<T::UserData>>
where
    T: TranslateAnimation<'s>,
{
    let (id, pack_id, animation_id) = key?;

    let handle = store.get_animation_handle(id)?;
    let pack = animation_storage.get(handle)?.pack(pack_id)?;
    let animation = pack.animation(animation_id)?;

    // アニメーション情報からインスタンスの再生フレームを算出
    // インスタンスキーには再生開始位置，終了位置が載っている
    // 開始位置と実際の再生フレートの差がインスタンスパーツ上の再生位置
    // この時点で再生速度を考慮する(f32 -> usize キャストは 0 方向へ丸められる)
    let end_frame = animation.total_frame() - instance.end_offset();
    let play_frame = (((current_frame - key_set_frame) as f32) * instance.speed_rate()) as usize;
    // 1回の再生時間算出
    let once_play_time = end_frame - instance.start_offset() + 1;
    // 再生回数と，その再生フレーム値を算出
    let played_loop_num = play_frame / once_play_time;
    let mut current_play_frame = play_frame % once_play_time;
    if let Some(loop_num) = instance.loop_num() {
        if loop_num <= played_loop_num {
            // 再生回数が指定回数を超えてるので終了
            return None;
        }
    }

    // 逆再生の場合か，pingpong再生の奇数ループ目は再生フレーム値が逆になる
    if instance.reverse() == true || (played_loop_num % 2 == 1 && instance.pingpong() == true) {
        current_play_frame = once_play_time - current_play_frame;
    }

    make_animation_nodes::<T>(
        current_play_frame + instance.start_offset(),
        root_transform,
        root_matrix,
        root_color,
        pack,
        animation,
        id,
        store,
        animation_storage,
    )
}

// アニメーション，パックデータからノード作成
fn make_animation_nodes<'s, T>(
    frame: usize,
    root_transform: &Transform,
    root_matrix: &Matrix4<f32>,
    root_color: &[f32; 4],
    pack: &Pack<T::UserData, T::PackKey, T::AnimationKey>,
    animation: &Animation<T::UserData>,
    // インスタンスノードに必要な情報
    id: &T::FileId,
    store: &AnimationStore<T>,
    animation_storage: &AssetStorage<AnimationData<T>>,
) -> Option<AnimationNodes<T::UserData>>
where
    T: TranslateAnimation<'s>,
{
    // 再生できないので総フレーム数よりあとの場合はノードを作らない
    if frame >= animation.total_frame() {
        return None;
    }

    log::trace!("make node: {} F", frame);

    let mut nodes = AnimationNodes::new();
    for (part_id, part) in pack.parts().enumerate() {
        log::trace!("\tmake node: part = {}", part_id);
        // 親ノードの情報を取得,なければ Entity の情報
        let (parent_transform, parent_color, parent_hide, parent_matrix) = part
            .parent_id()
            .and_then(|p| nodes.node(p as usize))
            .map(
                |Node {
                     transform,
                     color,
                     hide,
                     global_matrix,
                     ..
                 }| (transform, color, *hide, global_matrix),
            )
            .unwrap_or((root_transform, root_color, false, root_matrix));

        // パーツ座標のグローバル化
        let mut part_transform = parent_transform.clone();
        let local_transform = animation.local_transform(part_id, frame);
        part_transform.concat(&local_transform);
        part_transform.translation_mut().z = local_transform.translation().z;

        // 描画用座標のマトリクス計算
        let global_matrix = parent_matrix * local_transform.matrix();

        // パーツカラーのグローバル化
        let (r, g, b, a) = animation.local_color(part_id, frame).0.into_components();
        let mut part_color = [r, g, b, a];
        for (i, c) in parent_color.iter().enumerate() {
            part_color[i] *= c;
        }

        // 独立再生じゃないインスタンスパーツだった場合，このパーツの下にノードを追加する
        let instance_node = match (
            part.refference_animation_name(),
            animation.instance(part_id, frame),
        ) {
            (
                Some(AnimationName::FullName { pack, animation }),
                Some((instance_frame, instance_key)),
            ) => {
                if instance_key.independent() == false {
                    let root_transform = &part_transform;
                    let root_color = &part_color;
                    let root_matrix = &global_matrix;

                    make_instance_nodes(
                        instance_frame, // キーフレームがセットされたフレーム
                        frame,          // 親アニメーションの今のフレーム
                        instance_key,   // インスタンスキー情報
                        Some((id, pack, animation)),
                        root_transform,
                        root_matrix,
                        root_color,
                        store,
                        animation_storage,
                    )
                } else {
                    None
                }
            }
            _ => None,
        };

        // 親の非表示情報を引き継いでノード作成
        let mut node = Node::new(
            part_transform,
            global_matrix,
            part_color,
            parent_hide || animation.hide(part_id, frame),
        );

        //-------------------------------------
        // ユーザーデータとはスプライトシートのハンドルをここでセット
        // ユーザーデータ
        if let Some(user) = animation.user(part_id, frame) {
            node.set_user(*user);
        }

        // スプライトシート
        if let Some((handle, sprite_no)) = animation
            .cell(part_id, frame)
            .or(pack.setup_info().and_then(|setup| setup.cell(part_id, 0))) // セルがセットアップ上にあるかもしれない
            .and_then(|cell| {
                let map_id = cell.map_id();
                let cell_id = cell.cell_id();
                let handle = store.get_sprite_handle(id, map_id)?;
                Some((handle.clone(), cell_id))
            })
        {
            log::trace!("\tmake sprite: {:?}, {}", handle, sprite_no);
            node.set_sprite_info(handle, sprite_no);
        }

        // インスタンス追加
        if let Some(instance) = instance_node {
            nodes.add_instance(instance);
        }

        log::trace!("\tmake end: part = {}", part_id);
        nodes.push(node);
    }
    nodes.sort_by_z();
    Some(nodes)
}
