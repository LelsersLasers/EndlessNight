use crate::camera_manager::CameraManager;
use crate::light::Light;
use crate::toggle::ToggleKey;

use macroquad::prelude as mq;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum DirKey {
    Up,
    Down,
    Left,
    Right,
}
impl DirKey {
    pub fn opposite(&self) -> DirKey {
        match *self {
            DirKey::Up => DirKey::Down,
            DirKey::Down => DirKey::Up,
            DirKey::Left => DirKey::Right,
            DirKey::Right => DirKey::Left,
        }
    }
}

pub struct Player {
    pub pt: mq::Vec2,
    pub w: f32,
    pub h: f32,
    pub light: Light,

    pub keys: Vec<DirKey>,
    pub up_tk: ToggleKey,
    pub down_tk: ToggleKey,
    pub left_tk: ToggleKey,
    pub right_tk: ToggleKey,

    pub last_dir: DirKey,
    pub wall_dirs: Vec<DirKey>,
}
impl Player {
    pub fn new(pt: mq::Vec2, w: f32, h: f32, light: Light) -> Player {
        Player {
            pt,
            w,
            h,
            light,
            keys: vec![],
            up_tk: ToggleKey::new(),
            down_tk: ToggleKey::new(),
            left_tk: ToggleKey::new(),
            right_tk: ToggleKey::new(),
            last_dir: DirKey::Right,
            wall_dirs: vec![],
        }
    }
    pub fn update_light_pt(&mut self) {
        self.light.pt = self.pt + mq::vec2(self.w, self.h) / 2.;
    }
    pub fn draw(&mut self, color: mq::Color, cm: &CameraManager) {
        let pt = cm.calc_offset(self.pt);

        let (dir_color, dir) = match self.calc_dir() {
            (false, d) => (mq::BLUE, d),
            (true, d) => (mq::RED, d),
        };
        let center = pt + mq::vec2(self.w, self.h) / 2.;
        let dir_pt = match dir {
            DirKey::Up => center + mq::vec2(0., -self.h),
            DirKey::Down => center + mq::vec2(0., self.h),
            DirKey::Left => center + mq::vec2(-self.w, 0.),
            DirKey::Right => center + mq::vec2(self.w, 0.),
        };

        mq::draw_rectangle(pt.x, pt.y, self.w, self.h, color);
        mq::draw_line(center.x, center.y, dir_pt.x, dir_pt.y, 2., dir_color);
    }
    fn set_keys_down(&mut self) {
        let up = self
            .up_tk
            .down(mq::is_key_down(mq::KeyCode::W) || mq::is_key_down(mq::KeyCode::Up));
        let down = self
            .down_tk
            .down(mq::is_key_down(mq::KeyCode::S) || mq::is_key_down(mq::KeyCode::Down));
        let left = self
            .left_tk
            .down(mq::is_key_down(mq::KeyCode::A) || mq::is_key_down(mq::KeyCode::Left));
        let right = self
            .right_tk
            .down(mq::is_key_down(mq::KeyCode::D) || mq::is_key_down(mq::KeyCode::Right));

        if right {
            self.keys.push(DirKey::Right);
        } else if !(mq::is_key_down(mq::KeyCode::D) || mq::is_key_down(mq::KeyCode::Right)) {
            self.keys.retain(|k| *k != DirKey::Right);
        }

        if left {
            self.keys.push(DirKey::Left);
        } else if !(mq::is_key_down(mq::KeyCode::A) || mq::is_key_down(mq::KeyCode::Left)) {
            self.keys.retain(|k| *k != DirKey::Left);
        }

        if down {
            self.keys.push(DirKey::Down);
        } else if !(mq::is_key_down(mq::KeyCode::S) || mq::is_key_down(mq::KeyCode::Down)) {
            self.keys.retain(|k| *k != DirKey::Down);
        }

        if up {
            self.keys.push(DirKey::Up);
        } else if !(mq::is_key_down(mq::KeyCode::W) || mq::is_key_down(mq::KeyCode::Up)) {
            self.keys.retain(|k| *k != DirKey::Up);
        }
    }
    fn calc_dir(&mut self) -> (bool, DirKey) {
        // return: (is_moving, dir)
        if self.keys.is_empty() {
            (true, self.last_dir)
        } else {
            for k in self.keys.iter().rev() {
                if !self.keys.contains(&k.opposite()) {
                    self.last_dir = *k;
                    return (false, *k);
                }
            }
            (true, self.last_dir)
        }
    }
    fn move_player(&mut self, cm: &mut CameraManager, delta: f32) {
        let mut move_vec = mq::Vec2::ZERO;
        match self.calc_dir() {
            (false, DirKey::Up) => move_vec.y -= 1.,
            (false, DirKey::Down) => move_vec.y += 1.,
            (false, DirKey::Right) => move_vec.x += 1.,
            (false, DirKey::Left) => move_vec.x -= 1.,
            (true, _) => {}
        }

        move_vec = move_vec.normalize_or_zero() * 20. * delta;
        self.pt += move_vec;

        // let current_offset = cm.pt - self.pt;
        // cm.pt += (cm.target_offset - current_offset) * delta * 2.;
        // cm.pt = cm.target_offset - current_offset;
        cm.pt += move_vec;
    }
    pub fn update(&mut self, cm: &mut CameraManager, delta: f32) {
        self.set_keys_down();
        self.move_player(cm, delta);
        self.update_light_pt();

        
        self.wall_dirs.clear();
    }
    pub fn collide_immovable(&mut self, cm: &mut CameraManager, other: mq::Rect) -> bool {
        let rect = mq::Rect::new(self.pt.x, self.pt.y, self.w, self.h);
        if let Some(overlap) = rect.intersect(other) {
            let mut move_vec = mq::Vec2::ZERO;
            if overlap.w > overlap.h {
                if rect.y < other.y {
                    move_vec.y = other.y - (rect.y + rect.h);
                } else {
                    move_vec.y = other.y + other.h - rect.y;
                }
            } else if rect.x < other.x {
                move_vec.x = other.x - (rect.x + rect.w);
            } else {
                move_vec.x = other.x + other.w - rect.x;
            }

            let dir = self.calc_dir();
            match dir {
                (false, k) => self.wall_dirs.push(k),
                (true, _) => {}
            }

            self.pt += move_vec;
            cm.pt += move_vec;
            true
        } else {
            false
        }
    }
}
