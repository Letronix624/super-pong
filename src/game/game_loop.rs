use super::{Layers, Message};
use anyhow::Result;
use let_engine::prelude::*;

pub struct Loop {}
fn load_material(asset: &[u8], layers: u32) -> Option<Material> {
    let sampler = SamplerBuilder::default()
        .mag_filter(Filter::Nearest)
        .min_filter(Filter::Nearest)
        .build()
        .ok()?;
    let texure_settings = TextureSettings {
        srgb: true,
        sampler,
    };
    let texture =
        Texture::from_bytes(asset, ImageFormat::Png, layers, texure_settings.clone()).ok()?;
    Some(Material::new_default_textured(&texture))
}

impl Loop {
    pub fn load(layers: &Layers) -> Result<Self> {
        Ok(Self {})
    }

    pub fn unload(self) {}

    pub fn update(&mut self, layers: &Layers) -> Option<Message> {
        let mut message = None;
        message
    }
}
