pub use crossterm::event::MouseEvent;

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

	pub fn contains(&self, event: MouseEvent) -> bool {
		let ex: usize = event.column.into();
		let ey: usize = event.row.into();

		self.x <= ex
			&& ex <= self.x + self.width
			&& self.y <= ey && ey <= self.y + self.height
	}
}
