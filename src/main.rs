#![windows_subsystem = "windows"]

use anyhow::Result;
use let_engine::prelude::*;
use once_cell::sync::Lazy;

mod game;
pub mod objects;

pub static FONT_STINGRAY: Lazy<Font> = Lazy::new(|| {
    Font::from_slice(include_bytes!("../assets/fonts/Px437_CL_Stingray_8x16.ttf")).unwrap()
});
pub static FONT_RAWR: Lazy<Font> =
    Lazy::new(|| Font::from_slice(include_bytes!("../assets/fonts/Rawr-Regular.ttf")).unwrap());

pub static HEIGHT: f32 = 256.0;

fn main() -> Result<()> {
    let settings = game::GameSettings::load().unwrap_or_default();

    let tick_settings = TickSettingsBuilder::default()
        .update_physics(false)
        .build()?;

    let fullscreen = match settings.fullscreen {
        game::Fullscreen::Windowed => None,
        game::Fullscreen::Borderless => Some(Fullscreen::Borderless(None)),
        game::Fullscreen::Exclusive => Some(Fullscreen::Borderless(None)),
    };

    let window_builder = WindowBuilder::new()
        .fullscreen(fullscreen)
        .maximized(true)
        // .resizable(true)
        .title("Super Pong");
    let engine = Engine::new(
        EngineSettingsBuilder::default()
            .window_settings(window_builder)
            .tick_settings(tick_settings)
            .build()?,
    )?;

    let game = game::Game::new(settings)?;

    engine.start(game);

    Ok(())
}
