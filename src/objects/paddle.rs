use std::sync::Arc;

use anyhow::Result;
/// The player seen on the left side of the screen.
use let_engine::prelude::*;

#[derive(Clone, Debug)]
pub struct Paddle {
    object: Object,
}

impl Paddle {
    pub fn new(layer: &Arc<Layer>) -> Result<Self> {
        let mut object = NewObject::new();

        let transform = Transform {
            position: layer.side_to_world(vec2(-1.0, 0.0) + vec2(0.07, 0.0)),
            size: vec2(0.03, 0.1),
            rotation: 0.0,
        };

        object.appearance = Appearance::new()
            .visible(true)
            .transform(transform)
            .model(Model::Square);

        let object = object.init(layer)?;
        Ok(Self { object })
    }

    pub fn update(&mut self, mouse_y: f32) {
        let object = &mut self.object;
        let x = object.transform.position.x;

        object.transform.position = vec2(x, mouse_y); // object.transform.position.lerp(vec2(x, mouse_y), 0.12);

        object.sync();
    }
}
