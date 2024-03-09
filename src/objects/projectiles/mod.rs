use std::{sync::Arc, time::Instant};

use anyhow::Result;
use let_engine::prelude::*;
use once_cell::sync::Lazy;

pub trait Projectile: Send + Sync {
    fn update(&mut self); // -> ProjectileMessage;
    fn touching(&self) -> Vec<usize>;
    fn position(&self) -> Vec2;
    fn rebound(&mut self, direction: Vec2);
    fn friendly(&self) -> bool;
    fn damage(&self) -> f32;
    fn damage_multiplier(&mut self, multiplier: f32);
    fn age(&self) -> Instant;
    fn remove(&mut self);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectileMessage {
    None,
    Hit(Vec<usize>),
}

pub enum ProjectileType {
    Square,
}

impl ProjectileType {
    pub fn spawn(
        &self,
        layer: &Arc<Layer>,
        position: Vec2,
        direction: Vec2,
    ) -> Result<Box<dyn Projectile>> {
        Ok(match self {
            Self::Square => Box::new(Square::new(layer, position, direction)?),
        })
    }
}

#[derive(Clone)]
pub struct Square {
    object: Option<Object>,
    direction: Vec2,
    age: Instant,
    friendly: bool,
    damage: f32,
}

static SQUARE: Lazy<Appearance> = Lazy::new(|| {
    Appearance::new_instanced(Some(Model::Square), None)
        .transform(Transform::default().size(vec2(0.02, 0.02)))
});

impl Square {
    pub fn new(layer: &Arc<Layer>, position: Vec2, direction: Vec2) -> Result<Self> {
        let object = NewObjectBuilder::default()
            .appearance(SQUARE.clone())
            .transform(Transform::default().position(position))
            .build()?;

        let object = Some(object.init(layer)?);

        Ok(Self {
            object,
            direction,
            age: Instant::now(),
            friendly: false,
            damage: 1.0,
        })
    }
}

impl Projectile for Square {
    fn update(&mut self) {
        // -> ProjectileMessage {
        let object = self.object.as_mut().unwrap();
        let position = object.transform.position;
        let half_size = object.appearance.get_transform().size.y * 0.5;
        if position.y > 1.0 - half_size {
            // Change direction to up
            self.direction.y = -self.direction.y.abs();
        } else if position.y < -1.0 + half_size {
            // Change direction to down
            self.direction.y = self.direction.y.abs();
        }

        object.transform.position += self.direction * TIME.delta_time() as f32;
        object.sync().unwrap();
    }

    fn rebound(&mut self, direction: Vec2) {
        self.friendly = true;
        self.direction = direction;
    }

    fn touching(&self) -> Vec<usize> {
        let object = self.object.as_ref().unwrap();
        object
            .layer()
            .intersections_with_shape(Shape::square(0.03, 0.03), object.transform.into())
    }

    fn position(&self) -> Vec2 {
        self.object.as_ref().unwrap().transform.position
    }

    fn damage(&self) -> f32 {
        self.damage
    }

    fn damage_multiplier(&mut self, multiplier: f32) {
        self.damage *= multiplier;
    }

    fn age(&self) -> Instant {
        self.age
    }

    fn remove(&mut self) {
        let _ = self.object.take().unwrap().remove();
    }

    fn friendly(&self) -> bool {
        self.friendly
    }
}
