use macroquad::prelude as mq;

pub struct CameraManager {
    pub pt: mq::Vec2,
    pub target_offset: mq::Vec2,
}
impl CameraManager {
    pub fn new(pt: mq::Vec2, target: mq::Vec2) -> Self {
        CameraManager { pt, target_offset: target - pt }
    }
    pub fn calc_offset(&self, pt: mq::Vec2) -> mq::Vec2 {
        pt - self.pt
    }
}
