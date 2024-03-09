use crate::{
    game::{load_material, GameSettings, Layers, Message},
    FONT_RAWR,
};
use anyhow::Result;
use let_engine::prelude::*;

use super::button::Button;

#[derive(Clone, Debug)]
pub struct Settings {
    visible: bool,
    pub settings: GameSettings,

    panel: Object,
    label: Label<Object>,
    settings_labels: Label<Object>,
    back_button: Button,
}

impl Settings {
    pub fn new(layers: &Layers, settings: GameSettings) -> Result<Self> {
        let appearance = Appearance::new()
            .material(load_material(&asset("textures/ui/settings.png")?, 1))
            .visible(false)
            .model(Some(Model::Square))
            .color(Color::from_rgba(1.2, 1.2, 1.2, 0.99))
            .auto_scaled(256.0)?;

        let panel = NewObjectBuilder::default()
            .transform(Transform::default())
            .appearance(appearance)
            .build()?;
        let panel = panel.init(&layers.ui)?;

        let transform = Transform::default().size(vec2(0.8, 0.73));

        let label = Label::new(
            &FONT_RAWR,
            LabelCreateInfo::default()
                .appearance(Appearance::default())
                .transform(transform)
                .text("Settings")
                .scale(vec2(80.0, 80.0)),
        )
        .init_with_parent(&panel)?;

        let settings_labels = Label::new(
            &FONT_RAWR,
            LabelCreateInfo::default()
                .appearance(Appearance::default())
                .transform(Transform::default().size(panel.transform.size * vec2(0.92, 0.83)))
                .align(Direction::W)
                .text(
                    "
Vsync

Fullscreen

FPS limit

Screen shake

Particle Amount

Difficulty
                    ",
                )
                .scale(vec2(47.0, 47.0)),
        )
        .init_with_parent(&panel)?;

        let back_button = Button::new(
            super::button::Parent::Object(&panel),
            load_material(&asset("textures/ui/back.png")?, 1),
            None,
            vec2(0.8, 0.72),
        )?;

        Ok(Self {
            settings,
            visible: false,
            panel,
            label,
            settings_labels,
            back_button,
            // fullscreen,
        })
    }

    pub fn show(&mut self, show: bool) {
        self.visible = show;
        self.panel.appearance.set_visible(show);
        let _ = self.panel.move_to_bottom();
        if let Some(window) = SETTINGS.window() {
            window.set_cursor_visible(true);
        }
    }

    pub fn toggle(&mut self) {
        self.show(!self.visible);
    }

    pub fn update(&mut self) -> Option<Message> {
        let mut message = None;
        self.panel.sync().unwrap();
        self.label.sync();
        self.settings_labels.sync();
        self.back_button
            .on_press(|| message = Some(Message::ShowSettings(false)));

        message
    }
}
