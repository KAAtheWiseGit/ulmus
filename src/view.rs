use crossterm::style::Print;

#[derive(Default, Clone)]
pub struct View {
	buffer: String,
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

impl View {
	pub(crate) fn into_command(self) -> Print<String> {
		Print(self.buffer)
	}

	pub(crate) fn as_command(&self) -> Print<&str> {
		Print(&self.buffer)
	}
}
