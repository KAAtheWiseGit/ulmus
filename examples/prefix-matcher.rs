use ulmus::{Cmd, Model, Msg, Program};

use crossterm::{
	cursor,
	event::{Event, KeyCode, KeyModifiers},
};
use walkdir::WalkDir;

struct PrefixMatcher {
	query: String,
	paths: Vec<String>,
}

impl PrefixMatcher {
	fn new() -> Self {
		let paths = WalkDir::new(".")
			.into_iter()
			.filter_map(|e| e.ok())
			.map(|e| e.path().to_string_lossy().into_owned())
			.into_iter()
			.collect();

		return Self {
			query: String::new(),
			paths,
		};
	}
}

impl Model for PrefixMatcher {
	type CustomMsg = ();

	fn init(&self) -> Vec<Cmd<Self::CustomMsg>> {
		vec![
			// By default, Ulmus hides the cursor.  Turn it back on
			Cmd::Term(cursor::Show.into()),
			// Move the cursor to the start of the query
			Cmd::Term(cursor::MoveTo(2, 0).into()),
		]
	}

	fn update(
		&mut self,
		message: Msg<Self::CustomMsg>,
	) -> Vec<Cmd<Self::CustomMsg>> {
		match message {
			Msg::Term(Event::Key(key_event)) => {
				let is_ctrl = key_event
					.modifiers
					.contains(KeyModifiers::CONTROL);

				match key_event.code {
					KeyCode::Backspace => {
						self.query.pop();
					}
					KeyCode::Char('u') if is_ctrl => {
						self.query.clear();
					}
					KeyCode::Char(c) => {
						self.query.push(c);
					}
					KeyCode::Esc => {
						return vec![Cmd::Quit];
					}
					_ => {}
				}
			}
			_ => {}
		}

		// Moves the cursor to the end of the query.
		let cursor_command = Cmd::Term(
			cursor::MoveTo(self.query.len() as u16 + 2, 0).into(),
		);
		return vec![cursor_command];
	}

	fn view(&self) -> impl AsRef<str> {
		let matched_paths: Vec<String> = self
			.paths
			.clone()
			.into_iter()
			.filter(|p| p.starts_with(&self.query))
			.collect();
		let matched_paths = matched_paths.join("\n");

		return format!("> {}\n{}", &self.query, matched_paths);
	}
}

fn main() -> std::io::Result<()> {
	let program = Program::default();
	let mut model = PrefixMatcher::new();
	program.run(&mut model)?;
	Ok(())
}
