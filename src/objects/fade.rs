use std::time::{Duration, Instant};

use super::Layers;
use let_engine::prelude::*;

pub fn fade_in(duration: Duration, layers: &Layers) {
    let appearance = Appearance::new()
        .color(Color::BLACK)
        .transform(Transform::default().size(vec2(10.0, 10.0)));

    let mut object = NewObjectBuilder::default()
        .appearance(appearance)
        .transform(Transform::default())
        .build()
        .unwrap()
        .init(&layers.ui)
        .unwrap();
    std::thread::spawn(move || {
        let now = Instant::now();
        let start_time = now.elapsed();
        let time = duration.as_secs_f32();
        while now.elapsed() <= duration {
            let elapsed = now.elapsed().as_secs_f32();
            let percentage = (elapsed - time) / (start_time.as_secs_f32() - time);
            object.appearance.get_color_mut().set_a(percentage);
            object.sync();
        }
        object.remove();
    });
}
