use anyhow::Result;
use let_engine::prelude::*;

use crate::{game::Layers, FONT_STINGRAY};

#[derive(Clone, Debug)]
pub struct FramerateCounter {
    label: Label<Object>,
}

impl FramerateCounter {
    pub fn new(layers: &Layers) -> Result<Self> {
        let label = Label::new(
            &FONT_STINGRAY,
            LabelCreateInfo::default()
                .text(TIME.fps().to_string())
                .scale(vec2(20.0, 20.0))
                .align(Direction::Nw),
        )
        .init(&layers.ui)?;
        Ok(Self { label })
    }

    pub fn update(&mut self) {
        if let Some(window) = SETTINGS.window() {
            self.label.object.appearance.set_transform(
                Transform::default()
                    .size(CameraScaling::KeepVertical.scale(window.inner_size()))
                    .position(vec2(0.0, 0.0)),
            );
        }
        self.label
            .update_text((TIME.fps().round() as u32).to_string());
    }
}
