use std::collections::VecDeque;

use let_engine::{
    egui::{self, RichText, TextEdit, TextStyle, TopBottomPanel},
    SETTINGS,
};

use super::{Fullscreen, GameSettings, Message};

#[derive(Clone, Debug)]
pub struct Console {
    pub settings: GameSettings,
    active: bool,
    focus: bool,
    text: String,
    history: VecDeque<String>,
    last_command: String,
}

impl Console {
    pub fn new(settings: GameSettings) -> Self {
        Self {
            settings,
            active: false,
            focus: false,
            text: String::new(),
            history: VecDeque::new(),
            last_command: String::new(),
        }
    }
    pub fn update(&mut self, context: &egui::Context) -> Option<Message> {
        let mut message = None;
        TopBottomPanel::top("console")
            .resizable(true)
            .show_animated(context, self.active, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for item in self.history.iter() {
                        ui.label(RichText::new(item).code().monospace().strong());
                    }
                });
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    self.text = self.last_command.clone();
                }
                let line = ui.add(
                    TextEdit::singleline(&mut self.text)
                        .font(TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) && line.lost_focus() {
                    message = self.execute();
                    self.text.clear();
                    line.request_focus();
                }
                if self.focus {
                    line.request_focus();
                    self.focus = false;
                }
            });
        message
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        self.focus = true;
    }

    pub fn execute(&mut self) -> Option<Message> {
        self.print(self.text.clone());
        self.last_command = self.text.clone();

        let text = self.text.to_lowercase();
        let mut tokens = text.split_ascii_whitespace();

        let Some(command) = tokens.next() else {
            return None;
        };

        match command {
            "vsync" => {
                let Some(command) = tokens.next() else {
                    self.print(format!(
                        "usage:\n  vsync [on/off]\nvsync={}",
                        self.settings.vsync
                    ));
                    return None;
                };
                let vsync = match command {
                    "on" => true,
                    "true" => true,
                    "enable" => true,
                    "disable" => false,
                    "false" => false,
                    "off" => false,
                    e => {
                        self.print(format!("You can not set vsync to \"{e}\"."));
                        return None;
                    }
                };
                let settings = GameSettings {
                    vsync,
                    ..self.settings
                };
                self.print(format!("vsync set: {} -> {}", self.settings.vsync, vsync));
                return Some(Message::ApplySettings(settings));
            }
            "fps_limit" => {
                let Some(command) = tokens.next() else {
                    self.print(format!(
                        "usage:\n  fps_limit [number]\n fps_limit={}",
                        self.settings.fps_limit
                    ));
                    return None;
                };
                let Ok(fps_limit) = command.parse() else {
                    self.print(format!("You can not set your fps limit to \"{command}\"."));
                    return None;
                };
                let settings = GameSettings {
                    fps_limit,
                    ..self.settings
                };
                self.print(format!(
                    "framerate limit set: {} -> {}",
                    self.settings.fps_limit, fps_limit
                ));
                return Some(Message::ApplySettings(settings));
            }
            "fullscreen" => {
                let Some(command) = tokens.next() else {
                    self.print(format!(
                        "usage:\n  fullscreen [windowed/borderless/exclusive]\n fullscreen={:?}",
                        self.settings.fullscreen
                    ));
                    return None;
                };
                let fullscreen = match command {
                    "windowed" => Fullscreen::Windowed,
                    "w" => Fullscreen::Windowed,
                    "borderless" => Fullscreen::Borderless,
                    "b" => Fullscreen::Borderless,
                    "exclusive" => Fullscreen::Exclusive,
                    "x" => Fullscreen::Exclusive,
                    e => {
                        self.print(format!("Can not set fullscreen to \"{e}\""));
                        return None;
                    }
                };
                let settings = GameSettings {
                    fullscreen,
                    ..Default::default()
                };
                return Some(Message::ApplySettings(settings));
            }
            "scene" => {
                let Some(command) = tokens.next() else {
                    self.print("usage:\n  scene [scene]".to_string());
                    return None;
                };
                match command {
                    "menu" => return Some(Message::SwitchScene(super::GameScene::Menu)),
                    "ingame" => return Some(Message::SwitchScene(super::GameScene::Ingame)),
                    e => self.print(format!("There is no scene called \"{e}\".")),
                };
                return None;
            }
            "stage" => {
                let Some(command) = tokens.next() else {
                    self.print("usage:\n  stage [number]".to_string());
                    return None;
                };
                let Ok(level) = command.parse() else {
                    self.print(format!("Invalid stage \"{command}\"."));
                    return None;
                };

                return Some(Message::ChangeLevel(level));
            }
            "quit" => return Some(Message::Exit),
            "exit" => return Some(Message::Exit),
            "close" => self.active = false,
            "clear" => self.history.clear(),
            "clear-cache" => SETTINGS.clean_caches(),
            "help" => {
                self.print(HELP_MESSAGE.to_string());
            }
            _ => {
                self.print(format!("\"{command}\" not defined."));
            }
        }
        None
    }

    pub fn print(&mut self, text: String) {
        self.history.push_back(text);
        if self.history.len() >= 50 {
            self.history.pop_front();
        }
    }
}

const HELP_MESSAGE: &str = "
Available commands:
  vsync [on/off] - Enables or disables vsync.
  fps_limit [number] - sets the framerate limit of the game.
  fullscreen [windowed/borderless/exclusive] - Sets if the window is in fullscreen.
  scene [scene] - changes the scene.
  stage [number] - sets the stage.
  clear - clears the console
  clear-cache - clears the cache reducing memory usage.
  close - closes the terminal 
  quit/exit - quits the game immediately
  help - displays this help message.
";
