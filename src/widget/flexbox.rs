#![allow(unused)]

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

	width: Option<usize>,
	height: Option<usize>,
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
			width: None,
			height: None,
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
	fn set_width(&mut self, width: Option<usize>) {
		self.width = width;
	}

	fn set_height(&mut self, height: Option<usize>) {
		self.height = height;
	}

	fn get_width(&self) -> usize {
		self.width.unwrap_or_else(|| {
			self.widgets.iter().map(|w| w.get_width()).sum()
		})
	}

	fn get_height(&self) -> usize {
		self.height.unwrap_or_else(|| {
			self.widgets.iter().map(|w| w.get_height()).sum()
		})
	}
}
