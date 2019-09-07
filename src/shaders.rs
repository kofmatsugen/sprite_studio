use amethyst::renderer::rendy::{hal::pso::ShaderStageFlags, shader::SpirvShader};

lazy_static::lazy_static! {
    pub static ref SPRITE_VERTEX: SpirvShader = SpirvShader::new(
        include_bytes!("./shader/compiled/vertex/sprite.vert.spv").to_vec(),
        ShaderStageFlags::VERTEX,
        "main",
    );

    pub static ref SPRITE_FRAGMENT: SpirvShader = SpirvShader::new(
        include_bytes!("./shader/compiled/fragment/sprite.frag.spv").to_vec(),
        ShaderStageFlags::FRAGMENT,
        "main",
    );
}
