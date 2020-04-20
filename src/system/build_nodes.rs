use crate::{
    components::{AnimationNodes, AnimationTime, Node, PlayAnimationKey},
    resource::{data::AnimationData, name::AnimationName, AnimationStore},
    traits::translate_animation::TranslateAnimation,
};
use amethyst::{
    assets::AssetStorage,
    core::{math::Matrix4, Transform},
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
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
            ) {
                // 生成したノードをセットする
                match nodes.insert(e, node) {
                    Ok(_) => {}
                    Err(err) => log::error!("{:?}", err),
                }
            }
        }
    }
}

fn make_node<'s, T>(
    time: f32,
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

    let frame = animation.sec_to_frame(time);

    let mut nodes = AnimationNodes::new();
    for (part_id, part) in pack.parts().enumerate() {
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
            (Some(AnimationName::FullName { pack, animation }), Some(instance_key)) => {
                if instance_key.independent() == false {
                    log::info!("not independent instance:  {:?}/{:?}", pack, animation);
                    let root_transform = &part_transform;
                    let root_color = &part_color;
                    let root_matrix = &global_matrix;
                    let time = 0.;
                    make_node(
                        time,
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

        // ユーザーデータとはスプライトシートのハンドルをここでセット
        // ユーザーデータ
        if let Some(user) = animation.user(part_id, frame) {
            node.set_user(*user);
        }

        // スプライトシート
        if let Some((handle, sprite_no)) = animation.cell(part_id, frame).and_then(|cell| {
            let map_id = cell.map_id();
            let cell_id = cell.cell_id();
            let handle = store.get_sprite_handle(id, map_id)?;
            Some((handle.clone(), cell_id))
        }) {
            node.set_sprite_info(handle, sprite_no);
        }

        // インスタンス追加
        if let Some(instance) = instance_node {
            nodes.add_instance(instance);
        }

        nodes.push(node);
    }

    Some(nodes)
}
