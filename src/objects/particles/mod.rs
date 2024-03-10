use let_engine::prelude::*;
use std::{f32::consts::PI, sync::Arc, thread, time::Duration};

pub fn debris_particle(
    layer: &Arc<Layer>,
    appearance: Appearance,
    position: Vec2,
    mut direction: Vec2,
) {
    let mut object = NewObjectBuilder::default()
        .appearance(appearance)
        .transform(Transform::default().position(position))
        .build()
        .unwrap()
        .init(layer)
        .unwrap();
    thread::spawn(move || {
        for i in 0..3000 {
            thread::sleep(Duration::from_millis(1));
            object.transform.position += (direction - vec2(0.0, 0.7)) * 0.001;
            direction.y += 0.0007;
            object.transform.rotation = (PI * 0.5) * (i as f32 * 0.001).round();
            object.sync().unwrap();
        }
        object.remove()
    });
}
