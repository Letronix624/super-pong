use std::sync::Arc;

use anyhow::Result;
/// The player seen on the left side of the screen.
use let_engine::prelude::*;

use crate::HEIGHT;

#[derive(Clone, Debug)]
pub struct Paddle {
    health: f32, // 0 - 1
    object: Object,

    body: Object,
    health_bar: Object,
}

impl Paddle {
    pub fn new(layer: &Arc<Layer>) -> Result<Self> {
        let mut object = NewObject::new();
        let mut body = NewObject::new();
        let mut health_bar = NewObject::new();

        object.transform = Transform {
            position: vec2(0.07, 0.0),
            size: vec2(1.0, 1.0),
            rotation: 0.0,
        };
        object.appearance.set_visible(false);

        let sampler = SamplerBuilder::default()
            .mag_filter(Filter::Nearest)
            .min_filter(Filter::Nearest)
            .build()?;

        let texture = Texture::from_bytes(
            &asset("textures/paddle/paddle.png")?,
            ImageFormat::Png,
            1,
            TextureSettings::default().srgb(false).sampler(sampler),
        )?;

        let material = Material::new_default_textured(&texture);

        body.appearance = Appearance::new()
            .model(Model::Square)
            .material(Some(material));
        body.appearance.auto_scale(HEIGHT)?;
        let transform = body.appearance.get_transform();
        let size = transform.size;
        body.appearance.set_transform(transform.size(size));

        health_bar.appearance = Appearance::new()
            .model(Model::Square)
            .transform(Transform::default().size(size - 0.0156))
            .color(Color::from_rgb(0.49, 0.886, 0.643));

        let object = object.init(layer)?;
        let health_bar = health_bar.init_with_parent(&object)?;
        let body = body.init_with_parent(&object)?;
        Ok(Self {
            health: 1.0,
            object,
            body,
            health_bar,
        })
    }

    pub fn update(&mut self, mouse_y: f32) {
        let object = &mut self.object;
        let x = object.transform.position.x;

        // object.transform.position.lerp(vec2(x, mouse_y), 0.12);
        object.transform.position = vec2(x, mouse_y).clamp(vec2(0.0, -0.9), vec2(1.0, 0.9));

        object.sync();
    }
}
