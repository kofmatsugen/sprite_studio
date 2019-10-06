use crate::{
    components::{AnimationTime, PlayAnimationKey},
    iter::AnimationNodes,
    resource::AnimationStore,
    traits::{AnimationKey, AnimationUser},
    SpriteAnimation,
};
use amethyst::{
    assets::{AssetStorage, Handle},
    core::{
        math::{Matrix4, Vector4},
        transform::Transform,
    },
    ecs::{DispatcherBuilder, Join, Read, ReadStorage, SystemData, World},
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
use std::collections::BTreeMap;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct RenderSpriteAnimation<K, U> {
    _key: PhantomData<K>,
    _user: PhantomData<U>,
    target: Target,
}

impl<K, U> Default for RenderSpriteAnimation<K, U> {
    fn default() -> Self {
        RenderSpriteAnimation {
            _key: PhantomData,
            _user: PhantomData,
            target: Default::default(),
        }
    }
}

impl<K, U> RenderSpriteAnimation<K, U> {
    /// Set target to which 2d sprites will be rendered.
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

impl<B: Backend, K, U> RenderPlugin<B> for RenderSpriteAnimation<K, U>
where
    K: AnimationKey,
    U: AnimationUser,
{
    fn on_build<'a, 'b>(
        &mut self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
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
                DrawSpriteAnimationDesc::<K, U>::new().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawSpriteAnimationDesc<K, U> {
    _key: std::marker::PhantomData<K>,
    _user: std::marker::PhantomData<U>,
}

impl<K, U> DrawSpriteAnimationDesc<K, U> {
    fn new() -> Self {
        DrawSpriteAnimationDesc {
            _key: std::marker::PhantomData,
            _user: std::marker::PhantomData,
        }
    }
}

impl<B: Backend, K, U> RenderGroupDesc<B, World> for DrawSpriteAnimationDesc<K, U>
where
    K: AnimationKey,
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

        Ok(Box::new(DrawSpriteAnimation::<B, K, U> {
            pipeline,
            pipeline_layout,
            env,
            textures,
            vertex,
            sprites: Default::default(),
            _key: std::marker::PhantomData,
            _user: std::marker::PhantomData,
        }))
    }
}

#[derive(Debug)]
pub struct DrawSpriteAnimation<B: Backend, K, U> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    env: FlatEnvironmentSub<B>,
    textures: TextureSub<B>,
    vertex: DynamicVertexBuffer<B, SpriteArgs>,
    sprites: OneLevelBatch<TextureId, SpriteArgs>,
    _key: std::marker::PhantomData<K>,
    _user: std::marker::PhantomData<U>,
}

impl<B, K, U> RenderGroup<B, World> for DrawSpriteAnimation<B, K, U>
where
    B: Backend,
    K: AnimationKey,
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
            animation_store,
            sprite_animation_storage,
            transforms,
            tints,
            animation_times,
            animation_keys,
        ) = <(
            Read<AssetStorage<SpriteSheet>>,
            Read<AssetStorage<Texture>>,
            Read<AnimationStore<K, U>>,
            Read<AssetStorage<SpriteAnimation<U>>>,
            ReadStorage<Transform>,
            ReadStorage<Tint>,
            ReadStorage<AnimationTime>,
            ReadStorage<PlayAnimationKey<K>>,
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
            let key = match key.key() {
                Some(key) => key,
                None => continue,
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
                .with_blend_targets(vec![pso::ColorBlendDesc(
                    pso::ColorMask::ALL,
                    if transparent {
                        pso::BlendState::ALPHA
                    } else {
                        pso::BlendState::Off
                    },
                )])
                .with_depth_test(pso::DepthTest::On {
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

fn build_animation<B, K, U>(
    animation_key: (&K, usize, usize),
    current_time: f32,
    root_color: [f32; 4],
    animation_store: &Read<AnimationStore<K, U>>,
    sprite_animation_storage: &Read<AssetStorage<SpriteAnimation<U>>>,
    sprite_sheet_storage: &Read<AssetStorage<SpriteSheet>>,
    tex_storage: &Read<AssetStorage<Texture>>,
    root_matrix: Matrix4<f32>,
    factory: &Factory<B>,
    world: &World,
    textures_ref: &mut TextureSub<B>,
) -> Option<Vec<(TextureId, SpriteArgs)>>
where
    B: Backend,
    K: AnimationKey,
    U: AnimationUser,
{
    let nodes = AnimationNodes::new(
        animation_key,
        current_time,
        animation_store,
        sprite_animation_storage,
    )?;

    let mut global_matrixs = BTreeMap::new();
    let mut global_colors = BTreeMap::new();

    let groups = nodes.filter_map(|(id, part_info, key_frame, anim_data)| {
        let part_id = part_info.part_id();
        let parent_id = part_info.parent_id();
        // 親の位置からグローバル座標を算出．親がいなければルートが親
        let parent_matrix = match parent_id {
            Some(parent_id) => global_matrixs[&(id, parent_id)],
            None => root_matrix,
        };

        // 親の色を踏襲する
        let parent_color = match parent_id {
            Some(parent_id) => global_colors[&(id, parent_id)],
            None => root_color,
        };

        // グローバル座標計算
        let global_matrix = parent_matrix * key_frame.transform().matrix();

        // 後ろのパーツのサイズ計算のために BTreeMap にセット
        global_matrixs.insert((id, part_id), global_matrix);

        // 乗算カラー値計算
        let (r, g, b, a) = key_frame.color().0.into_components();
        let part_color = [r, g, b, a];
        let mut global_color = [0.; 4];
        for i in 0..4 {
            global_color[i] = part_color[i] * parent_color[i];
        }
        // 後ろのパーツの色計算のために BTreeMap にセット
        global_colors.insert((id, part_id), global_color);

        // 以下で描画設定
        // 表示が不要ならここで終了
        if key_frame.visible() == false {
            return None;
        }

        log::debug!("{}: {:?} => {:?}", part_id, part_color, global_color);
        let command = key_frame
            .cell()
            .and_then(|(map_id, sprite_index)| {
                anim_data
                    .sprite_sheet(map_id)
                    .map(|sheet| (sheet, sprite_index))
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
