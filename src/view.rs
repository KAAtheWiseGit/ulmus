pub struct View {
	buffer: String,
}

impl View {
	pub fn new() -> Self {
		Self {
			buffer: String::new(),
		}
	}
}

impl<T: crossterm::Command> From<T> for View {
	fn from(value: T) -> Self {
		let mut buffer = String::new();
		if value.write_ansi(&mut buffer).is_err() {
			unreachable!("This operation should be infallible");
		}
		Self { buffer }
	}
}

pub(crate) struct ViewCommand {
	buffer: String,
}

impl crossterm::Command for ViewCommand {
	fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
		f.write_str(&self.buffer)
	}
}

impl View {
	pub(crate) fn to_command(self) -> ViewCommand {
		ViewCommand {
			buffer: self.buffer,
		}
	}
}
