use crossterm::{cursor, event::MouseEvent, Command as _};

use std::fmt::Write;

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
			// TODO: truncate lines
			// This is a tough one.  The length of a line can be
			// approximated by stripping ANSI codes and using
			// `unicode-width`.  Truncating is harder, as those two
			// stages have to be combined.  I'll probably have to
			// write a custom parser using `vte`.
			out.write_str(line);

			cursor::MoveToColumn(area.x).write_ansi(&mut out);
			cursor::MoveDown(1).write_ansi(&mut out);
		}

		out
	}

	fn process_mouse(&self, event: MouseEvent, area: Area) -> Message {
		if !area.contains(event) {
			return Message::empty();
		}

		let Some(ref on_click) = self.on_click else {
			return Message::empty();
		};

		on_click(event)
	}
}
