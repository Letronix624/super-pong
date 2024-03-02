use std::sync::Arc;

use anyhow::Result;
use let_engine::prelude::*;

use crate::{FONT_RAWR, HEIGHT};

pub struct Button {
    pub object: Object,
    pub text: Label<Object>,
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
        label_create_info: LabelCreateInfo,
        position: Vec2,
    ) -> Result<Self> {
        let mut label_create_info = label_create_info;
        let appearance = Appearance::new().material(material).auto_scaled(HEIGHT)?;
        let size = appearance.get_transform().size;
        let collider = ColliderBuilder::new(Shape::square(size.x, size.y)).build();
        label_create_info.appearance.get_transform_mut().size = size;

        let mut object = NewObjectBuilder::default()
            .appearance(appearance)
            .transform(Transform::default().position(position))
            .build()?;
        object.set_collider(Some(collider));

        let object = match parent {
            Parent::Layer(layer) => object.init(layer)?,
            Parent::Object(parent) => object.init_with_parent(parent)?,
        };

        let text = Label::new(&FONT_RAWR, label_create_info).init_with_parent(&object)?;

        Ok(Self {
            object,
            text,
            hovered: false,
            pressed: false,
        })
    }

    fn update(&mut self) -> Option<ButtonReport> {
        self.object.sync();
        let layer = self.object.layer();
        let intersections =
            layer.intersections_with_ray(INPUT.cursor_to_world(layer), vec2(0.0, 0.0), 0.0, true);
        let mut report = None;

        if intersections.contains(self.object.id()) {
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
        } else {
            self.object
                .appearance
                .set_color(Color::from_rgb(1.0, 1.0, 1.0));
            if self.pressed {
                report = Some(ButtonReport::Released)
            }
            self.hovered = false;
            self.pressed = false;
        }
        report
    }

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
