use crossterm::{cursor, style::Print, Command as _};

use std::{
	cmp::Ordering,
	fmt::{Result, Write},
};

use super::Widget;
use crate::{Command, Message, Reactive, View};

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
	fn update(&mut self, _: Message) -> Vec<Command> {
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

	fn get_width(&self) -> usize {
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

	fn get_height(&self) -> usize {
		if let Some(height) = self.height {
			return height;
		}

		self.content.lines().count()
	}
}

impl View for Text {
	#[allow(unused_must_use)]
	fn view(&self) -> Print<String> {
		let mut out = String::new();
		for (i, line) in self.content.lines().enumerate() {
			if self.height.is_some_and(|height| height == i) {
				break;
			}

			cursor::SavePosition.write_ansi(&mut out);
			if i > 0 {
				cursor::MoveDown(i as u16).write_ansi(&mut out);
			}

			fit_write_str(line, self.get_width(), &mut out);
			cursor::RestorePosition.write_ansi(&mut out);
		}

		Print(out)
	}
}

fn fit_write_str(s: &str, len: usize, f: &mut impl Write) -> Result {
	let s_len = s.chars().count();

	match s_len.cmp(&len) {
		Ordering::Less => {
			f.write_str(s)?;
			f.write_str(&" ".repeat(len - s_len))?;
		}
		Ordering::Greater => {
			for (i, ch) in s.chars().enumerate() {
				if i == len {
					break;
				}
				f.write_char(ch)?;
			}
		}
		Ordering::Equal => {
			f.write_str(s)?;
		}
	}

	Ok(())
}
