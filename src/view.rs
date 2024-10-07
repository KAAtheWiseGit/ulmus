use crossterm::cursor;

use std::fmt::{self, Write};

trait View {
	type IntoCommand<'a>: crossterm::Command
	where
		Self: 'a;

	fn into_command(&self) -> Self::IntoCommand<'_>;
}

impl<T: AsRef<str>> View for T {
	type IntoCommand<'a> = StrView<'a> where Self: 'a;

	fn into_command(&self) -> Self::IntoCommand<'_> {
		todo!()
	}
}

struct StrView<'a> {
	source: &'a str,
}

impl crossterm::Command for StrView<'_> {
	fn write_ansi(&self, f: &mut impl Write) -> fmt::Result {
		cursor::SavePosition.write_ansi(f)?;

		for line in self.source.lines() {
			f.write_str(line)?;
			cursor::MoveDown(1).write_ansi(f)?;
		}

		cursor::RestorePosition.write_ansi(f)?;

		Ok(())
	}
}
