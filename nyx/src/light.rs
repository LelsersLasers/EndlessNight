use crate::vector::Vector2D;
use macroquad::prelude as mq;

pub struct Light {
    pub pt: Vector2D,
    pub power: f32,
    pub intensity: f32,
    pub color: mq::Color,
}
impl Light {
    pub fn new(pt: Vector2D, power: f32, intensity: f32, color: mq::Color) -> Light {
        Light {
            pt,
            power,
            intensity,
            color,
        }
    }
}
