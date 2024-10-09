use crossterm::{cursor, event::MouseEvent, Command as _};

use std::{
	cmp::Ordering,
	fmt::{Result, Write},
};

use super::Widget;
use crate::{Area, Message};

pub struct Text {
	content: String,

	on_click: Option<Box<dyn Fn(MouseEvent) -> Message>>,
}

impl Text {
	pub fn new(content: String) -> Box<Text> {
		Box::new(Text {
			content,
			on_click: None,
		})
	}

	pub fn new_with<F>(content: String, on_click: F) -> Box<Text>
	where
		F: Fn(MouseEvent) -> Message + 'static,
	{
		Box::new(Text {
			content,
			on_click: Some(Box::new(on_click)),
		})
	}
}

impl Widget for Text {
	fn get_width_hint(&self) -> u16 {
		self.content
			.lines()
			// TODO: handle width
			.map(|s| s.chars().count())
			.max()
			// If there are no lines, the width is 0
			.unwrap_or(0) as u16
	}

	fn get_height_hint(&self) -> u16 {
		self.content.lines().count() as u16
	}

	#[allow(unused_must_use)]
	fn render(&self, area: Area) -> String {
		let mut out = String::new();
		// XXX: methods on area
		cursor::MoveTo(area.x, area.y).write_ansi(&mut out);

		for line in self.content.lines().take(area.height as usize) {
			fit_write_str(line, area.width, &mut out);

			cursor::MoveToColumn(area.x).write_ansi(&mut out);
			cursor::MoveDown(1).write_ansi(&mut out);
		}

		out
	}

	fn on_click(&self, area: Area, event: MouseEvent) -> Message {
		if !area.contains(event) {
			return Message::empty();
		}

		let Some(ref on_click) = self.on_click else {
			return Message::empty();
		};

		on_click(event)
	}
}

fn fit_write_str(s: &str, len: u16, f: &mut impl Write) -> Result {
	let len = len as usize;
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
