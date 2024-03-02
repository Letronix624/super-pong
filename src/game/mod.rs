use super::objects::Objects;
use anyhow::{anyhow, Result};
use let_engine::prelude::*;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

mod game_loop;
mod main_menu;

#[derive(Default, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct GameState {
    pub stage: u32,
}

impl GameState {
    pub fn data_dir() -> Result<PathBuf> {
        use std::fs;
        let project_dir = directories::ProjectDirs::from("net", "Let", "SuperPong")
            .ok_or(anyhow!("Failed to get project directory."))?;

        let data_dir = project_dir.data_dir();

        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
        };
        Ok(data_dir.to_path_buf())
    }

    pub fn load() -> Result<Self> {
        use std::fs;
        let data_dir = Self::data_dir()?;

        let file = fs::read(data_dir.join("state.sav"))?;

        Ok(bincode::deserialize(&file)?)
    }

    pub fn save(&self) -> Result<()> {
        use std::fs;
        let data_dir = Self::data_dir()?;

        let data = bincode::serialize(&self)?;

        Ok(fs::write(data_dir.join("state.sav"), data)?)
    }
}

#[derive(Default, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Difficulty {
    Normal,
    #[default]
    Hard,
}

#[derive(Default, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct GameSettings {
    difficulty: Difficulty,
    vsync: bool,
    fps_limit: u32,
    particle_high: bool,
    screen_shake: u32,
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
    state: GameState,
    settings: GameSettings,

    scene: Scene,

    exit: bool,
}

impl Game {
    pub fn new(state: GameState, settings: GameSettings) -> Result<Self> {
        let layers = Layers::new();
        Ok(Self {
            objects: Objects::new(&layers),
            state,
            // Start with menu scene
            scene: Scene::Menu(main_menu::MainMenu::load(&layers)?),
            settings,
            layers,
            exit: false,
        })
    }

    pub fn switch_scene(&mut self, scene: GameScene) -> Result<()> {
        // First load the new scene and replace the old one
        let scene = match &scene {
            GameScene::Menu => {
                let scene = Scene::Menu(main_menu::MainMenu::load(&self.layers)?);
                std::mem::replace(&mut self.scene, scene)
            }
            GameScene::Ingame => {
                let scene = Scene::Ingame(game_loop::Loop::load(&self.layers)?);
                std::mem::replace(&mut self.scene, scene)
            }
        };

        // then unload the old scene.
        match scene {
            Scene::Menu(menu) => menu.unload(),
            Scene::Ingame(game_loop) => game_loop.unload(),
        }

        Ok(())
    }
}

impl let_engine::Game for Game {
    fn exit(&self) -> bool {
        self.exit
    }
    fn update(&mut self) {
        self.objects.update();
        match self.scene.update(&self.layers) {
            Some(Message::Exit) => self.exit = true,
            Some(Message::SwitchScene(scene)) => self.switch_scene(scene).unwrap(),
            _ => (),
        };
    }
    fn event(&mut self, event: events::Event) {
        match event {
            Event::Input(InputEvent::KeyboardInput { input }) => {
                if let Some(code) = input.keycode {
                    match code {
                        // Fullscreen on F11
                        VirtualKeyCode::F11 => {
                            if input.state == ElementState::Pressed {
                                if let Some(window) = SETTINGS.window() {
                                    window.set_fullscreen(!window.fullscreen());
                                }
                            }
                        }
                        // Stop on ESC
                        VirtualKeyCode::Escape => {
                            self.exit = true;
                        }
                        _ => (),
                    }
                }
            }
            Event::Window(WindowEvent::Resized(size)) => {
                if let Some(camera) = self.objects.camera.as_mut() {
                    camera.update(size.into());
                }
            }
            Event::Window(WindowEvent::CloseRequested) => {
                self.exit = true;
            }
            _ => (),
        }
    }
}

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
            mode: CameraScaling::Expand,
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
    pub fn update(&mut self, layers: &Layers) -> Option<Message> {
        match self {
            Self::Menu(menu) => menu.update(layers),
            Self::Ingame(game_loop) => game_loop.update(layers),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Message {
    Exit,
    SwitchScene(GameScene),
}
