use crossterm::{cursor, event::MouseEvent, Command as _};

use std::fmt::Write;

use super::Widget;
use crate::{Area, Message};

pub struct Text {
	content: String,

	on_mouse: Option<Box<dyn Fn(MouseEvent) -> Message>>,
}

impl Text {
	pub fn new<S>(content: S) -> Box<Text>
	where
		S: AsRef<str>,
	{
		Box::new(Text {
			content: content.as_ref().to_owned(),
			on_mouse: None,
		})
	}

	pub fn on_mouse<F>(mut self: Box<Text>, handler: F) -> Box<Text>
	where
		F: Fn(MouseEvent) -> Message + 'static,
	{
		self.on_mouse = Some(Box::new(handler));
		self
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
		let Some(ref on_click) = self.on_mouse else {
			return Message::empty();
		};

		on_click(event)
	}
}
