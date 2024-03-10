use anyhow::Result;
use let_engine::prelude::*;
use std::sync::Arc;

use crate::game::sounds::Sounds;

use super::projectiles::ProjectileType;

pub mod target;

pub trait Enemy: Send + Sync {
    fn damage_if_id_right(&mut self, ids: &[usize], damage: f32) -> (f32, bool);
    fn update(&mut self) -> EnemyMessage;
    fn remove(&mut self);
}

pub enum EnemyMessage {
    None,
    Shoot {
        projectile_type: ProjectileType,
        position: Vec2,
        direction: Vec2,
    },
    Particle, //(Vec<Box<dyn Particles>>),
}

pub enum EnemyType {
    Target,
    Fairy,
    Bird,
    Pegasus,
    Griffin,
    Bat,
    Harpy,
    Dragon,
    Vampire,
    Lindworm,
    Drone,
    AndroidPegasus,
    AndroidGriffin,
    Gimp,
    Chimere,
    MetalUnicorn,
    Spirit,
    Devil,
    Flesh,
    Death,
    MoreFlesh,
    BloodGoop,
    ChunkyFlesh,
    Abomination,
    FleshBoss,
}

impl EnemyType {
    pub fn spawn(&self, layer: &Arc<Layer>, sounds: &Sounds) -> Result<Box<dyn Enemy>> {
        Ok(match self {
            Self::Target => Box::new(target::Target::new(layer, sounds.clone())?),
            _ => todo!(),
        })
    }
}
