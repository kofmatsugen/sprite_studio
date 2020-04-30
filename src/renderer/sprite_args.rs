use amethyst::renderer::rendy::{
    hal::format::Format,
    util::types::vertex::{AsVertex, VertexFormat},
};
use glsl_layout::{float, vec2, vec4, AsStd140};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, AsStd140)]
#[repr(C, align(4))]
pub(crate) struct SpriteArgs {
    /// Upper-left coordinate of the sprite in the spritesheet
    pub u_offset: vec2,
    /// Bottom-right coordinate of the sprite in the spritesheet
    pub v_offset: vec2,
    /// Depth value of this sprite
    pub depth: float,
    /// Tint for this this sprite
    pub tint: vec4,
    pub deforms: [vec2; 4],
}

impl AsVertex for SpriteArgs {
    fn vertex() -> VertexFormat {
        VertexFormat::new((
            (Format::Rg32Sfloat, "u_offset"),
            (Format::Rg32Sfloat, "v_offset"),
            (Format::R32Sfloat, "depth"),
            (Format::Rgba32Sfloat, "tint"),
            (Format::Rg32Sfloat, "deform_lt"),
            (Format::Rg32Sfloat, "deform_lb"),
            (Format::Rg32Sfloat, "deform_rt"),
            (Format::Rg32Sfloat, "deform_rb"),
        ))
    }
}
