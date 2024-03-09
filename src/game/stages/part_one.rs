use std::{collections::VecDeque, time::Duration};

use let_engine::prelude::*;

use crate::{
    game::game_loop::{Level, LevelMessage as Msg},
    objects::enemies::EnemyType,
};

pub fn tutorial() -> Level {
    let color = Color::from_rgb(0.9, 0.3, 0.1);

    let events = VecDeque::from([
        Msg::ShowTitle {
            color,
            size: vec2(70.0, 70.0),
            text: "Tutorial Stage".to_string(),
        },
        Msg::ChangeWaitingTime(Duration::from_secs(4)),
        Msg::ShowTitle {
            color,
            size: vec2(60.0, 60.0),
            text: "Move your mouse up and down to control your paddle.".to_string(),
        },
        Msg::ShowTitle {
            color,
            size: vec2(60.0, 60.0),
            text: "To move the arrow press the mouse buttons.".to_string(),
        },
        Msg::ShowTitle {
            color,
            size: vec2(60.0, 60.0),
            text: "By touching the projectiles you send them to where the arrow is pointing."
                .to_string(),
        },
        Msg::SpawnEnemy(EnemyType::Target),
        Msg::ChangeWaitingTime(Duration::from_secs(5)),
        Msg::ShowTitle {
            color,
            size: vec2(60.0, 60.0),
            text: "Hit the enemies with their projectiles to damage them.".to_string(),
        },
        Msg::ChangeWaitingTime(Duration::from_secs(1)),
        Msg::SpawnEnemy(EnemyType::Target),
        Msg::SpawnEnemy(EnemyType::Target),
        Msg::ChangeWaitingTime(Duration::from_secs(6)),
        Msg::ShowTitle {
            color,
            size: vec2(60.0, 60.0),
            text: "You can return an extra hard projectile back by touching it with the side of your paddle.".to_string(),
        },
        Msg::ChangeWaitingTime(Duration::from_secs(1)),
        Msg::SpawnEnemy(EnemyType::Target),
        Msg::SpawnEnemy(EnemyType::Target),
        Msg::ChangeWaitingTime(Duration::from_secs(5)),
        Msg::SpawnEnemy(EnemyType::Target),
    ]);

    Level::new(2, Duration::from_secs(2), events)
}
