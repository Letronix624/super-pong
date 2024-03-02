use std::time::Duration;

use crate::{
    objects::{
        button::{Button, Parent},
        fade::fade_in,
    },
    FONT_STINGRAY, HEIGHT,
};

use super::{Game, Layers, Message};
use anyhow::Result;
use let_engine::prelude::*;

pub struct MainMenu {
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
fn load_material(asset: &[u8], layers: u32) -> Option<Material> {
    let sampler = SamplerBuilder::default()
        .mag_filter(Filter::Nearest)
        .min_filter(Filter::Nearest)
        .build()
        .ok()?;
    let texure_settings = TextureSettings {
        srgb: true,
        sampler,
    };
    let texture =
        Texture::from_bytes(asset, ImageFormat::Png, layers, texure_settings.clone()).ok()?;
    Some(Material::new_default_textured(&texture))
}

impl MainMenu {
    pub fn load(layers: &Layers) -> Result<Self> {
        let appearance = Appearance::new()
            .material(load_material(
                &asset("textures/environment/backgrounds/bottom-backdrop.png")?,
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
                &asset("textures/environment/backgrounds/sky.png")?,
                1,
            ))
            .auto_scaled(HEIGHT)?;

        let background = NewObjectBuilder::default()
            .appearance(appearance)
            .transform(Transform::default().position(vec2(2.6, 0.0)))
            .build()?
            .init(&layers.main)?;

        layers.main.move_to_top(&bottom_backdrop)?;
        layers.main.move_to_top(&background)?;

        let appearance = Appearance::new()
            .material(load_material(&asset("textures/environment/birds.png")?, 2))
            .auto_scaled(HEIGHT)?;

        let birds = NewObjectBuilder::default()
            .transform(Transform::default().position(vec2(-0.5, -0.7)))
            .appearance(appearance)
            .build()?
            .init(&layers.main)?;

        let appearance = Appearance::new()
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

        let play_button = Button::new(
            Parent::Layer(&layers.main),
            load_material(&asset("textures/ui/button.png")?, 1),
            LabelCreateInfo::default()
                .text("Play")
                .scale(vec2(60.0, 60.0))
                .align(Direction::Center),
            Vec2::new(0.0, 0.3),
        )?;
        let settings_button = Button::new(
            Parent::Layer(&layers.main),
            load_material(&asset("textures/ui/button.png")?, 1),
            LabelCreateInfo::default()
                .text("Settings")
                .scale(vec2(60.0, 60.0))
                .align(Direction::Center),
            Vec2::new(0.0, 0.55),
        )?;
        let quit_button = Button::new(
            Parent::Layer(&layers.main),
            load_material(&asset("textures/ui/button.png")?, 1),
            LabelCreateInfo::default()
                .text("Quit")
                .scale(vec2(60.0, 60.0))
                .align(Direction::Center),
            Vec2::new(0.0, 0.8),
        )?;

        let version_number = Label::new(
            &FONT_STINGRAY,
            LabelCreateInfo::default()
                .text(env!("CARGO_PKG_VERSION"))
                .scale(vec2(30.0, 30.0))
                .transform(Transform::default())
                .align(Direction::Sw),
        )
        .init(&layers.ui)?;

        fade_in(Duration::from_secs(5), layers);

        Ok(Self {
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
        self.background.remove();
        self.bottom_backdrop.remove();
        self.rainbow.remove();
        self.title.remove();
        self.birds.remove();
        self.play_button.object.remove();
        self.settings_button.object.remove();
        self.quit_button.object.remove();
        self.version_number.object.remove();
    }

    pub fn update(&mut self, layers: &Layers) -> Option<Message> {
        let mut message = None;
        let middle = layers.main.side_to_world(Vec2::ZERO);

        self.title.transform.position.x = middle.x;
        self.play_button.object.transform.position.x = middle.x;
        self.settings_button.object.transform.position.x = middle.x;
        self.quit_button.object.transform.position.x = middle.x;

        let time = (TIME.time() / 10.0).sin() as f32;

        self.background.transform.position.x = 2.0 + time * 0.5;
        self.bottom_backdrop.transform.position.x = 2.1 + time;

        self.birds.transform.position.x += TIME.delta_time() as f32 * 0.07;
        self.birds
            .appearance
            .set_layer((TIME.time() % 1.0).round() as u32)
            .unwrap();

        if let Some(window) = SETTINGS.window() {
            self.version_number
                .object
                .appearance
                .get_transform_mut()
                .size = CameraScaling::Expand.scale(window.inner_size());
            self.version_number.sync();
        }

        self.play_button
            .on_release(|| message = Some(Message::SwitchScene(super::GameScene::Ingame)));
        self.settings_button.on_release(|| println!("Options"));
        self.quit_button.on_release(|| {
            message = Some(Message::Exit);
        });

        self.title.sync();
        self.background.sync();
        self.bottom_backdrop.sync();
        self.birds.sync();
        message
    }
}
