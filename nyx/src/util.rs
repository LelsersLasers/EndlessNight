use macroquad::prelude as mq;

pub fn check_collide(
    a_pt: mq::Vec2,
    a_w: f32,
    a_h: f32,
    b_pt: mq::Vec2,
    b_w: f32,
    b_h: f32,
) -> Option<mq::Rect> {
    let a_rect = mq::Rect::new(a_pt.x, a_pt.y, a_w, a_h);
    let b_rect = mq::Rect::new(b_pt.x, b_pt.y, b_w, b_h);
    a_rect.intersect(b_rect)
}
