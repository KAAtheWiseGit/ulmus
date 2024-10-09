use super::Widget;
use crate::{Command, Message, Reactive, View};

pub enum Direction {
	Row,
	Column,
}

pub enum Size {
	Length(usize),
	Auto,
	Fraction(usize),
}

pub struct Flexbox {
	widgets: Vec<Box<dyn Widget>>,
	sizes: Vec<Size>,

	direction: Direction,
}

impl Flexbox {
	pub fn from(
		widgets: Vec<Box<dyn Widget>>,
		sizes: Vec<Size>,
		direction: Direction,
	) -> Self {
		// TODO automatically truncate/extend `sizes`
		assert_eq!(widgets.len(), sizes.len());

		Self {
			widgets,
			sizes,
			direction,
		}
	}
}

impl Reactive for Flexbox {
	fn update(&mut self, _message: Message) -> Vec<Command> {
		todo!()
	}
}

impl View for Flexbox {
	fn view(&self) -> String {
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
