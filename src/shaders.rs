use amethyst::renderer::rendy::{hal::pso::ShaderStageFlags, shader::SpirvShader};

lazy_static::lazy_static! {
    pub static ref SPRITE_VERTEX: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("./shader/compiled/vertex/sprite.vert.spv"),
        ShaderStageFlags::VERTEX,
        "main",
    ).unwrap();

    pub static ref SPRITE_FRAGMENT: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("./shader/compiled/fragment/sprite.frag.spv"),
        ShaderStageFlags::FRAGMENT,
        "main",
    ).unwrap();
}
