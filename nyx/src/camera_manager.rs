use macroquad::prelude as mq;

pub struct CameraManager {
    pub pt: mq::Vec2,
}
impl CameraManager {
    pub fn new(pt: mq::Vec2) -> CameraManager {
        CameraManager { pt }
    }
    pub fn calc_offset(&self, pt: mq::Vec2) -> mq::Vec2 {
        self.pt - pt
    }
}
