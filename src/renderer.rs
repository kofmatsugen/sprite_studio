mod sprite_args;

use sprite_args::SpriteArgs;

use crate::{
    components::{AnimationNodes, Node},
    traits::translate_animation::TranslateAnimation,
};
use amethyst::{
    assets::{AssetStorage, Handle},
    core::math::{Matrix4, Vector4},
    ecs::{Join, Read, ReadStorage, SystemData, World},
    error::Error,
    renderer::{
        batch::{GroupIterator, OneLevelBatch},
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        pipeline::{PipelineDescBuilder, PipelinesBuilder},
        pod::IntoPod,
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
        sprite::SpriteSheet,
        submodules::{DynamicVertexBuffer, FlatEnvironmentSub, TextureId, TextureSub},
        types::{Backend, Texture},
        util::simple_shader_set,
    },
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct RenderSpriteAnimation<T> {
    _translation: PhantomData<T>,
    target: Target,
}

impl<T> Default for RenderSpriteAnimation<T> {
    fn default() -> Self {
        RenderSpriteAnimation {
            _translation: PhantomData,
            target: Default::default(),
        }
    }
}

impl<T> RenderSpriteAnimation<T> {
    /// Set target to which 2d sprites will be rendered.
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

impl<'s, B, T> RenderPlugin<B> for RenderSpriteAnimation<T>
where
    B: Backend,
    T: TranslateAnimation<'s> + std::fmt::Debug,
{
    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(self.target, |ctx| {
            ctx.add(
                RenderOrder::Transparent,
                DrawSpriteAnimationDesc::<T>::new().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}

#[derive(Debug)]
pub struct DrawSpriteAnimationDesc<T> {
    _translation: PhantomData<T>,
}

impl<T> DrawSpriteAnimationDesc<T> {
    fn new() -> Self {
        DrawSpriteAnimationDesc {
            _translation: PhantomData,
        }
    }
}

impl<'s, B, T> RenderGroupDesc<B, World> for DrawSpriteAnimationDesc<T>
where
    B: Backend,
    T: TranslateAnimation<'s> + std::fmt::Debug,
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

        Ok(Box::new(DrawSpriteAnimation::<B, T> {
            pipeline,
            pipeline_layout,
            env,
            textures,
            vertex,
            sprites: Default::default(),
            _translation: PhantomData,
        }))
    }
}

#[derive(Debug)]
pub struct DrawSpriteAnimation<B: Backend, T> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    env: FlatEnvironmentSub<B>,
    textures: TextureSub<B>,
    vertex: DynamicVertexBuffer<B, SpriteArgs>,
    sprites: OneLevelBatch<TextureId, SpriteArgs>,
    _translation: PhantomData<T>,
}

impl<'s, B, T> RenderGroup<B, World> for DrawSpriteAnimation<B, T>
where
    B: Backend,
    T: TranslateAnimation<'s> + std::fmt::Debug,
{
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: Subpass<B>,
        world: &World,
    ) -> PrepareResult {
        let (sprite_sheet_storage, tex_storage, nodes) = <(
            Read<AssetStorage<SpriteSheet>>,
            Read<AssetStorage<Texture>>,
            ReadStorage<AnimationNodes<T::UserData>>,
        )>::fetch(world);

        self.env.process(factory, index, world);

        let sprites_ref = &mut self.sprites;
        let textures_ref = &mut self.textures;

        sprites_ref.clear_inner();

        let mut joined = (&nodes,).join().collect::<Vec<_>>();
        joined.sort_by(|(nodes1,), (nodes2,)| {
            let z1 = nodes1.nodes().nth(0).unwrap().transform.translation().z;
            let z2 = nodes2.nodes().nth(0).unwrap().transform.translation().z;
            z1.partial_cmp(&z2).unwrap()
        });

        for (nodes,) in joined {
            build_animation::<B, T>(
                nodes,
                &sprite_sheet_storage,
                &tex_storage,
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
    let shader_vertex = unsafe { crate::shaders::SPRITE_VERTEX.module(factory).unwrap() };
    let shader_fragment = unsafe { crate::shaders::SPRITE_FRAGMENT.module(factory).unwrap() };

    // パイプライン生成．
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
    deform_offsets: &[[f32; 2]; 4],
) -> Option<(SpriteArgs, &'a Handle<Texture>)> {
    let sprite_sheet = sprite_storage.get(&sprite_sheet)?;
    if !tex_storage.contains(&sprite_sheet.texture) {
        return None;
    }

    let sprite = &sprite_sheet.sprites[sprite_no];

    let transform = global_matrix;
    let pos = transform * Vector4::new(-sprite.offsets[0], -sprite.offsets[1], 0.0, 1.0);

    let mut deforms = [
        [0.0; 2].into(), // right bottom
        [0.0; 2].into(), // left bottom
        [0.0; 2].into(), //
        [0.0; 2].into(),
    ];
    for (i, deform) in deform_offsets.iter().enumerate() {
        let left = if i % 2 == 0 { 0.5 } else { -0.5 };
        let top = if i / 2 == 0 { 0.5 } else { -0.5 };
        deforms[i] = (transform
            * Vector4::new(
                -sprite.offsets[0] + left * sprite.width + deform[0],
                -sprite.offsets[1] + top * sprite.height + deform[1],
                0.0,
                1.0,
            ))
        .xy()
        .into_pod();
    }

    log::debug!("\tmatrix: {:?}", transform);
    log::debug!("\t\tcolor: {:?}", tint);
    log::debug!("\t\tdeform offset: {:?}", deform_offsets);
    log::debug!("\t\tdeforms: {:?}", deforms);

    Some((
        SpriteArgs {
            u_offset: [sprite.tex_coords.left, sprite.tex_coords.right].into(),
            v_offset: [sprite.tex_coords.top, sprite.tex_coords.bottom].into(),
            depth: pos.z,
            tint: tint.into(),
            deforms,
        },
        &sprite_sheet.texture,
    ))
}

fn build_animation<'s, B, T>(
    nodes: &AnimationNodes<T::UserData>,
    sprite_sheet_storage: &Read<AssetStorage<SpriteSheet>>,
    tex_storage: &Read<AssetStorage<Texture>>,
    factory: &Factory<B>,
    world: &World,
    textures_ref: &mut TextureSub<B>,
) -> Option<Vec<(TextureId, SpriteArgs)>>
where
    B: Backend,
    T: TranslateAnimation<'s>,
{
    let node_groups = nodes.nodes().filter_map(
        |Node {
             global_matrix,
             hide,
             color,
             sprite_sheet,
             sprite_no,
             deform_offsets,
             ..
         }| {
            if *hide == true {
                return None;
            }

            let sprite_sheet = sprite_sheet.as_ref()?;
            let sprite_no = (*sprite_no)?;
            from_global_matrix_data(
                tex_storage,
                sprite_sheet_storage,
                sprite_sheet,
                sprite_no,
                global_matrix,
                *color,
                deform_offsets,
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
        },
    );

    let mut n_group = node_groups.collect::<Vec<_>>();

    let instance_group = nodes
        .instance_nodes()
        .filter_map(|nodes| {
            Some(
                build_animation::<B, T>(
                    nodes,
                    sprite_sheet_storage,
                    tex_storage,
                    factory,
                    world,
                    textures_ref,
                )?
                .into_iter(),
            )
        })
        .flatten();

    let mut i_group = instance_group.collect::<Vec<_>>();
    n_group.append(&mut i_group);
    Some(n_group)
}
