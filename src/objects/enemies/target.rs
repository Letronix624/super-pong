use once_cell::sync::Lazy;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use let_engine::prelude::*;
use rand::random;

use crate::{
    game::{load_material, sounds::Sounds, SAMPLER},
    objects::{particles::debris_particle, projectiles::ProjectileType},
    HEIGHT,
};

use super::{Enemy, EnemyMessage};

#[derive(Clone)]
pub struct Target {
    object: Option<Object>,
    position: Vec2,
    hp: f32,

    last_hit: Instant,
    last_shot: Instant,
    sounds: Sounds,
}

impl Target {
    pub fn new(layer: &Arc<Layer>, sounds: Sounds) -> Result<Self> {
        let material = load_material(&asset("textures/enemies/target/target.png")?, 2);
        let appearance = Appearance::new()
            .model(Some(Model::Square))
            .material(material)
            .auto_scaled(HEIGHT)?;

        let position: Vec2 = (rand::random::<Vec2>() + vec2(2.0, -0.7)) * vec2(1.0, 1.2);
        let transform = Transform::default().position(position + vec2(6.0, 0.0));

        let sizex = appearance.get_transform().size.x;

        let mut object = NewObjectBuilder::default()
            .appearance(appearance)
            .transform(transform)
            .build()?;

        object.set_collider(Some(ColliderBuilder::new(Shape::circle(sizex)).build()));

        let object = Some(object.init(layer)?);

        Ok(Self {
            object,
            position,
            hp: 2.0,
            last_hit: Instant::now(),
            last_shot: Instant::now(),
            sounds,
        })
    }

    /// Animates the target objects.
    fn animate(&mut self) {
        let time = TIME.time() as f32;
        let object = self.object.as_mut().unwrap();

        object.transform.position.y = self.position.y + time.cos() * 0.05;
        object.transform.position.x = object
            .transform
            .position
            .lerp(self.position, TIME.delta_time() as f32)
            .x;
    }
}

impl Enemy for Target {
    fn damage_if_id_right(&mut self, ids: &[usize], damage: f32) -> (f32, bool) {
        let mut hit = false;
        for id in ids {
            if id == self.object.as_ref().unwrap().id() {
                self.sounds.target_hit.play().unwrap();
                self.hp -= damage;
                self.last_hit = Instant::now();
                hit = true;
            }

            if self.hp <= 1.0 {
                self.object
                    .as_mut()
                    .unwrap()
                    .appearance
                    .set_layer(1)
                    .unwrap();
            }
        }
        (self.hp, hit)
    }
    fn update(&mut self) -> EnemyMessage {
        self.animate();
        let object = self.object.as_mut().unwrap();

        if self.last_hit.elapsed() < Duration::from_millis(200) {
            object.appearance.set_color(Color::from_rgb(2.0, 2.0, 2.0));
        } else {
            object.appearance.set_color(Color::from_rgb(1.0, 1.0, 1.0));
        }

        object.sync().unwrap();
        if self.last_shot.elapsed() > Duration::from_secs(5) {
            self.last_shot = Instant::now();
            let position = vec2(5.0, random::<f32>() - 0.5);
            let target = vec2(0.0, (random::<f32>() - 0.5) * 2.0);
            EnemyMessage::Shoot {
                projectile_type: ProjectileType::Square,
                direction: (target - position).normalize(),
                position,
            }
        } else {
            EnemyMessage::None
        }
    }
    fn remove(&mut self) {
        self.sounds.target_destroy.play().unwrap();
        let object = self.object.as_mut().unwrap();
        for _ in 0..3 {
            debris_particle(
                object.layer(),
                TARGETDEBRIS.clone(),
                object.transform.position,
                random::<Vec2>() - vec2(0.5, 0.0),
            );
        }
        let _ = std::mem::take(&mut self.object).unwrap().remove();
    }
}

static TARGETDEBRIS: Lazy<Appearance> = Lazy::new(|| {
    Appearance::new_instanced(
        Some(Model::Square),
        Some(Material::new_default_textured_instance(
            &Texture::from_bytes(
                &asset("textures/enemies/target/gib.png").unwrap(),
                ImageFormat::Png,
                1,
                TextureSettings::default().srgb(true).sampler(SAMPLER),
            )
            .unwrap(),
        )),
    )
    .auto_scaled(HEIGHT)
    .unwrap()
});
