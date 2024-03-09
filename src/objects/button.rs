use std::sync::Arc;

use anyhow::Result;
use let_engine::prelude::*;

use crate::{FONT_RAWR, HEIGHT};

#[derive(Clone, Debug)]
pub struct Button {
    pub object: Object,
    pub text: Option<Label<Object>>,
    hovered: bool,
    pressed: bool,
}

pub enum Parent<'a> {
    Layer(&'a Arc<Layer>),
    Object(&'a Object),
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonReport {
    Pressed,
    Released,
}

impl Button {
    pub fn new(
        parent: Parent,
        material: Option<Material>,
        label_create_info: Option<LabelCreateInfo>,
        position: Vec2,
    ) -> Result<Self> {
        let label_create_info = label_create_info;
        let appearance = Appearance::new()
            .model(Some(Model::Square))
            .material(material)
            .auto_scaled(HEIGHT)?;
        let size = appearance.get_transform().size;
        let collider = ColliderBuilder::new(Shape::square(size.x, size.y)).build();

        let mut object = NewObjectBuilder::default()
            .appearance(appearance)
            .transform(Transform::default().position(position))
            .build()?;
        object.set_collider(Some(collider));

        let object = match parent {
            Parent::Layer(layer) => object.init(layer)?,
            Parent::Object(parent) => object.init_with_parent(parent)?,
        };

        let text = label_create_info.map(|mut label_create_info| {
            label_create_info.appearance.get_transform_mut().size = size;
            Label::new(&FONT_RAWR, label_create_info)
                .init_with_parent(&object)
                .unwrap()
        });

        Ok(Self {
            object,
            text,
            hovered: false,
            pressed: false,
        })
    }

    pub fn set_visibility(&mut self, visible: bool) {
        self.object.appearance.set_visible(visible);
        // if let Some(text) = self.text.as_mut() {
        //     text.object.appearance.set_visible(visible);
        // }
    }

    fn update(&mut self) -> Option<ButtonReport> {
        self.object.sync().unwrap();
        if let Some(text) = self.text.as_mut() {
            text.sync();
        }
        let layer = self.object.layer();
        let intersections =
            layer.intersections_with_ray(INPUT.cursor_to_world(layer), vec2(0.0, 0.0), 0.0, true);
        let mut report = None;

        if let Some(id) = intersections.first() {
            if id != self.object.id() {
                self.object
                    .appearance
                    .set_color(Color::from_rgb(1.0, 1.0, 1.0));
                self.hovered = false;
                self.pressed = false;
                return None;
            }
            self.hovered = true;
            self.object
                .appearance
                .set_color(Color::from_rgb(1.5, 1.5, 1.5));
            let pressed = INPUT.mouse_down(&MouseButton::Left);
            if !self.pressed && pressed {
                report = Some(ButtonReport::Pressed)
            } else if self.pressed && !pressed {
                report = Some(ButtonReport::Released)
            }
            self.pressed = pressed;
            if pressed {
                self.object
                    .appearance
                    .set_color(Color::from_rgb(0.5, 0.5, 0.5));
            }
        }
        report
    }

    #[allow(dead_code)]
    pub fn on_press(&mut self, action: impl FnOnce()) {
        if let Some(ButtonReport::Pressed) = self.update() {
            action()
        }
    }

    pub fn on_release(&mut self, action: impl FnOnce()) {
        if let Some(ButtonReport::Released) = self.update() {
            action()
        }
    }
}

// #[derive(Clone, Debug)]
// pub struct Toggle {
//     pub object: Object,
//     hovered: bool,
//     pressed: bool,
// }

// impl Toggle {
//     pub fn new() -> Result<Self> {

//         Ok(Self { object: , hovered: , pressed:  })
//     }
// }
