use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}
impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D { x, y }
    }
    pub fn calc_length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn get_angle(&self) -> f32 {
        self.y.atan2(self.x) * 180.0 / std::f32::consts::PI
    }
    pub fn with_angle(&self, angle: f32) -> Vector2D {
        let length = self.calc_length();
        Self::new(angle.cos(), length * angle.sin()) * length
    }
    pub fn normalized(&self) -> Vector2D {
        let length = self.calc_length();
        if length == 0. {
            *self
        } else {
            Vector2D::new(self.x / length.max(0.), self.y / length.max(0.))
        }
    }
    pub fn with_len(&self, len: f32) -> Vector2D {
        self.normalized() * len
    }
}

impl Add for Vector2D {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Vector2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul<f32> for Vector2D {
    // scalar multiplication
    type Output = Self;
    fn mul(self, scale: f32) -> Self::Output {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}
impl MulAssign<f32> for Vector2D {
    // scalar multiplication
    fn mul_assign(&mut self, scale: f32) {
        *self = Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}
