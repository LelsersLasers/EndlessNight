use crate::light::Light;
use crate::camera_manager::CameraManager;

use macroquad::prelude as mq;

pub struct Player {
    pub pt: mq::Vec2,
    pub light: Light,
}
impl Player {
    pub fn new(pt: mq::Vec2, light: Light) -> Player {
        Player { pt, light }
    }
    pub fn update_light_pt(&mut self) {
        self.light.pt = self.pt;
    }
    pub fn draw(&self, color: mq::Color) {
        mq::draw_rectangle(self.pt.x - 4., self.pt.y - 5., 8., 10., color)
    }
    pub fn update(&mut self, camera_manager: &mut CameraManager, delta: f32) {
        let mut move_vec = mq::Vec2::ZERO;
        if mq::is_key_down(mq::KeyCode::W) {
            move_vec.y -= 1.;
        }
        if mq::is_key_down(mq::KeyCode::S) {
            move_vec.y += 1.;
        }
        if mq::is_key_down(mq::KeyCode::A) {
            move_vec.x -= 1.;
        }
        if mq::is_key_down(mq::KeyCode::D) {
            move_vec.x += 1.;
        }
        move_vec = move_vec.normalize_or_zero() * 15. * delta;
        self.pt += move_vec;
        self.update_light_pt();
        camera_manager.pt += move_vec;
    }
}
