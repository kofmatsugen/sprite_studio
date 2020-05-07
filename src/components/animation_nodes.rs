use crate::{
    components::{AnimationTime, PlayAnimationKey},
    resource::{
        animation::Animation, data::AnimationData, name::AnimationName, pack::Pack, AnimationStore,
    },
    traits::animation_file::AnimationFile,
    types::InstanceKey,
};
use amethyst::{
    assets::AssetStorage,
    core::{math::Matrix4, Transform},
    ecs::{Read, ReadStorage},
    renderer::resources::Tint,
    renderer::sprite::SpriteSheetHandle,
};
use smallvec::SmallVec;

pub struct AnimationNodes<T> {
    play_frame: usize,
    nodes: SmallVec<[Node<T>; 32]>,
    instance_nodes: Vec<AnimationNodes<T>>,
}

impl<T> AnimationNodes<T> {
    pub fn node(&self, part_id: usize) -> Option<&Node<T>> {
        self.nodes.get(part_id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node<T>> {
        self.nodes.iter()
    }

    pub fn instance_nodes(&self) -> impl Iterator<Item = &Self> {
        self.instance_nodes.iter()
    }
    pub fn play_frame(&self) -> usize {
        self.play_frame
    }

    fn new(play_frame: usize) -> Self {
        let nodes = SmallVec::new();
        AnimationNodes {
            play_frame,
            nodes,
            instance_nodes: Vec::with_capacity(4),
        }
    }

    pub(crate) fn sort_by_z(&mut self) {
        self.nodes.sort_by(|node1, node2| {
            let z1 = node1.transform.translation().z;
            let z2 = node2.transform.translation().z;
            z1.partial_cmp(&z2).unwrap()
        });
        for instance in self.instance_nodes.iter_mut() {
            instance.sort_by_z();
        }
        self.instance_nodes.sort_by(|instance1, instance2| {
            let z1 = instance1.nodes[0].transform.translation().z;
            let z2 = instance2.nodes[0].transform.translation().z;
            z1.partial_cmp(&z2).unwrap()
        });
    }

    fn push(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    fn add_instance(&mut self, instance: Self) {
        self.instance_nodes.push(instance);
    }
}

pub type BuildRequireData<'s, T> = (
    ReadStorage<'s, AnimationTime>,
    ReadStorage<'s, PlayAnimationKey<T>>,
    ReadStorage<'s, Transform>,
    ReadStorage<'s, Tint>,
    Read<'s, AssetStorage<AnimationData<T>>>,
    Read<'s, AnimationStore<T>>,
);

impl<'s, U> AnimationNodes<U> {
    // 実時間でノードを作成
    pub fn make_node<T>(
        time: &AnimationTime,
        tint: Option<&Tint>,
        key: Option<(&T::FileId, &T::PackKey, &T::AnimationKey)>,
        root_transform: &Transform,
        root_matrix: &Matrix4<f32>,
        store: &AnimationStore<T>,
        animation_storage: &AssetStorage<AnimationData<T>>,
    ) -> Option<AnimationNodes<T::UserData>>
    where
        T: AnimationFile,
    {
        let current_time = match time {
            AnimationTime::Play { current_time, .. } => *current_time,
            AnimationTime::Stop { stopped_time, .. } => *stopped_time,
        };
        let root_color = tint
            .map(|tint| {
                let (r, g, b, a) = tint.0.into_components();
                [r, g, b, a]
            })
            .unwrap_or([1.0; 4]);
        log::debug!("make node start: {:?}", key?);
        let (id, pack_id, animation_id) = key?;

        let handle = store.get_animation_handle(id)?;
        let pack = animation_storage.get(handle)?.pack(pack_id)?;
        let animation = pack.animation(animation_id)?;

        let current_frame = animation.sec_to_frame(current_time);

        Self::make_animation_nodes(
            current_frame,
            root_transform,
            root_matrix,
            &root_color,
            pack,
            animation,
            id,
            store,
            animation_storage,
        )
    }

    // インスタンスパーツのノード作成
    fn make_instance_nodes<T>(
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
        T: AnimationFile,
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
        let play_frame =
            (((current_frame - key_set_frame) as f32) * instance.speed_rate()) as usize;
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

        Self::make_animation_nodes(
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
    fn make_animation_nodes<T>(
        current_frame: usize,
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
        T: AnimationFile,
    {
        // 再生できないので総フレーム数よりあとの場合はノードを作らない
        if current_frame >= animation.total_frame() {
            return None;
        }

        log::trace!("make node: {} F", current_frame);

        let mut nodes = AnimationNodes::new(current_frame);
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
                     }| (transform.clone(), *color, *hide, *global_matrix),
                )
                .unwrap_or((root_transform.clone(), *root_color, false, *root_matrix));

            // パーツ座標のグローバル化
            let mut part_transform = parent_transform.clone();
            let mut local_transform = animation.local_transform(part_id, current_frame);

            // ルートの移動値は後で足すのでここではゼロとする
            if part_id == crate::constant::ROOT_PART_ID {
                local_transform.set_translation_xyz(0., 0., 0.);
            }

            part_transform.concat(&local_transform);
            part_transform.translation_mut().z =
                local_transform.translation().z + root_transform.translation().z;

            // 描画用座標のマトリクス計算
            let global_matrix = parent_matrix * local_transform.matrix();

            // パーツカラーのグローバル化
            let (r, g, b, a) = animation
                .local_color(part_id, current_frame)
                .0
                .into_components();
            let mut part_color = [r, g, b, a];
            for (i, c) in parent_color.iter().enumerate() {
                part_color[i] *= c;
            }

            // 独立再生じゃないインスタンスパーツだった場合，このパーツの下にノードを追加する
            let instance_node = match (
                part.refference_animation_name(),
                animation.instance(part_id, current_frame),
            ) {
                (
                    Some(AnimationName::FullName { pack, animation }),
                    Some((instance_frame, instance_key)),
                ) => {
                    if instance_key.independent() == false {
                        let root_transform = &part_transform;
                        let root_color = &part_color;
                        let root_matrix = &global_matrix;

                        Self::make_instance_nodes(
                            instance_frame, // キーフレームがセットされたフレーム
                            current_frame,  // 親アニメーションの今のフレーム
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
                parent_hide || animation.hide(part_id, current_frame),
            );

            //-------------------------------------
            // ユーザーデータとはスプライトシートのハンドルをここでセット
            // ユーザーデータ
            if let Some(user) = animation.user(part_id, current_frame) {
                node.set_user(*user);
            }

            // スプライトシート
            if let Some((handle, sprite_no)) = animation
                .cell(part_id, current_frame)
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

            if let Some(deforms) = animation.vertex(part_id, current_frame) {
                node.set_deform(
                    [deforms.lt().0, deforms.lt().1],
                    [deforms.lb().0, deforms.lb().1],
                    [deforms.rt().0, deforms.rt().1],
                    [deforms.rb().0, deforms.rb().1],
                );
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
}

pub struct Node<T> {
    pub transform: Transform,
    pub global_matrix: Matrix4<f32>,
    pub hide: bool,
    pub user: Option<T>,
    pub sprite_sheet: Option<SpriteSheetHandle>, // 描画用スプライトシートハンドル
    pub sprite_no: Option<usize>,
    pub color: [f32; 4],
    pub deform_offsets: [[f32; 2]; 4],
}

impl<T> Node<T> {
    pub(crate) fn new(
        transform: Transform,
        global_matrix: Matrix4<f32>,
        color: [f32; 4],
        hide: bool,
    ) -> Self {
        Node {
            transform,
            global_matrix,
            hide,
            user: None,
            sprite_sheet: None,
            sprite_no: None,
            color,
            deform_offsets: [[0.; 2]; 4],
        }
    }

    pub(crate) fn set_user(&mut self, user: T) {
        self.user = user.into();
    }

    pub(crate) fn set_sprite_info(&mut self, sprite_sheet: SpriteSheetHandle, sprite_no: usize) {
        self.sprite_sheet = sprite_sheet.into();
        self.sprite_no = sprite_no.into();
    }

    pub(crate) fn set_deform(&mut self, lt: [f32; 2], lb: [f32; 2], rt: [f32; 2], rb: [f32; 2]) {
        self.deform_offsets = [rt, lt, rb, lb];
    }
}
