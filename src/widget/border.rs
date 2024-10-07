use std::str::Lines;

use super::Widget;
use crate::{Cmd, Msg};

pub struct Border<W: Widget> {
	child: W,

	width: Option<usize>,
	height: Option<usize>,

	pub top: String,
	pub bottom: String,
	pub left: String,
	pub right: String,

	pub top_left: String,
	pub top_right: String,
	pub bottom_left: String,
	pub bottom_right: String,
}

impl<W: Widget> Widget for Border<W> {
	type WidgetMsg = W::WidgetMsg;

	fn update(
		&mut self,
		message: Msg<Self::WidgetMsg>,
	) -> Vec<Cmd<Self::WidgetMsg>> {
		self.child.update(message)
	}

	fn set_width(&mut self, width: Option<usize>) {
		self.width = width;

		self.child.set_width(self.width.map(|width| {
			width - self.left.len() - self.right.len()
		}));
	}

	fn set_height(&mut self, height: Option<usize>) {
		self.height = height;

		self.child.set_height(self.height.map(|height| {
			height - self.top.lines().count()
				- self.bottom.lines().count()
		}));
	}

	fn height(&self) -> usize {
		if let Some(height) = self.height {
			return height;
		}

		self.child.height()
			+ self.top.lines().count()
			+ self.bottom.lines().count()
	}

	fn width(&self) -> usize {
		if let Some(width) = self.width {
			return width;
		}

		self.child.width()
			+ self.right.lines().count()
			+ self.left.lines().count()
	}

	fn lines(&self) -> impl Iterator<Item = &str> {
		self.child.lines()
	}
}

struct BorderLines<'a> {
	child: &'a dyn Iterator<Item = &'a str>,

	top: Lines<'a>,
	bottom: Lines<'a>,
	left: Lines<'a>,
	right: Lines<'a>,

	top_left: Lines<'a>,
	top_right: Lines<'a>,
	bottom_left: Lines<'a>,
	bottom_right: Lines<'a>,
}

impl<'a> Iterator for BorderLines<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}
