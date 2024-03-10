use std::time::Duration;

use crate::{
    objects::{
        button::{Button, Parent},
        fade::fade_in,
    },
    FONT_STINGRAY, HEIGHT,
};

use super::{load_material, Layers, Message};
use anyhow::Result;
use let_engine::prelude::*;

pub struct MainMenu {
    layers: Layers,
    background: Object,
    bottom_backdrop: Object,
    title: Object,
    birds: Object,
    rainbow: Object,
    play_button: Button,
    settings_button: Button,
    quit_button: Button,
    version_number: Label<Object>,
}

impl MainMenu {
    pub fn new(layers: &Layers) -> Result<Self> {
        let appearance = Appearance::new()
            .model(Some(Model::Square))
            .material(load_material(
                &asset("textures/environment/backgrounds/bottom_backdrop.png")?,
                1,
            ))
            .auto_scaled(HEIGHT)?;
        let bottom_backdrop = NewObjectBuilder::default()
            .transform(Transform::default().position(vec2(2.6, 0.7)))
            .appearance(appearance)
            .build()?
            .init(&layers.main)?;

        let appearance = Appearance::new()
            .material(load_material(
                &asset("textures/environment/backgrounds/title_sky.png")?,
                1,
            ))
            .model(Some(Model::Square))
            .auto_scaled(HEIGHT)?;

        let background = NewObjectBuilder::default()
            .appearance(appearance)
            .transform(Transform::default().position(vec2(2.6, 0.0)))
            .build()?
            .init(&layers.main)?;

        bottom_backdrop.move_to_top()?;
        background.move_to_top()?;

        let appearance = Appearance::new()
            .model(Some(Model::Square))
            .material(load_material(&asset("textures/environment/birds.png")?, 2))
            .auto_scaled(HEIGHT)?;

        let birds = NewObjectBuilder::default()
            .transform(Transform::default().position(vec2(-0.5, -0.7)))
            .appearance(appearance)
            .build()?
            .init(&layers.main)?;

        let appearance = Appearance::new()
            .model(Some(Model::Square))
            .material(load_material(&asset("textures/title.png")?, 1))
            .auto_scaled(HEIGHT)?;

        let title = NewObjectBuilder::default()
            .transform(
                Transform::default()
                    .position(layers.main.side_to_world(Vec2::ZERO) + vec2(0.0, -0.15)),
            )
            .appearance(appearance)
            .build()?
            .init(&layers.main)?;

        let apperance = Appearance::new()
            .model(Some(Model::Square))
            .material(load_material(
                &asset("textures/environment/rainbow.png")?,
                1,
            ))
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.9))
            .auto_scaled(HEIGHT)?;

        let rainbow = NewObjectBuilder::default()
            .transform(Transform::default())
            .appearance(apperance)
            .build()?
            .init_with_parent(&title)?;

        let button_material = load_material(&asset("textures/ui/button.png")?, 1);

        let play_button = Button::new(
            Parent::Layer(&layers.ui),
            button_material.clone(),
            Some(
                LabelCreateInfo::default()
                    .text("Play")
                    .scale(vec2(60.0, 60.0))
                    .align(Direction::Center),
            ),
            vec2(0.0, 0.3),
        )?;
        let settings_button = Button::new(
            Parent::Layer(&layers.ui),
            button_material.clone(),
            Some(
                LabelCreateInfo::default()
                    .text("Settings")
                    .scale(vec2(60.0, 60.0))
                    .align(Direction::Center),
            ),
            vec2(0.0, 0.55),
        )?;
        let quit_button = Button::new(
            Parent::Layer(&layers.ui),
            button_material,
            Some(
                LabelCreateInfo::default()
                    .text("Quit")
                    .scale(vec2(60.0, 60.0))
                    .align(Direction::Center),
            ),
            vec2(0.0, 0.8),
        )?;

        let version_number = Label::new(
            &FONT_STINGRAY,
            LabelCreateInfo::default()
                .text(env!("CARGO_PKG_VERSION"))
                .scale(vec2(30.0, 30.0))
                .transform(Transform::default().position(vec2(0.0, 0.0)))
                .align(Direction::Sw),
        )
        .init(&layers.ui)?;

        if let Some(window) = SETTINGS.window() {
            window.set_cursor_visible(true);
        }

        fade_in(Duration::from_secs(5), layers);

        Ok(Self {
            layers: layers.clone(),
            background,
            bottom_backdrop,
            birds,
            title,
            rainbow,
            play_button,
            settings_button,
            quit_button,
            version_number,
        })
    }

    pub fn unload(self) {
        let _ = self.background.remove();
        let _ = self.bottom_backdrop.remove();
        let _ = self.rainbow.remove();
        let _ = self.title.remove();
        let _ = self.birds.remove();
        self.play_button.remove();
        self.settings_button.remove();
        self.quit_button.remove();
        let _ = self.version_number.object.remove();
    }

    pub fn update(&mut self) -> Result<Option<Message>> {
        let mut message = None;
        let middle = self.layers.main.side_to_world(Vec2::ZERO);

        self.title.transform.position.x = middle.x;

        let time = (TIME.time() / 10.0).sin() as f32;

        self.background.transform.position.x = 2.0 + time * 0.5;
        self.bottom_backdrop.transform.position.x = 2.1 + time;

        self.birds.transform.position.x += TIME.delta_time() as f32 * 0.07;
        self.birds
            .appearance
            .set_layer((TIME.time() % 1.0).round() as u32)?;

        if let Some(window) = SETTINGS.window() {
            self.version_number
                .object
                .appearance
                .get_transform_mut()
                .size = CameraScaling::KeepVertical.scale(window.inner_size());
            self.version_number.sync();
        }

        self.play_button
            .on_release(|| message = Some(Message::SwitchScene(super::GameScene::Ingame)));
        self.settings_button
            .on_release(|| message = Some(Message::ShowSettings(true)));
        self.quit_button.on_release(|| {
            message = Some(Message::Exit);
        });

        self.title.sync()?;
        self.background.sync()?;
        self.bottom_backdrop.sync()?;
        self.birds.sync()?;

        Ok(message)
    }
}
