use std::sync::Arc;

use anyhow::Result;
use let_engine::prelude::*;

mod objects;
use objects::*;

fn main() -> Result<()> {
    let window_builder = WindowBuilder::new().fullscreen(false).title("Super Pong");

    let tick_settings = TickSettingsBuilder::default()
        .update_physics(false)
        .build()?;

    let engine = Engine::new(
        EngineSettingsBuilder::default()
            .window_settings(window_builder)
            .tick_settings(tick_settings)
            .build()?,
    )?;

    let game = Game::default();

    engine.start(game);

    Ok(())
}

struct Layers {
    main: Arc<Layer>,
    ui: Arc<Layer>,
}

impl Default for Layers {
    fn default() -> Self {
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

#[derive(Default)]
struct Game {
    layers: Layers,

    paddle: Option<paddle::Paddle>,

    exit: bool,
}

impl let_engine::Game for Game {
    fn exit(&self) -> bool {
        self.exit
    }
    fn start(&mut self) {
        self.paddle = paddle::Paddle::new(&self.layers.main).ok();
    }
    fn update(&mut self) {
        if let Some(paddle) = self.paddle.as_mut() {
            paddle.update(INPUT.cursor_position().y);
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
            Event::Window(WindowEvent::CloseRequested) => {
                self.exit = true;
            }
            _ => (),
        }
    }
}
