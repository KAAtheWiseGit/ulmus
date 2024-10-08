use super::Widget;
use crate::Reactive;

pub enum Direction {
	Row,
	Column,
}

/// Describes how the available space will be allocated between widgets.  `Auto`
/// has the highest priority, then goes `Length`, and, finally, `Fraction`.
pub enum Size {
	/// As much size as the widget requests.
	Auto,
	/// A proportion of the space remaining after the `Length` and `Auto`
	/// sizes have been allocated.  For example, if two widgets request
	/// `Fraction(2)`, each will get half of the remaning space.
	Fraction(usize),
	/// Literal length in cells.
	Length(usize),
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
	type CustomMsg = ();

	fn update(
		&mut self,
		message: crate::Msg<Self::CustomMsg>,
	) -> Vec<crate::Cmd<Self::CustomMsg>> {
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

impl crossterm::Command for Flexbox {
	fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
		todo!()
	}
}
