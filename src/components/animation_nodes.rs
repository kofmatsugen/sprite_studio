use amethyst::{
    core::{math::Matrix4, Transform},
    ecs::{Component, DenseVecStorage},
    renderer::sprite::SpriteSheetHandle,
};
use smallvec::SmallVec;
pub struct AnimationNodes<T> {
    nodes: SmallVec<[Node<T>; 32]>,
    instance_nodes: Vec<AnimationNodes<T>>,
}

impl<T> AnimationNodes<T> {
    pub(crate) fn new() -> Self {
        let nodes = SmallVec::new();
        AnimationNodes {
            nodes,
            instance_nodes: Vec::with_capacity(4),
        }
    }

    pub(crate) fn push(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    pub(crate) fn add_instance(&mut self, instance: Self) {
        self.instance_nodes.push(instance);
    }

    pub fn node(&self, part_id: usize) -> Option<&Node<T>> {
        self.nodes.get(part_id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node<T>> {
        self.nodes.iter()
    }

    pub fn instance_nodes(&self) -> impl Iterator<Item = &Self> {
        self.instance_nodes.iter()
    }
}

impl<T> Component for AnimationNodes<T>
where
    T: 'static + Send + Sync,
{
    type Storage = DenseVecStorage<Self>;
}

pub struct Node<T> {
    pub transform: Transform,
    pub global_matrix: Matrix4<f32>,
    pub hide: bool,
    pub user: Option<T>,
    pub sprite_sheet: Option<SpriteSheetHandle>, // 描画用スプライトシートハンドル
    pub sprite_no: Option<usize>,
    pub color: [f32; 4],
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
        }
    }

    pub(crate) fn set_user(&mut self, user: T) {
        self.user = user.into();
    }

    pub(crate) fn set_sprite_info(&mut self, sprite_sheet: SpriteSheetHandle, sprite_no: usize) {
        self.sprite_sheet = sprite_sheet.into();
        self.sprite_no = sprite_no.into();
    }
}
