use std::ops::{Add, Sub, Mul};


#[derive(Clone, Copy)]
struct Vector {
	x: f32,
	y: f32,
}
impl Vector {
	fn new(x: f32, y: f32) -> Vector {
		Vector { x, y }
	}
	fn calc_length(&self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
	fn get_angle(&self) -> f32 {
		self.y.atan2(self.x) * 180.0 / std::f32::consts::PI
	}
	fn with_angle(&self, angle: f32) -> Vector {
		let length = self.calc_length();
		Self::new(angle.cos(), length * angle.sin()) * length
	}
	fn with_len(&self, len: f32) -> Vector {
		let current_length = self.calc_length();
		*self * (len / current_length.max(0.))
	}
}
impl Add for Vector {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}
impl Sub for Vector {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}
impl Mul<f32> for Vector { // scalar multiplication
    type Output = Self;
    fn mul(self, scale: f32) -> Self::Output {
		Self {
			x: self.x * scale,
			y: self.y * scale,
		}
    }
}
impl Mul for Vector { // dot product
    type Output = f32;
    fn mul(self, other: Self) -> Self::Output {
		self.x * other.x + self.y * other.y
    }
}