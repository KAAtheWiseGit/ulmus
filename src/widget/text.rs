use crossterm::{cursor, Command as _};

use std::{
	cmp::Ordering,
	fmt::{Result, Write},
};

use super::Widget;
use crate::{Area, Message, View};

pub struct Text {
	content: String,
}

impl Text {
	pub fn new(content: String) -> Box<Text> {
		Box::new(Text { content })
	}
}

impl Widget for Text {
	fn get_width_hint(&self) -> usize {
		self.content
			.lines()
			// TODO: handle width
			.map(|s| s.chars().count())
			.max()
			// If there are no lines, the width is 0
			.unwrap_or(0)
	}

	fn get_height_hint(&self) -> usize {
		self.content.lines().count()
	}

	fn on_click(&self, event: crossterm::event::MouseEvent) -> Message {
		todo!()
	}
}

impl View for Text {
	#[allow(unused_must_use)]
	fn view(&self, area: Area) -> String {
		let mut out = String::new();
		// XXX: methods on area
		cursor::MoveTo(area.x as u16, area.y as u16)
			.write_ansi(&mut out);

		for line in self.content.lines().take(area.height) {
			fit_write_str(line, area.width, &mut out);

			cursor::MoveToColumn(area.x as u16)
				.write_ansi(&mut out);
			cursor::MoveDown(1).write_ansi(&mut out);
		}

		out
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
