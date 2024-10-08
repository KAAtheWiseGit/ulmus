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
