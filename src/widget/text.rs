use super::Widget;
use crate::{Cmd, Msg};

pub struct Text {
	pub content: String,

	width: Option<usize>,
	height: Option<usize>,
}

impl Widget for Text {
	type WidgetMsg = ();

	fn update(
		&mut self,
		_: Msg<Self::WidgetMsg>,
	) -> Vec<Cmd<Self::WidgetMsg>> {
		vec![]
	}

	fn set_width(&mut self, width: Option<usize>) {
		self.width = width;
	}

	fn set_height(&mut self, height: Option<usize>) {
		self.height = height;
	}

	fn width(&self) -> usize {
		if let Some(width) = self.width {
			return width;
		}

		self.content
			.lines()
			// TODO: handle width
			.map(|s| s.chars().count())
			.max()
			// If there are no lines, the width is 0
			.unwrap_or(0)
	}

	fn height(&self) -> usize {
		if let Some(height) = self.height {
			return height;
		}

		self.content.lines().count()
	}

	fn lines(&self) -> impl Iterator<Item = &str> {
		self.content.lines()
	}
}
