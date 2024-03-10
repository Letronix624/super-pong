use self::console::Console;

use super::objects::Objects;
use anyhow::{anyhow, Result};
use let_engine::prelude::*;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

mod console;
mod game_loop;
mod main_menu;
pub mod sounds;
pub mod stages;

pub const SAMPLER: Sampler = Sampler {
    mag_filter: Filter::Nearest,
    min_filter: Filter::Nearest,
    mipmap_mode: Filter::Nearest,
    address_mode: [
        AddressMode::Repeat,
        AddressMode::ClampToEdge,
        AddressMode::ClampToEdge,
    ],
    border_color: BorderColor::FloatTransparentBlack,
};

#[derive(Default, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Difficulty {
    Normal,
    #[default]
    Hard,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct GameSettings {
    pub difficulty: Difficulty,
    pub vsync: bool,
    pub fps_limit: u32,
    pub fullscreen: Fullscreen,
    pub resolution: Vec2,
    pub particle_high: bool,
    pub screen_shake: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Hard,
            vsync: true,
            fullscreen: Fullscreen::Exclusive,
            fps_limit: 0,
            resolution: vec2(455.0, 256.0),
            particle_high: true,
            screen_shake: 100,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Fullscreen {
    Windowed,
    Borderless,
    Exclusive,
}

impl Fullscreen {
    pub fn apply(&self) -> Result<()> {
        if let Some(window) = SETTINGS.window() {
            let Some(window_mode) = window.currect_monitor() else {
                return Ok(());
            };
            let window_mode = window_mode.video_modes();

            let Some(video_mode) = window_mode.first() else {
                return Err(anyhow!("Failed to find any video mode."));
            };
            match self {
                Fullscreen::Windowed => {
                    window.set_fullscreen(None);
                }
                Fullscreen::Borderless => {
                    window.set_fullscreen(Some(let_engine::prelude::Fullscreen::Borderless(None)))
                }
                Fullscreen::Exclusive => window.set_fullscreen(Some(
                    let_engine::window::Fullscreen::Exclusive(video_mode.clone()),
                )),
            }
        }
        Ok(())
    }
}

impl GameSettings {
    pub fn config_dir() -> Result<PathBuf> {
        use std::fs;
        let project_dir = directories::ProjectDirs::from("net", "Let", "SuperPong")
            .ok_or(anyhow!("Failed to get project directory."))?;

        let config_dir = project_dir.config_dir();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        };
        Ok(config_dir.to_path_buf())
    }

    pub fn load() -> Result<Self> {
        use std::fs;
        let config_dir = Self::config_dir()?;

        let file = fs::read(config_dir.join("config.ron"))?;
        let settings = ron::de::from_reader(file.as_slice())?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        use std::fs;
        let data_dir = Self::config_dir()?;

        let data = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::new())?;

        Ok(fs::write(data_dir.join("config.ron"), data)?)
    }
}

pub struct Game {
    layers: Layers,
    objects: Objects,
    console: Console,

    settings: GameSettings,

    scene: Scene,

    exit: bool,
}

impl Game {
    pub fn new(settings: GameSettings) -> Result<Self> {
        let layers = Layers::new();
        Ok(Self {
            objects: Objects::new(&layers, settings)?,
            console: Console::new(settings),
            // Start with menu scene
            scene: Scene::Menu(main_menu::MainMenu::new(&layers)?),
            settings,
            layers,
            exit: false,
        })
    }

    pub fn switch_scene(&mut self, scene: &GameScene) -> Result<()> {
        // First load the new scene and replace the old one
        let scene = match &scene {
            GameScene::Menu => {
                let scene = Scene::Menu(main_menu::MainMenu::new(&self.layers)?);
                std::mem::replace(&mut self.scene, scene)
            }
            GameScene::Ingame => {
                let scene = Scene::Ingame(game_loop::Loop::new(&self.layers)?);
                std::mem::replace(&mut self.scene, scene)
            }
        };

        // then unload the old scene.
        match scene {
            Scene::Menu(menu) => menu.unload(),
            Scene::Ingame(game_loop) => game_loop.unload(),
        }

        // Optimize memory performance
        SETTINGS.clean_caches();

        Ok(())
    }

