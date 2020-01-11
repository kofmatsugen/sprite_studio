use crate::{
    components::{AnimationTime, PlayAnimationKey},
    resource::{data::AnimationData, AnimationStore},
    traits::{AnimationKey, AnimationUser, FileId},
};
use amethyst::{
    assets::{AssetStorage, Handle},
    core::{
        math::{Matrix4, Vector4},
        transform::Transform,
    },
    ecs::{DispatcherBuilder, Join, Read, ReadStorage, SystemData, World, WorldExt},
    error::Error,
    renderer::{
        batch::{GroupIterator, OneLevelBatch},
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        pipeline::{PipelineDescBuilder, PipelinesBuilder},
        pod::{IntoPod, SpriteArgs},
        rendy::{
            command::{QueueId, RenderPassEncoder},
            factory::Factory,
            graph::{
                render::{PrepareResult, RenderGroup, RenderGroupDesc},
                GraphContext, NodeBuffer, NodeImage,
            },
            hal::{self, device::Device, pass::Subpass, pso},
            mesh::AsVertex,
            shader::Shader,
        },
        resources::Tint,
        sprite::SpriteSheet,
        sprite_visibility::SpriteVisibilitySortingSystem,
        submodules::{DynamicVertexBuffer, FlatEnvironmentSub, TextureId, TextureSub},
        types::{Backend, Texture},
        util::simple_shader_set,
    },
};
use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug)]
pub struct RenderSpriteAnimation<ID, P, A, U> {
    _file_id: PhantomData<ID>,
    _pack_name: PhantomData<P>,
    _animation_name: PhantomData<A>,
    _user: PhantomData<U>,
    target: Target,
}

impl<ID, P, A, U> Default for RenderSpriteAnimation<ID, P, A, U> {
    fn default() -> Self {
        RenderSpriteAnimation {
            _file_id: PhantomData,
            _pack_name: PhantomData,
            _animation_name: PhantomData,
            _user: PhantomData,
            target: Default::default(),
        }
    }
}

