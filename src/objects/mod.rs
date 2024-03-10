use std::sync::Arc;

use crate::game::{GameSettings, Message};

use super::game::Layers;
use anyhow::Result;
use let_engine::prelude::*;
use rand::random;

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

    shaking: f32,
}

impl Camera {
    pub fn new(layer: &Arc<Layer>) -> Result<Self> {
        let mut object = NewObject::new();

        object.appearance = Appearance::new().visible(false);

        let object = object.init(layer)?;
        layer.set_camera(&object)?;
        Ok(Self {
            object,
            shaking: 0.0,
        })
    }

    pub fn shake(&mut self) {
        self.shaking = TIME.fps() as f32 * 0.1;
    }

    pub fn update(&mut self) {
        if let Some(window) = SETTINGS.window() {
            let scale = window.inner_size();
            let scaling = CameraScaling::KeepVertical.scale(scale);
            self.object.transform.position.x = scaling.x * 0.5;

            if self.shaking > 0.0 {
                self.object.transform.position.x += random::<f32>() * 0.05;
                self.object.layer().set_zoom(1.0 + random::<f32>() * 0.02)
            }
            self.shaking -= 1.0;

            self.object.sync().unwrap();
        }
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

    pub fn update(&mut self) -> Option<Message> {
        self.settings.update()
    }

    /// Updates all tick updated objects.
    pub fn tick_update(&mut self) {
        self.framerate_display.update();
    }
}