    pub fn execute_message(&mut self, message: Message) {
        match message {
            Message::Exit => self.exit = true,
            Message::ShowSettings(show) => self.objects.settings.show(show),
            Message::SwitchScene(scene) => {
                if let Err(error) = self.switch_scene(&scene) {
                    self.console
                        .print(format!("Error: Could not switch scene.\n{error}"));
                }
            }
            Message::ApplySettings(settings) => {
                self.settings = settings;

                if settings.vsync {
                    if SETTINGS
                        .graphics
                        .set_present_mode(PresentMode::Mailbox)
                        .is_err()
                    {
                        SETTINGS
                            .graphics
                            .set_present_mode(PresentMode::Fifo)
                            .unwrap();
                    }
                } else if SETTINGS
                    .graphics
                    .set_present_mode(PresentMode::Immediate)
                    .is_err()
                {
                    self.settings.vsync = true;
                    self.console.print("Failed to disable vsync".to_string());
                    self.console.print(format!(
                        "This device only supports these present modes:\n{:?}",
                        SETTINGS.graphics.get_supported_present_modes()
                    ));
                };

                if let Err(e) = settings.fullscreen.apply() {
                    self.console.print(e.to_string());
                }

                SETTINGS.graphics.set_fps_cap(settings.fps_limit as u64);
                self.console.settings = self.settings;
                if let Err(error) = self.settings.save() {
                    self.console.print(format!("Failed to save game: {error}"));
                };
            }
        }
    }
}

impl let_engine::Game for Game {
    fn exit(&self) -> bool {
        self.exit
    }
    fn update(&mut self) {
        match self.scene.update() {
            Ok(Some(message)) => self.execute_message(message),
            Err(error) => crash("Failed to update scene", &error.to_string()),
            _ => (),
        }
    }
    fn tick(&mut self) {
        if let Some(message) = self.objects.tick_update() {
            self.execute_message(message);
        }
    }
    fn start(&mut self) {
        self.execute_message(Message::ApplySettings(self.settings));
    }
    fn event(&mut self, event: events::Event) {
        match event {
            Event::Egui(ctx) => {
                if let Some(message) = self.console.update(&ctx) {
                    self.execute_message(message);
                }
            }
            Event::Input(InputEvent::KeyboardInput { input }) => {
                if let Key::Named(code) = input.keycode {
                    match code {
                        // Show console
                        NamedKey::F7 => {
                            if input.state == ElementState::Released {
                                self.console.toggle();
                            }
                        }
                        // Show settings
                        NamedKey::Escape => {
                            if input.state == ElementState::Pressed {
                                self.objects.settings.toggle();
                            }
                        }
                        _ => (),
                    }
                }
            }
            Event::Input(InputEvent::MouseMotion(delta)) => {
                if let Scene::Ingame(game_loop) = &mut self.scene {
                    game_loop.paddle.delta = delta;
                }
            }
            Event::Window(WindowEvent::Resized(_)) => {
                self.objects.camera.update();
            }
            Event::Window(WindowEvent::CloseRequested) => {
                self.exit = true;
            }
            _ => (),
        }
    }
}

#[derive(Clone)]
pub struct Layers {
    pub main: Arc<Layer>,
    pub ui: Arc<Layer>,
}

impl Layers {
    pub fn new() -> Self {
        let main = SCENE.new_layer();
        main.set_camera_settings(CameraSettings {
            zoom: 1.0,
            mode: CameraScaling::KeepVertical,
        });
        let ui = SCENE.new_layer();
        ui.set_camera_settings(CameraSettings {
            zoom: 1.0,
            mode: CameraScaling::KeepVertical,
        });
        Self { main, ui }
    }
}

impl Default for Layers {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GameScene {
    #[default]
    Menu,
    Ingame,
}

pub enum Scene {
    Menu(main_menu::MainMenu),
    Ingame(game_loop::Loop),
}

impl Scene {
    pub fn update(&mut self) -> Result<Option<Message>> {
        match self {
            Self::Menu(menu) => menu.update(),
            Self::Ingame(game_loop) => game_loop.update(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Exit,
    ShowSettings(bool),
    SwitchScene(GameScene),
    ApplySettings(GameSettings),
}

pub fn load_material(asset: &[u8], layers: u32) -> Option<Material> {
    let texure_settings = TextureSettings {
        srgb: true,
        sampler: SAMPLER,
    };
    let texture =
        Texture::from_bytes(asset, ImageFormat::Png, layers, texure_settings.clone()).ok()?;
    Some(Material::new_default_textured(&texture))
}

fn crash(title: &str, error: &str) -> ! {
    native_dialog::MessageDialog::new()
        .set_title(title)
        .set_text(error)
        .set_type(native_dialog::MessageType::Error)
        .show_alert()
        .expect(error);
    panic!("{title}: {}", error);
}
