use std::sync::Arc;

use crate::game::{GameSettings, Message};

use super::game::Layers;
use anyhow::Result;
use let_engine::prelude::*;

pub mod button;
pub mod fade;
pub mod framerate_counter;
pub mod paddle;
pub mod settings;

pub mod enemies;
pub mod particles;
pub mod projectiles;

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
        self.object.sync().unwrap();
    }
}

pub struct Objects {
    pub camera: Camera,
    pub settings: settings::Settings,
    framerate_display: framerate_counter::FramerateCounter,
}

impl Objects {
    pub fn new(layers: &Layers, settings: GameSettings) -> Result<Self> {
        Ok(Self {
            camera: Camera::new(&layers.main)?,
            settings: settings::Settings::new(layers, settings)?,
            framerate_display: framerate_counter::FramerateCounter::new(layers)?,
        })
    }

    /// Updates all tick updated objects.
    pub fn tick_update(&mut self) -> Option<Message> {
        let message = self.settings.update();
        self.framerate_display.update();
        message
    }
}
