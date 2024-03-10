use anyhow::Result;
use let_engine::prelude::*;
use std::io::Cursor;

#[derive(Clone)]
pub struct Sounds {
    pub critical: Sound,
    pub damage: Sound,
    pub square_hit: Sound,
    pub target_hit: Sound,
    pub target_destroy: Sound,
}

impl Sounds {
    /// Loads the sounds into memory
    pub fn new() -> Result<Self> {
        let critical = Sound::new(
            SoundData::from_cursor(Cursor::new(asset("sounds/critical.ogg")?))?,
            SoundSettings::default().volume(0.5),
        );
        let damage = Sound::new(
            SoundData::gen_square_wave(90.0, 0.06),
            SoundSettings::default().volume(0.2),
        );
        let square_hit = Sound::new(
            SoundData::gen_square_wave(777.0, 0.03),
            SoundSettings::default().volume(0.2),
        );
        let target_hit = Sound::new(
            SoundData::from_cursor(Cursor::new(asset("sounds/target-hit.ogg")?))?,
            SoundSettings::default().volume(0.5),
        );
        let target_destroy = Sound::new(
            SoundData::from_cursor(Cursor::new(asset("sounds/target-destroy.ogg")?))?,
            SoundSettings::default().volume(0.5),
        );

        Ok(Self {
            critical,
            damage,
            square_hit,
            target_hit,
            target_destroy,
        })
    }
}
