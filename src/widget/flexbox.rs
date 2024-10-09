use crossterm::event::MouseEvent;

use super::Widget;
use crate::{Area, Message};

pub enum Direction {
	Vertical,
	Horizontal,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Size {
	Length(u16),
	Auto,
	Fraction(u16),
}

pub struct Flexbox {
	direction: Direction,
	widgets: Vec<Box<dyn Widget>>,
	sizes: Vec<Size>,
}

impl Flexbox {
	pub fn new(
		direction: Direction,
		widgets: Vec<Box<dyn Widget>>,
		sizes: Vec<Size>,
	) -> Box<Flexbox> {
		// TODO automatically truncate/extend `sizes`
		assert_eq!(widgets.len(), sizes.len());

		Box::new(Flexbox {
			widgets,
			sizes,
			direction,
		})
	}

	pub fn vertical(
		widgets: Vec<Box<dyn Widget>>,
		sizes: Vec<Size>,
	) -> Box<Flexbox> {
		Flexbox::new(Direction::Vertical, widgets, sizes)
	}

	pub fn horizontal(
		widgets: Vec<Box<dyn Widget>>,
		sizes: Vec<Size>,
	) -> Box<Flexbox> {
		Flexbox::new(Direction::Horizontal, widgets, sizes)
	}

	fn layout(&self, direction: Direction, length: u16) -> Vec<u16> {
		let mut out = vec![0; self.widgets.len()];
		let mut total_length = length;

		macro_rules! cut {
			($i:ident, $length:expr) => {
				if total_length >= $length {
					total_length -= $length;
					out[$i] = $length;
				} else {
					out[$i] = total_length;
					return out;
				}
			};
		}

		for (i, s) in self.sizes.iter().enumerate() {
			if let Size::Length(size_length) = s {
				cut!(i, *size_length);
			}
		}

		for (i, s) in self.sizes.iter().enumerate() {
			if let Size::Auto = s {
				let hint_length = match direction {
					Direction::Vertical => self.widgets[i]
						.get_width_hint()
						as u16,
					Direction::Horizontal => {
						self.widgets[i]
							.get_height_hint() as u16
					}
				};

				cut!(i, hint_length);
			}
		}

		// TODO: fractions

		out
	}
}

impl Widget for Flexbox {
	fn get_width_hint(&self) -> usize {
		self.widgets.iter().map(|w| w.get_width_hint()).sum()
	}

	fn get_height_hint(&self) -> usize {
		self.widgets.iter().map(|w| w.get_height_hint()).sum()
	}

	fn render(&self, area: Area) -> String {
		todo!()
	}

	fn on_click(&self, area: Area, event: MouseEvent) -> Message {
		todo!()
	}
}
