use crate::light_modes::LightMode;
use crate::vector::Vector2D;
use macroquad::prelude as mq;

pub struct Light {
    pub pt: Vector2D,
    pub power: f32,
    pub light_mode: LightMode,
    pub color: mq::Color,
}
impl Light {
    pub fn new(pt: Vector2D, power: f32, light_mode: LightMode, color: mq::Color) -> Light {
        Light {
            pt,
            power,
            light_mode,
            color,
        }
    }
    pub fn calc_power(&self, time: f32) -> f32 {
        self.power + self.light_mode.calc_power_offset(time)
    }
}
