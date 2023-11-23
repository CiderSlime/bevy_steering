use bevy::prelude::*;


#[allow(unused)]
pub fn rotate(v: Vec2, angle_degrees: f64) -> Vec2 {
    let angle_radians = angle_degrees.to_radians();
    let sin_a = angle_radians.sin() as f32;
    let cos_a = angle_radians.cos() as f32;

    let result = Vec2 {
        x: v.x * cos_a - v.y * sin_a,
        y: v.x * sin_a + v.y * cos_a,
    };
    // info!("{} * {} = {}", v, angle_degrees, result);
    result
}

pub fn truncate_exceeded(v: Vec2, max: f32) -> Vec2 {
    // truncates Vec2 if it exceeds max length
    let i = max / v.length();
    let i = if i < 1.0 { i } else { 1.0 };
    v * i
}

pub fn check_overflow(mut t: Vec3, width: f32, height: f32) -> Vec3 {
    if t.x > width {t.x = 0.}
    if t.x < 0. {t.x = width}
    if t.y > height {t.y = 0.}
    if t.y < 0. {t.y = height}
    t
}