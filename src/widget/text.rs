use crossterm::cursor;

use std::fmt::{Result, Write};

use super::Widget;
use crate::{Cmd, Msg, Reactive};

pub struct Text {
	content: String,

	width: Option<usize>,
	height: Option<usize>,
}

impl Text {
	pub fn from(content: String) -> Self {
		Self {
			content,
			width: None,
			height: None,
		}
	}
}

impl Reactive for Text {
	type CustomMsg = ();

	fn update(
		&mut self,
		_: Msg<Self::CustomMsg>,
	) -> Vec<Cmd<Self::CustomMsg>> {
		vec![]
	}
}

impl Widget for Text {
	fn set_width(&mut self, width: Option<usize>) {
		self.width = width;
	}

	fn set_height(&mut self, height: Option<usize>) {
		self.height = height;
	}

	fn width(&self) -> usize {
		self.width.unwrap_or(
			self.content
				.lines()
				// TODO: handle width
				.map(|s| s.chars().count())
				.max()
				// If there are no lines, the width is 0
				.unwrap_or(0),
		)
	}

	fn height(&self) -> usize {
		if let Some(height) = self.height {
			return height;
		}

		self.content.lines().count()
	}
}

impl crossterm::Command for Text {
	fn write_ansi(&self, f: &mut impl Write) -> Result {
		for (i, line) in self.content.lines().enumerate() {
			if self.height.is_some_and(|height| height == i) {
				break;
			}

			cursor::SavePosition.write_ansi(f)?;
			if i > 0 {
				cursor::MoveDown(i as u16).write_ansi(f)?;
			}

			fit_write_str(line, self.width(), f)?;
			cursor::RestorePosition.write_ansi(f)?;
		}

		Ok(())
	}
}

fn fit_write_str(s: &str, len: usize, f: &mut impl Write) -> Result {
	let s_len = s.chars().count();

	if s_len < len {
		f.write_str(s)?;
		f.write_str(&" ".repeat(len - s_len))?;
	} else if s_len > len {
		for (i, ch) in s.chars().enumerate() {
			if i == len {
				break;
			}
			f.write_char(ch)?;
		}
	} else {
		f.write_str(s)?;
	}

	Ok(())
}