impl<ID, P, A, U> RenderSpriteAnimation<ID, P, A, U> {
    /// Set target to which 2d sprites will be rendered.
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

impl<B: Backend, ID, P, A, U> RenderPlugin<B> for RenderSpriteAnimation<ID, P, A, U>
where
    ID: FileId,
    P: AnimationKey,
    A: AnimationKey,
    U: AnimationUser,
{
    fn on_build<'a, 'b>(
        &mut self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        world.register::<PlayAnimationKey<ID, P, A>>();
        builder.add(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &[],
        );
        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(self.target, |ctx| {
            ctx.add(
                RenderOrder::Transparent,
                DrawSpriteAnimationDesc::<ID, P, A, U>::new().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}

#[derive(Debug)]
pub struct DrawSpriteAnimationDesc<ID, P, A, U> {
    _file_id: PhantomData<ID>,
    _pack_name: PhantomData<P>,
    _animation_name: PhantomData<A>,
    _user: std::marker::PhantomData<U>,
}

impl<ID, P, A, U> DrawSpriteAnimationDesc<ID, P, A, U> {
    fn new() -> Self {
        DrawSpriteAnimationDesc {
            _file_id: PhantomData,
            _pack_name: PhantomData,
            _animation_name: PhantomData,
            _user: std::marker::PhantomData,
        }
    }
}

impl<B, ID, P, A, U> RenderGroupDesc<B, World> for DrawSpriteAnimationDesc<ID, P, A, U>
where
    B: Backend,
    ID: FileId,
    P: AnimationKey,
    A: AnimationKey,
    U: AnimationUser,
{
    fn build(
        self,
        _ctx: &GraphContext<B>,
        factory: &mut Factory<B>,
        _queue: QueueId,
        _aux: &World,
        framebuffer_width: u32,
        framebuffer_height: u32,
        subpass: Subpass<'_, B>,
        _buffers: Vec<NodeBuffer>,
        _images: Vec<NodeImage>,
    ) -> Result<Box<dyn RenderGroup<B, World>>, failure::Error> {
        #[cfg(feature = "profiler")]
        profile_scope!("build");

        let env = FlatEnvironmentSub::new(factory)?;
        let textures = TextureSub::new(factory)?;
        let vertex = DynamicVertexBuffer::new();

        let (pipeline, pipeline_layout) = build_sprite_pipeline(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
            true,
            vec![env.raw_layout(), textures.raw_layout()],
        )?;

        Ok(Box::new(DrawSpriteAnimation::<B, ID, P, A, U> {
            pipeline,
            pipeline_layout,
            env,
            textures,
            vertex,
            sprites: Default::default(),
            _file_id: PhantomData,
            _pack_name: PhantomData,
            _animation_name: PhantomData,
            _user: std::marker::PhantomData,
        }))
    }
}

#[derive(Debug)]
pub struct DrawSpriteAnimation<B: Backend, ID, P, A, U> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    env: FlatEnvironmentSub<B>,
    textures: TextureSub<B>,
    vertex: DynamicVertexBuffer<B, SpriteArgs>,
    sprites: OneLevelBatch<TextureId, SpriteArgs>,
    _file_id: PhantomData<ID>,
    _pack_name: PhantomData<P>,
    _animation_name: PhantomData<A>,
    _user: std::marker::PhantomData<U>,
}

impl<B, ID, P, A, U> RenderGroup<B, World> for DrawSpriteAnimation<B, ID, P, A, U>
where
    B: Backend,
    ID: FileId,
    P: AnimationKey,
    A: AnimationKey,
    U: AnimationUser,
{
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: Subpass<B>,
        world: &World,
    ) -> PrepareResult {
        let (
            sprite_sheet_storage,
            tex_storage,
            sprite_animation_storage,
            animation_store,
            transforms,
            tints,
            animation_times,
            animation_keys,
        ) = <(
            Read<AssetStorage<SpriteSheet>>,
            Read<AssetStorage<Texture>>,
            Read<AssetStorage<AnimationData<U>>>,
            Read<AnimationStore<ID, U>>,
            ReadStorage<Transform>,
            ReadStorage<Tint>,
            ReadStorage<AnimationTime>,
            ReadStorage<PlayAnimationKey<ID, P, A>>,
        )>::fetch(world);

        self.env.process(factory, index, world);

        let sprites_ref = &mut self.sprites;
        let textures_ref = &mut self.textures;

        sprites_ref.clear_inner();

        for (transform, key, current, tint) in (
            &transforms,
            &animation_keys,
            &animation_times,
            tints.maybe(),
        )
            .join()
        {
            let current_time = current.current_time();
            let matrix = *transform.global_matrix();
            let key = match (key.file_id(), key.pack_name(), key.animation_name()) {
                (id, Some(pack), Some(anim)) => (id, pack, anim),
                _ => continue,
            };
            let color = tint
                .map(|tint| {
                    let (r, g, b, a) = tint.0.into_components();
                    [r, g, b, a]
                })
                .unwrap_or([1.0; 4]);

            build_animation(
                key,
                current_time,
                color,
                &animation_store,
                &sprite_animation_storage,
                &sprite_sheet_storage,
                &tex_storage,
                matrix,
                &factory,
                &world,
                textures_ref,
            )
            .map(|commands| {
                commands.into_iter().for_each_group(|tex_id, batch_data| {
                    sprites_ref.insert(tex_id, batch_data.drain(..))
                })
            });
        }

        self.textures.maintain(factory, world);

        sprites_ref.prune();
        self.vertex.write(
            factory,
            index,
            self.sprites.count() as u64,
            self.sprites.data(),
        );

        PrepareResult::DrawRecord
    }

    fn draw_inline(
        &mut self,
        mut encoder: RenderPassEncoder<B>,
        index: usize,
        _subpass: Subpass<B>,
        _world: &World,
    ) {
        let layout = &self.pipeline_layout;
        encoder.bind_graphics_pipeline(&self.pipeline);
        self.env.bind(index, layout, 0, &mut encoder);
        self.vertex.bind(index, 0, 0, &mut encoder);
        for (&tex, range) in self.sprites.iter() {
            if self.textures.loaded(tex) {
                self.textures.bind(layout, 1, tex, &mut encoder);
                unsafe {
                    encoder.draw(0..4, range);
                }
            }
        }
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _: &World) {
        unsafe {
            factory.device().destroy_graphics_pipeline(self.pipeline);
            factory
                .device()
                .destroy_pipeline_layout(self.pipeline_layout);
        }
    }
}

fn build_sprite_pipeline<B: Backend>(
    factory: &Factory<B>,
    subpass: hal::pass::Subpass<'_, B>,
    framebuffer_width: u32,
    framebuffer_height: u32,
    transparent: bool,
    layouts: Vec<&B::DescriptorSetLayout>,
) -> Result<(B::GraphicsPipeline, B::PipelineLayout), failure::Error> {
    let pipeline_layout = unsafe {
        factory
            .device()
            .create_pipeline_layout(layouts, None as Option<(_, _)>)
    }?;

    // AmethystのDrawFlat2Dのシェーダーを流用．
    // todo: SpirVのコンパイル方法を調べる必要あり
    let shader_vertex = unsafe { crate::shaders::SPRITE_VERTEX.module(factory).unwrap() };
    let shader_fragment = unsafe { crate::shaders::SPRITE_FRAGMENT.module(factory).unwrap() };

    // パイプライン生成．
    // todo: これの意味を調べる．vulkan のドキュメントが使える？
    let pipes = PipelinesBuilder::new()
        .with_pipeline(
            PipelineDescBuilder::new()
                .with_vertex_desc(&[(SpriteArgs::vertex(), pso::VertexInputRate::Instance(1))])
                .with_input_assembler(pso::InputAssemblerDesc::new(hal::Primitive::TriangleStrip))
                .with_shaders(simple_shader_set(&shader_vertex, Some(&shader_fragment)))
                .with_layout(&pipeline_layout)
                .with_subpass(subpass)
                .with_framebuffer_size(framebuffer_width, framebuffer_height)
                .with_blend_targets(vec![pso::ColorBlendDesc {
                    mask: pso::ColorMask::ALL,
                    blend: if transparent {
                        pso::BlendState::ALPHA
                    } else {
                        pso::BlendState::REPLACE
                    }
                    .into(),
                }])
                .with_depth_test(pso::DepthTest {
                    fun: pso::Comparison::Less,
                    write: !transparent,
                }),
        )
        .build(factory, None);

    unsafe {
        factory.destroy_shader_module(shader_vertex);
        factory.destroy_shader_module(shader_fragment);
    }

    match pipes {
        Err(e) => {
            unsafe {
                factory.device().destroy_pipeline_layout(pipeline_layout);
            }
            Err(e)
        }
        Ok(mut pipes) => Ok((pipes.remove(0), pipeline_layout)),
    }
}

// シェーダーにわたすパラメータ生成
fn from_global_matrix_data<'a>(
    tex_storage: &AssetStorage<Texture>,
    sprite_storage: &'a AssetStorage<SpriteSheet>,
    sprite_sheet: &Handle<SpriteSheet>,
    sprite_no: usize,
    global_matrix: &Matrix4<f32>,
    tint: [f32; 4],
) -> Option<(SpriteArgs, &'a Handle<Texture>)> {
    let sprite_sheet = sprite_storage.get(&sprite_sheet)?;
    if !tex_storage.contains(&sprite_sheet.texture) {
        return None;
    }

    let sprite = &sprite_sheet.sprites[sprite_no];

    let transform = global_matrix;
    let dir_x = transform.column(0) * sprite.width;
    let dir_y = transform.column(1) * -sprite.height;
    let pos = transform * Vector4::new(-sprite.offsets[0], -sprite.offsets[1], 0.0, 1.0);

    log::debug!("\tmatrix: {:?}", transform);
    log::debug!("\t\tpos  : {:?}", pos.xy());
    log::debug!("\t\tdir_x: {:?}", dir_x.xy());
    log::debug!("\t\tdir_y: {:?}", dir_y.xy());
    log::debug!("\t\tcolor: {:?}", tint);

    Some((
        SpriteArgs {
            dir_x: dir_x.xy().into_pod(),
            dir_y: dir_y.xy().into_pod(),
            pos: pos.xy().into_pod(),
            u_offset: [sprite.tex_coords.left, sprite.tex_coords.right].into(),
            v_offset: [sprite.tex_coords.top, sprite.tex_coords.bottom].into(),
            depth: pos.z,
            tint: tint.into(),
        },
        &sprite_sheet.texture,
    ))
}

fn build_animation<B, ID, P, A, U>(
    (id, &pack_id, &animation_id): (&ID, &P, &A),
    current_time: f32,
    root_color: [f32; 4],
    animation_store: &Read<AnimationStore<ID, U>>,
    sprite_animation_storage: &Read<AssetStorage<AnimationData<U>>>,
    sprite_sheet_storage: &Read<AssetStorage<SpriteSheet>>,
    tex_storage: &Read<AssetStorage<Texture>>,
    root_matrix: Matrix4<f32>,
    factory: &Factory<B>,
    world: &World,
    textures_ref: &mut TextureSub<B>,
) -> Option<Vec<(TextureId, SpriteArgs)>>
where
    B: Backend,
    ID: FileId,
    P: AnimationKey,
    A: AnimationKey,
    U: AnimationUser,
{
    let pack = animation_store
        .get_animation_handle(id)
        .and_then(|handle| sprite_animation_storage.get(handle))
        .and_then(|data| data.pack(&pack_id.to_string()))?;

    let animation = pack.animation(&animation_id.to_string())?;
    let frame = animation.sec_to_frame_loop(current_time);
    let mut global_matrixs = BTreeMap::new();
    let mut global_colors = BTreeMap::new();

    let groups = pack.parts().enumerate().filter_map(|(part_id, part)| {
        let parent_id = part.parent_id().map(|parent_id| parent_id as usize);
        let hash_key = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            pack_id.hash(&mut hasher);
            animation_id.hash(&mut hasher);
            part_id.hash(&mut hasher);
            hasher.finish()
        };

        // 親の位置からグローバル座標を算出．親がいなければルートが親
        let parent_matrix = match parent_id {
            Some(parent_id) => {
                let hash_key = {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    pack_id.hash(&mut hasher);
                    animation_id.hash(&mut hasher);
                    parent_id.hash(&mut hasher);
                    hasher.finish()
                };
                global_matrixs[&hash_key]
            }

            None => root_matrix,
        };

        // 親の色を踏襲する
        let parent_color = match parent_id {
            Some(parent_id) => {
                let hash_key = {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    pack_id.hash(&mut hasher);
                    animation_id.hash(&mut hasher);
                    parent_id.hash(&mut hasher);
                    hasher.finish()
                };
                global_colors[&hash_key]
            }
            None => root_color,
        };

        // グローバル座標計算
        let global_matrix = parent_matrix * animation.local_transform(part_id, frame).matrix();

        // 後ろのパーツのサイズ計算のために BTreeMap にセット
        global_matrixs.insert(hash_key, global_matrix);

        // 乗算カラー値計算
        let (r, g, b, a) = animation.local_color(part_id, frame).0.into_components();
        let part_color = [r, g, b, a];
        let mut global_color = [0.; 4];
        for i in 0..4 {
            global_color[i] = part_color[i] * parent_color[i];
        }
        // 後ろのパーツの色計算のために BTreeMap にセット
        global_colors.insert(hash_key, global_color);

        // 以下で描画設定
        // 表示が不要ならここで終了
        if animation.hide(part_id, frame) == true {
            return None;
        }

        let command = animation
            .cell(part_id, frame)
            .and_then(|cell| {
                let map_id = cell.map_id();
                let cell_id = cell.cell_id();
                let handle = match animation_store.get_sprite_handle(id, map_id) {
                    Some(handle) => Some(handle),
                    None => {
                        log::error!("notfound {:?}", (map_id, cell_id));
                        None
                    }
                }?;
                Some((handle, cell_id))
            })
            .and_then(|(sprite_sheet, sprite_no)| {
                from_global_matrix_data(
                    tex_storage,
                    sprite_sheet_storage,
                    &sprite_sheet,
                    sprite_no,
                    &global_matrix,
                    global_color,
                )
                .and_then(|(batch_data, texture)| {
                    let (tex_id, _) = textures_ref.insert(
                        factory,
                        world,
                        texture,
                        hal::image::Layout::ShaderReadOnlyOptimal,
                    )?;
                    Some((tex_id, batch_data))
                })
            });

        command
    });
    Some(groups.collect())
}
