use macroquad::prelude as mq;
use crate::vector::Vector;

pub struct Light {
	pub position: Vector,
	pub strength: f32,
	pub intensity: f32,
	pub color: mq::Color,
}