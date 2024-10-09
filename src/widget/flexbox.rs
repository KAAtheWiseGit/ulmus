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
		mut sizes: Vec<Size>,
	) -> Box<Flexbox> {
		sizes.resize(widgets.len(), Size::Auto);

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

	fn layout(&self, length: u16) -> Vec<u16> {
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
				let hint_length = match self.direction {
					Direction::Vertical => self.widgets[i]
						.get_height_hint(),
					Direction::Horizontal => {
						self.widgets[i].get_width_hint()
					}
				};

				cut!(i, hint_length);
			}
		}

		// TODO: fractions

		out
	}

	fn split(&self, area: Area) -> Vec<Area> {
		let lengths = match self.direction {
			Direction::Vertical => self.layout(area.height),
			Direction::Horizontal => self.layout(area.width),
		};
		let mut out = vec![];
		let mut offset = 0;

		for length in lengths {
			let curr_area = match self.direction {
				Direction::Vertical => Area {
					y: area.y + offset,
					height: length,
					..area
				},
				Direction::Horizontal => Area {
					x: area.x + offset,
					width: length,
					..area
				},
			};
			out.push(curr_area);

			offset += length;
		}

		out
	}
}

impl Widget for Flexbox {
	fn get_width_hint(&self) -> u16 {
		self.widgets.iter().map(|w| w.get_width_hint()).sum()
	}

	fn get_height_hint(&self) -> u16 {
		self.widgets.iter().map(|w| w.get_height_hint()).sum()
	}

	fn render(&self, area: Area) -> String {
		let mut out = String::new();

		for (area, widget) in self.split(area).iter().zip(&self.widgets)
		{
			out += &widget.render(*area);
		}

		out
	}

	fn process_mouse(&self, event: MouseEvent, area: Area) -> Message {
		for (area, widget) in self.split(area).iter().zip(&self.widgets)
		{
			if area.contains(event) {
				return widget.process_mouse(event, *area);
			}
		}

		Message::empty()
	}
}
