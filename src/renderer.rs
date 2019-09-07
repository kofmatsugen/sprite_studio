use amethyst::{
    assets::AssetStorage,
    core::{timing::Time, transform::Transform},
    ecs::{DispatcherBuilder, Read, ReadExpect, ReadStorage, SystemData, World},
    error::Error,
    renderer::{
        batch::OrderedOneLevelBatch,
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        pipeline::{PipelineDescBuilder, PipelinesBuilder},
        pod::SpriteArgs,
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
        sprite::{SpriteRender, SpriteSheet},
        sprite_visibility::{SpriteVisibility, SpriteVisibilitySortingSystem},
        submodules::{DynamicVertexBuffer, FlatEnvironmentSub, TextureId, TextureSub},
        types::{Backend, Texture},
        util::simple_shader_set,
    },
};

#[derive(Default, Debug)]
pub struct RenderSpriteAnimation {
    target: Target,
}

impl RenderSpriteAnimation {
    /// Set target to which 2d sprites will be rendered.
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

impl<B: Backend> RenderPlugin<B> for RenderSpriteAnimation {
    fn on_build<'a, 'b>(
        &mut self,
        world: &mut World,
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
                DrawSpriteAnimationDesc::new().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawSpriteAnimationDesc;

impl DrawSpriteAnimationDesc {
    fn new() -> Self {
        DrawSpriteAnimationDesc
    }
}

impl<B: Backend> RenderGroupDesc<B, World> for DrawSpriteAnimationDesc {
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
            false,
            vec![env.raw_layout(), textures.raw_layout()],
        )?;

        Ok(Box::new(DrawSpriteAnimation::<B> {
            pipeline,
            pipeline_layout,
            env,
            textures,
            vertex,
            sprites: Default::default(),
        }))
    }
}

#[derive(Debug)]
pub struct DrawSpriteAnimation<B: Backend> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    env: FlatEnvironmentSub<B>,
    textures: TextureSub<B>,
    vertex: DynamicVertexBuffer<B, SpriteArgs>,
    sprites: OrderedOneLevelBatch<TextureId, SpriteArgs>,
}

impl<B> RenderGroup<B, World> for DrawSpriteAnimation<B>
where
    B: Backend,
{
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: Subpass<B>,
        world: &World,
    ) -> PrepareResult {
        // let (sprite_sheet_storage, tex_storage, visibility, sprite_renders, transforms, tints) =
        //     <(
        //         Read<'_, AssetStorage<SpriteSheet>>,
        //         Read<'_, AssetStorage<Texture>>,
        //         ReadExpect<'_, SpriteVisibility>,
        //         ReadStorage<'_, SpriteRender>,
        //         ReadStorage<'_, Transform>,
        //         ReadStorage<'_, Tint>,
        //     )>::fetch(world);

        let time = <Read<Time>>::fetch(world);

        self.env.process(factory, index, world);
        self.sprites.swap_clear();

        let sprites_ref = &mut self.sprites;
        let textures_ref = &mut self.textures;
        let mut changed = false;

        log::info!("[prepare] {}: queue: {:?}", time.frame_number(), _queue);
        log::info!("[prepare] {}: index: {:?}", time.frame_number(), index);
        log::info!("[prepare] {}: subpass: {:?}", time.frame_number(), _subpass);

        PrepareResult::DrawRecord
    }

    fn draw_inline(
        &mut self,
        mut encoder: RenderPassEncoder<B>,
        index: usize,
        _subpass: Subpass<B>,
        world: &World,
    ) {
        let time = <Read<Time>>::fetch(world);
        log::info!("[draw] {}: encoder: {:?}", time.frame_number(), encoder);
        log::info!("[draw] {}: index: {:?}", time.frame_number(), index);
        log::info!("[draw] {}: subpass: {:?}", time.frame_number(), _subpass);

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

    let shader_vertex = unsafe { crate::shaders::SPRITE_VERTEX.module(factory).unwrap() };
    let shader_fragment = unsafe { crate::shaders::SPRITE_FRAGMENT.module(factory).unwrap() };

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
                        pso::BlendState::PREMULTIPLIED_ALPHA
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
