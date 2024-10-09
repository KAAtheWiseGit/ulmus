pub use crossterm::event::MouseEvent;

#[derive(Copy, Clone, Debug)]
pub struct Area {
	pub x: u16,
	pub y: u16,
	pub width: u16,
	pub height: u16,
}

impl Area {
	pub fn new(x: u16, y: u16, width: u16, height: u16) -> Area {
		Area {
			x,
			y,
			width,
			height,
		}
	}

	pub fn contains(&self, event: MouseEvent) -> bool {
		let (x, y) = (event.column, event.row);

		self.x <= x
			&& x < self.x + self.width
			&& self.y <= y && y < self.y + self.height
	}
}
