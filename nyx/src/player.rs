use crate::light::Light;
use crate::camera_manager::CameraManager;

use macroquad::prelude as mq;

pub struct Player {
    pub pt: mq::Vec2,
    pub w: f32,
    pub h: f32,
    pub light: Light,
}
impl Player {
    pub fn new(pt: mq::Vec2, w: f32, h: f32, light: Light) -> Player {
        Player { pt, w, h, light }
    }
    pub fn update_light_pt(&mut self) {
        self.light.pt = self.pt + mq::vec2(self.w, self.h) / 2.;
    }
    pub fn draw(&self, color: mq::Color, cm: &CameraManager) {
        let pt = cm.calc_offset(self.pt);
        mq::draw_rectangle(pt.x, pt.y, self.w, self.h, color)
    }
    pub fn update(&mut self, cm: &mut CameraManager, delta: f32) {
        let mut move_vec = mq::Vec2::ZERO;
        if mq::is_key_down(mq::KeyCode::W) || mq::is_key_down(mq::KeyCode::Up) {
            move_vec.y -= 1.;
        }
        if mq::is_key_down(mq::KeyCode::S) || mq::is_key_down(mq::KeyCode::Down) {
            move_vec.y += 1.;
        }
        if mq::is_key_down(mq::KeyCode::A) || mq::is_key_down(mq::KeyCode::Left) {
            move_vec.x -= 1.;
        }
        if mq::is_key_down(mq::KeyCode::D) || mq::is_key_down(mq::KeyCode::Right) {
            move_vec.x += 1.;
        }
        move_vec = move_vec.normalize_or_zero() * 20. * delta;
        self.pt += move_vec;
        self.update_light_pt();

        // let current_offset = cm.pt - self.pt;
        // cm.pt += (cm.target_offset - current_offset) * delta * 2.;
        // cm.pt = cm.target_offset - current_offset;
        cm.pt += move_vec;
    }
}
