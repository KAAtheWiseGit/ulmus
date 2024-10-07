use crossterm::cursor;

use std::fmt::{self, Write};

pub trait View {
	type IntoCommand<'a>: crossterm::Command
	where
		Self: 'a;

	fn as_command(&self) -> Self::IntoCommand<'_>;
}

impl<T: AsRef<str>> View for T {
	type IntoCommand<'a> = StrView<'a> where Self: 'a;

	fn as_command(&self) -> Self::IntoCommand<'_> {
		StrView::from(self.as_ref())
	}
}

pub struct StrView<'a> {
	source: &'a str,
}

impl<'a> StrView<'a> {
	fn from(source: &'a str) -> StrView<'a> {
		Self { source }
	}
}

impl crossterm::Command for StrView<'_> {
	fn write_ansi(&self, f: &mut impl Write) -> fmt::Result {
		cursor::SavePosition.write_ansi(f)?;

		for (i, line) in self.source.lines().enumerate() {
			cursor::SavePosition.write_ansi(f)?;
			if i > 0 {
				cursor::MoveDown(i as u16).write_ansi(f)?;
			}
			f.write_str(line)?;
			cursor::RestorePosition.write_ansi(f)?;
		}

		cursor::RestorePosition.write_ansi(f)?;

		Ok(())
	}
}
