use std::{f32::consts::PI, sync::Arc};

use anyhow::Result;
/// The player seen on the left side of the screen.
use let_engine::prelude::*;

use crate::HEIGHT;

#[derive(Clone, Debug)]
pub struct Paddle {
    pub health: f32,
    max_health: f32,

    pub delta: Vec2,
    pub object: Object,
    pub arrow: Object,
    pub cursor: Object,

    pub body: Object,
    pub health_bar: Object,
}

const ARROW: ([Vertex; 4], [u32; 6]) = (
    [
        vert(0.0, -0.1),
        vert(0.0, -0.5),
        vert(0.04, -0.46),
        vert(-0.04, -0.46),
    ],
    [0, 1, 1, 2, 1, 3],
);

impl Paddle {
    pub fn new(layer: &Arc<Layer>) -> Result<Self> {
        let mut object = NewObject::new();
        let mut body = NewObject::new();
        let mut health_bar = NewObject::new();
        let mut arrow = NewObject::new();
        let mut cursor = NewObject::new();

        object.transform = Transform {
            position: vec2(0.07, 0.0),
            size: vec2(1.0, 1.0),
            rotation: 0.0,
        };

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
            .model(Some(Model::Square))
            .material(Some(material));
        body.appearance.auto_scale(HEIGHT)?;
        let transform = body.appearance.get_transform();
        let size = transform.size;
        body.appearance.set_transform(transform.size(size));

        object.set_collider(Some(
            ColliderBuilder::new(Shape::square(size.x, size.y)).build(),
        ));

        health_bar.appearance = Appearance::new()
            .model(Some(Model::Square))
            .transform(Transform::default().size(size - 0.0156))
            .color(Color::from_rgb(0.49, 0.886, 0.643));

        let arrow_model = Model::Custom(ModelData::new(Data::Fixed {
            vertices: &ARROW.0,
            indices: &ARROW.1,
        })?);

        arrow.appearance = Appearance::new()
            .model(Some(arrow_model))
            .material(
                Material::new(
                    MaterialSettingsBuilder::default()
                        .topology(Topology::LineList)
                        .line_width(16.0)
                        .build()?,
                )
                .ok(),
            )
            .color(Color::from_r(0.5));
        arrow.transform.rotation = PI / 2.0;

        cursor.appearance = Appearance::new()
            .model(Some(Model::Square))
            .transform(Transform::default().size(vec2(0.04, 0.04)))
            .color(Color::from_rgba(
                10.0 / 255.0,
                22.0 / 255.0,
                48.0 / 255.0,
                0.7,
            ));

        let object = object.init(layer)?;
        let arrow = arrow.init_with_parent(&object)?;
        let health_bar = health_bar.init_with_parent(&object)?;
        let body = body.init_with_parent(&object)?;
        let cursor = cursor.init_with_parent(&object)?;
        Ok(Self {
            health: 3.0,
            max_health: 3.0,
            delta: Vec2::ZERO,
            object,
            arrow,
            cursor,
            body,
            health_bar,
        })
    }

    pub fn damage(&mut self, damage: f32) {
        self.health -= damage;
        let body_size = self.body.appearance.get_transform().size - 0.0156;
        let size = body_size * vec2(1.0, self.health / self.max_health);
        self.health_bar.appearance.get_transform_mut().size = size;
        self.health_bar.appearance.get_transform_mut().position.y = -size.y + body_size.y;
        self.health_bar
            .appearance
            .set_color(Color::from_rgb(0.5, 0.04, 0.05).lerp(
                Color::from_rgb(0.49, 0.886, 0.643),
                self.health / self.max_health,
            ));
        self.health_bar.sync().unwrap();
    }

    pub fn rebound_direction(&self) -> Vec2 {
        Vec2::from_angle(self.arrow.transform.rotation - PI * 0.5)
    }

    pub fn update(&mut self) {
        let mouse_delta = std::mem::take(&mut self.delta);
        let delta_time = TIME.delta_time() as f32;

        let mut position =
            (self.cursor.transform.position + mouse_delta * 0.003).clamp_length_max(0.4);

        position = position.lerp(Vec2::ZERO, delta_time * 5.0) * vec2(0.7, 1.0);

        self.cursor.transform.position = position;
        self.cursor
            .appearance
            .get_color_mut()
            .set_a(position.length() * 2.0);

        let object = &mut self.object;

        object.transform.position = (object.transform.position
            + vec2(0.0, position.y) * delta_time * 6.0)
            .clamp(vec2(0.0, -0.9), vec2(1.0, 0.9));

        let rotation = &mut self.arrow.transform.rotation;
        let delta_time = delta_time * 3.0;
        *rotation -= (INPUT.mouse_down(&MouseButton::Left) as u8) as f32 * delta_time;
        *rotation += (INPUT.mouse_down(&MouseButton::Right) as u8) as f32 * delta_time;
        *rotation = rotation.clamp(PI * 0.25, PI * 0.75);

        self.arrow.sync().unwrap();
        self.cursor.sync().unwrap();
        object.sync().unwrap();
    }

    pub fn unload(self) {
        self.object.remove().unwrap();
    }
}
