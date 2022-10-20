

pub enum LightMode {
	Static,
	Sin(f32, f32, f32), // (power, variance, period, start_time)
}
impl LightMode {
	pub fn get_power_offset(&self, time: f32) -> f32 {
		match self {
			LightMode::Static => 0.,
			LightMode::Sin(variance, period, start_time) => {
				variance * (((2. * std::f32::consts::PI) / period) * (time - start_time)).sin()
			}
		}
	}
}