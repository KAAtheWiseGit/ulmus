pub trait View {
	fn view(&self, area: Area) -> String;
}

#[derive(Copy, Clone)]
pub struct Area {
	pub x: usize,
	pub y: usize,
	pub width: usize,
	pub height: usize,
}

impl Area {
	pub fn new(x: usize, y: usize, width: usize, height: usize) -> Area {
		Area {
			x,
			y,
			width,
			height,
		}
	}
}
