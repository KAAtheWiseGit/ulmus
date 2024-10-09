use super::Widget;
use crate::{Area, Command, Message, Reactive, View};

pub enum Direction {
	Vertical,
	Horizontal,
}

pub enum Size {
	Length(usize),
	Auto,
	Fraction(usize),
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
}

impl Reactive for Flexbox {
	fn update(&mut self, _message: Message) -> Vec<Command> {
		todo!()
	}
}

impl View for Flexbox {
	fn view(&self, _area: Area) -> String {
		todo!()
	}
}

impl Widget for Flexbox {
	fn get_width_hint(&self) -> usize {
		self.widgets.iter().map(|w| w.get_width_hint()).sum()
	}

	fn get_height_hint(&self) -> usize {
		self.widgets.iter().map(|w| w.get_height_hint()).sum()
	}
}
