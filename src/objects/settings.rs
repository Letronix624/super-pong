use std::sync::Arc;

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

pub struct GameMenu {
    background: Object,
    resume: Button,
    options: Button,
    main_menu: Button,
}

impl GameMenu {
    pub fn new(layer: &Arc<Layer>) -> Result<Self> {
        let background = NewObjectBuilder::default()
            .appearance(
                Appearance::default()
                    .model(Some(Model::Square))
                    .transform(Transform::default().size(vec2(10.0, 1.0)))
                    .color(Color::from_rgba(0.0, 0.0, 0.0, 0.9)),
            )
            .build()?
            .init(layer)?;

        let resume = gm_button(&background, "Resume", vec2(0.0, 0.0))?;

        let options = gm_button(&background, "Options", vec2(0.0, 0.4))?;

        let main_menu = gm_button(&background, "Main Menu", vec2(0.0, 0.8))?;

        Ok(Self {
            background,
            resume,
            options,
            main_menu,
        })
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.background.appearance.set_visible(visible);
        self.resume.object.appearance.set_visible(visible);
        self.options.object.appearance.set_visible(visible);
        self.main_menu.object.appearance.set_visible(visible);
        if let Some(window) = SETTINGS.window() {
            window.set_cursor_visible(visible);
            let _ = window.set_cursor_grab(if visible {
                CursorGrabMode::None
            } else {
                CursorGrabMode::Confined
            });
        }
    }

    pub fn toggle(&mut self) {
        self.set_visible(!self.background.appearance.get_visible());
        self.set_enabled(!self.resume.object.collider().unwrap().is_enabled());
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.resume.set_enabled(enabled);
        self.options.set_enabled(enabled);
        self.main_menu.set_enabled(enabled);
        if enabled {
            TIME.set_scale(0.0);
        } else {
            TIME.set_scale(1.0);
        }
    }

    pub fn remove(self) {
        let _ = self.background.remove();
        self.resume.remove();
        self.options.remove();
        self.main_menu.remove();
    }

    pub fn update(&mut self) -> Result<Option<Message>> {
        let mut message = None;
        let mut visibility = false;
        self.resume.on_release(|| visibility = true);
        if visibility {
            self.set_visible(false);
            self.set_enabled(false);
        }
        self.options
            .on_release(|| message = Some(Message::ShowSettings(true)));
        self.main_menu
            .on_release(|| message = Some(Message::SwitchScene(crate::game::GameScene::Menu)));

        self.background.sync()?;
        Ok(message)
    }
}

fn gm_button(background: &Object, text: &str, position: Vec2) -> Result<Button> {
    Button::new(
        super::button::Parent::Object(background),
        load_material(&asset("textures/ui/button.png")?, 1),
        Some(
            LabelCreateInfo::default()
                .text(text)
                .align(Direction::Center)
                .scale(vec2(60.0, 60.0)),
        ),
        position,
    )
}
