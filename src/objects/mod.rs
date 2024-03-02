use std::sync::Arc;

use super::game::Layers;
use anyhow::Result;
use let_engine::prelude::*;

pub mod button;
pub mod fade;
pub mod paddle;
pub mod settings;

#[derive(Debug, Clone)]
pub struct Camera {
    object: Object,
}

impl Camera {
    pub fn new(layer: &Arc<Layer>) -> Result<Self> {
        let mut object = NewObject::new();

        object.appearance = Appearance::new().visible(false);

        let object = object.init(layer)?;
        layer.set_camera(&object)?;
        Ok(Self { object })
    }

    pub fn update(&mut self, width: [u32; 2]) {
        let scaling = CameraScaling::KeepVertical.scale(vec2(width[0] as f32, width[1] as f32));
        self.object.transform.position.x = scaling.x * 0.5;
        self.object.sync();
    }
}

#[derive(Clone, Debug)]
pub struct Objects {
    pub paddle: Option<paddle::Paddle>,
    pub camera: Option<Camera>,
}

impl Objects {
    pub fn new(layers: &Layers) -> Self {
        Self {
            paddle: paddle::Paddle::new(&layers.main).ok(),
            camera: Camera::new(&layers.main).ok(),
        }
    }

    /// Updates all objects
    pub fn update(&mut self) {
        if let Some(paddle) = self.paddle.as_mut() {
            paddle.update(INPUT.cursor_position().y);
        }
    }
}
