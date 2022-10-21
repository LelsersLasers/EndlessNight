use crate::light_modes::LightMode;
use macroquad::prelude as mq;

#[derive(Clone, Copy)]
pub struct Light {
    pub pt: mq::Vec2,
    pub power: f32,
    pub light_mode: LightMode,
    pub color: mq::Color,
}
impl Light {
    pub fn new(pt: mq::Vec2, power: f32, light_mode: LightMode, color: mq::Color) -> Light {
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
