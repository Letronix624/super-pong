use let_engine::prelude::*;
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{
    objects::{
        enemies::{Enemy, EnemyMessage, EnemyType},
        paddle::Paddle,
        particles::{Particle, ParticleMessage},
        projectiles::Projectile,
    },
    FONT_STINGRAY, HEIGHT,
};

use super::{load_material, stages::part_one::tutorial, Layers, Message, SAMPLER};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct GameState {
    pub stage: u32,
    pub kills: u32,
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

    pub fn load_or_init() -> Result<Self> {
        use std::fs;
        let data_dir = Self::data_dir()?;

        let file_path = data_dir.join("state.sav");
        if file_path.exists() {
            let file = fs::read(&file_path)?;

            let data = bincode::deserialize(&file);
            if let Err(error) = data {
                let new = file_path.with_extension("old");
                let error =
                    anyhow!("Could not deserialize game save. It might be corrupted:\n{error}");
                if native_dialog::MessageDialog::new()
                    .set_title("Load error")
                    .set_text(&format!("{error}\n Should we move the file to {new:?}?"))
                    .set_type(native_dialog::MessageType::Error)
                    .show_confirm()?
                {
                    fs::rename(&file_path, &new)?;
                    return Ok(Self::default());
                }
                return Err(error);
            }

            Ok(data?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        use std::fs;
        let data_dir = Self::data_dir()?;

        let data = bincode::serialize(&self)?;

        Ok(fs::write(data_dir.join("state.sav"), data)?)
    }
}

pub struct Loop {
    state: GameState,

    pub paddle: Paddle,

    title: Title,
    background: Background,
    level: Option<Level>,

    pub enemies: Vec<Box<dyn Enemy>>,
    pub projectiles: Vec<Box<dyn Projectile>>,
    pub particles: Vec<Box<dyn Particle>>,
}

impl Loop {
    pub fn new(layers: &Layers) -> Result<Self> {
        let state = GameState::load_or_init()?;
        let background = Background::new(&layers.main)?;
        let title = Title::new(&layers.ui)?;
        let level = Some(tutorial());
        if let Some(window) = SETTINGS.window() {
            let _ = window.set_cursor_grab(CursorGrabMode::Confined);
            window.set_cursor_visible(false);
        }

        Ok(Self {
            state,
            paddle: Paddle::new(&layers.main)?,
            title,
            background,
            level,
            enemies: vec![],
            projectiles: vec![],
            particles: vec![],
        })
    }

    pub fn unload(mut self) {
        self.background.unload();
        self.title.remove();
        self.paddle.unload();
        for enemy in &mut self.enemies {
            enemy.remove();
        }
        for projectile in &mut self.projectiles {
            projectile.remove();
        }
        for particle in &mut self.particles {
            particle.remove();
        }
    }

    pub fn update(&mut self) -> Result<Option<Message>> {
        self.paddle.update();

        if self.paddle.health <= 0.0 {
            return Ok(Some(Message::SwitchScene(super::GameScene::Menu)));
        }

        if let Some(level) = self.level.as_mut() {
            let message = level.progress()?;

            match message {
                LevelMessage::SpawnEnemy(enemy) => {
                    self.enemies.push(enemy.spawn(self.background.sky.layer())?);
                }
                LevelMessage::ShowTitle { color, size, text } => {
                    self.title.set_color(color);
                    self.title.set_size(size);
                    self.title.set_text(&text);
                    self.title.update()?;
                    self.title.fade_in_out();
                }
                LevelMessage::Done => {
                    self.state.stage += 1;
                    self.state.save()?;
                    return Ok(Some(Message::SwitchScene(super::GameScene::Menu)));
                }
                _ => (),
            }
        }
        self.title.update()?;

        self.projectiles.retain_mut(|projectile| {
            if projectile.age().elapsed() > Duration::from_secs(20) {
                projectile.remove();
                return false;
            }

            let touching = projectile.touching();
            if projectile.friendly() {
                let mut hit = false;
                self.enemies.retain_mut(|enemy| {
                    let touch = enemy.damage_if_id_right(&touching, projectile.damage());
                    if touch.1 {
                        hit = true;
                        if touch.0 <= 0.0 {
                            // kill
                            if let Some(level) = self.level.as_mut() {
                                level.kill();
                                self.state.kills += 1;
                            }
                            enemy.remove();
                            false
                        } else {
                            // damage
                            true
                        }
                    } else {
                        true
                    }
                });
                if hit {
                    projectile.remove();
                    return false;
                }
            } else {
                // send it to the arrow
                let position = projectile.position();
                if position.x < 0.0 {
                    self.paddle.damage(projectile.damage());
                    projectile.remove();
                    return false;
                }

                if touching.contains(self.paddle.object.id()) {
                    if position.x < 0.1 {
                        // hard shot
                        projectile.damage_multiplier(2.0);
                        projectile.rebound(self.paddle.rebound_direction() * 2.0);
                    } else {
                        projectile.rebound(self.paddle.rebound_direction());
                    }
                }
            }
            projectile.update();

            true
        });

        for enemy in &mut self.enemies {
            let message = enemy.update();

            match message {
                EnemyMessage::None => (),
                EnemyMessage::Shoot {
                    projectile_type,
                    position,
                    direction,
                } => {
                    let projectile =
                        projectile_type.spawn(self.background.sky.layer(), position, direction)?;
                    self.projectiles.push(projectile);
                }
                EnemyMessage::Particle => (),
            }
        }

        for particle in &mut self.particles {
            let message = particle.update();

            match message {
                ParticleMessage::None => (),
                ParticleMessage::Done => (// delete
                    ),
            }
        }

        self.background.update()?;

        Ok(None)
    }
}

pub struct Level {
    enemy_limit: u32,
    enemies: u32,
    event_duration: Duration,
    last_event: Instant,
    events: VecDeque<LevelMessage>,
}

impl Level {
    pub fn new(enemy_limit: u32, event_duration: Duration, events: VecDeque<LevelMessage>) -> Self {
        Self {
            enemy_limit,
            enemies: 0,
            event_duration,
            last_event: Instant::now(),
            events,
        }
    }

    /// Progresses the current stage.
    pub fn progress(&mut self) -> Result<LevelMessage> {
        if self.enemies < self.enemy_limit && self.last_event.elapsed() > self.event_duration {
            if let Some(message) = self.events.pop_front() {
                self.last_event = Instant::now();
                match message {
                    LevelMessage::SpawnEnemy(_) => self.enemies += 1,
                    LevelMessage::ChangeWaitingTime(duration) => self.event_duration = duration,
                    _ => (),
                }

                Ok(message)
            } else if self.enemies == 0 {
                Ok(LevelMessage::Done)
            } else {
                Ok(LevelMessage::None)
            }
        } else {
            Ok(LevelMessage::None)
        }
    }

    pub fn kill(&mut self) {
        self.enemies -= 1;
    }
}

pub enum LevelMessage {
    None,
    Done,
    SpawnEnemy(EnemyType),
    ChangeWaitingTime(Duration),
    ShowTitle {
        color: Color,
        size: Vec2,
        text: String,
    },
}

struct Title {
    back_label: Label<Object>,
    label: Label<Object>,
}

impl Title {
    pub fn new(layer: &Arc<Layer>) -> Result<Self> {
        let appearance = Appearance::new()
            .color(Color::BLACK)
            .transform(Transform::default().size(vec2(0.55, 0.7)));

        let back_label = Label::new(
            &FONT_STINGRAY,
            LabelCreateInfo::default()
                .align(Direction::N)
                .transform(Transform::default().position(vec2(0.01, 0.01)))
                .appearance(appearance.clone())
                .scale(Vec2::splat(70.0)),
        )
        .init(layer)?;

        let appearance = appearance.color(Color::WHITE);

        let label = Label::new(
            &FONT_STINGRAY,
            LabelCreateInfo::default()
                .align(Direction::N)
                .appearance(appearance)
                .scale(Vec2::splat(70.0)),
        )
        .init(layer)?;

        Ok(Self { back_label, label })
    }

    pub fn update(&mut self) -> Result<()> {
        self.back_label.object.update()?;
        self.label.object.update()?;
        self.back_label.sync();
        self.label.sync();
        Ok(())
    }

    pub fn remove(self) {
        let _ = self.back_label.object.remove();
        let _ = self.label.object.remove();
    }

    pub fn set_text(&mut self, text: &str) {
        self.back_label.update_text(text);
        self.label.update_text(text);
        self.label.object.sync().unwrap();
    }

    pub fn set_color(&mut self, color: Color) {
        self.label.object.appearance.set_color(color);
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.back_label.scale = size;
        self.label.scale = size;
    }

    pub fn fade_in_out(&mut self) {
        let mut back_label = self.back_label.clone();
        let mut label = self.label.clone();
        let duration = Duration::from_secs(1);
        std::thread::spawn(move || {
            let now = Instant::now();
            let start_time = now.elapsed();
            let time = duration.as_secs_f32();
            while now.elapsed() <= duration {
                let elapsed = now.elapsed().as_secs_f32();
                let percentage = 1.0 - (elapsed - time) / (start_time.as_secs_f32() - time);
                let _ = back_label.update();
                let _ = label.update();
                back_label
                    .object
                    .appearance
                    .get_color_mut()
                    .set_a(percentage);
                label.object.appearance.get_color_mut().set_a(percentage);
                let _ = back_label.object.sync();
                let _ = label.object.sync();
            }
            std::thread::sleep(duration * 2);
            let now = Instant::now();
            let start_time = now.elapsed();
            let time = duration.as_secs_f32();
            while now.elapsed() <= duration {
                let elapsed = now.elapsed().as_secs_f32();
                let percentage = (elapsed - time) / (start_time.as_secs_f32() - time);
                let _ = back_label.update();
                let _ = label.update();
                back_label
                    .object
                    .appearance
                    .get_color_mut()
                    .set_a(percentage);
                label.object.appearance.get_color_mut().set_a(percentage);
                let _ = back_label.object.sync();
                let _ = label.object.sync();
            }
        });
    }
}

struct Background {
    sky: Object,
    sun: Object,
    far_clouds: Object,
    hills: Object,
    clouds: Object,
    terrain: Object,
}

const BACKGROUND_MODEL: ([Vertex; 4], [u32; 6]) = (
    [
        vert(-4.0, 1.0),
        vert(-4.0, -1.0),
        vert(4.0, 1.0),
        vert(4.0, -1.0),
    ],
    [0, 1, 2, 1, 2, 3],
);

impl Background {
    pub fn new(layer: &Arc<Layer>) -> Result<Self> {
        let texture = Texture::from_bytes(
            &asset("textures/environment/backgrounds/background1.png")?,
            ImageFormat::Png,
            5,
            TextureSettings {
                srgb: true,
                sampler: SAMPLER,
            },
        )?;

        let material = Material::new_default_textured(&texture);

        let model = Model::Custom(ModelData::new(Data::Fixed {
            vertices: &BACKGROUND_MODEL.0,
            indices: &BACKGROUND_MODEL.1,
        })?);

        let mut appearance = Appearance::new()
            .model(Some(model))
            .material(Some(material))
            .auto_scaled(HEIGHT)?;

        let position =
            Transform::default().position(vec2(appearance.get_transform().size.x * 4.0, 0.0));

        let terrain = NewObjectBuilder::default()
            .transform(position)
            .appearance(appearance.clone())
            .build()?;
        appearance.next_frame()?;
        let clouds = NewObjectBuilder::default()
            .transform(position)
            .appearance(appearance.clone())
            .build()?;
        appearance.next_frame()?;
        let hills = NewObjectBuilder::default()
            .transform(position)
            .appearance(appearance.clone())
            .build()?;
        appearance.next_frame()?;
        let far_clouds = NewObjectBuilder::default()
            .transform(position)
            .appearance(appearance.clone())
            .build()?;
        let sun = NewObjectBuilder::default()
            .transform(Transform::default().position(vec2(0.8, -0.7)))
            .appearance(
                Appearance::new()
                    .model(Some(Model::Square))
                    .material(load_material(
                        &asset("textures/environment/sun-moon.png")?,
                        2,
                    ))
                    .auto_scaled(HEIGHT)?,
            )
            .build()?;
        appearance.next_frame()?;
        let sky = NewObjectBuilder::default()
            .transform(position)
            .appearance(appearance.clone())
            .build()?;

        let sky = sky.init(layer)?;
        let sun = sun.init(layer)?;
        let far_clouds = far_clouds.init(layer)?;
        let hills = hills.init(layer)?;
        let clouds = clouds.init(layer)?;
        let terrain = terrain.init(layer)?;
        terrain.move_to_top()?;
        clouds.move_to_top()?;
        hills.move_to_top()?;
        far_clouds.move_to_top()?;
        sun.move_to_top()?;
        sky.move_to_top()?;

        Ok(Self {
            sky,
            sun,
            far_clouds,
            hills,
            clouds,
            terrain,
        })
    }

    pub fn unload(self) {
        let _ = self.sky.remove();
        let _ = self.sun.remove();
        let _ = self.far_clouds.remove();
        let _ = self.hills.remove();
        let _ = self.clouds.remove();
        let _ = self.terrain.remove();
    }

    pub fn update(&mut self) -> Result<()> {
        let size_x = self.sky.appearance.get_transform().size.x * 2.0;
        let time = TIME.time() as f32 * 0.15;

        self.far_clouds.transform.position.x = (size_x - time) % size_x;
        self.hills.transform.position.x = (size_x - time * 1.3) % size_x;
        self.clouds.transform.position.x = (size_x - time * 1.5) % size_x;
        self.terrain.transform.position.x = (size_x - time * 2.0) % size_x;

        self.sky.sync()?;
        self.sun.sync()?;
        self.far_clouds.sync()?;
        self.hills.sync()?;
        self.clouds.sync()?;
        self.terrain.sync()?;
        Ok(())
    }
}
