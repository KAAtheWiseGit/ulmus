use ulmus::{Cmd, Model, Msg, Program};

use crossterm::{
	cursor,
	event::{Event, KeyCode},
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
		// By default, Ulmus hides the cursor.  This enables it back.
		vec![Cmd::Term(cursor::Show.into())]
	}

	fn update(
		&mut self,
		message: Msg<Self::CustomMsg>,
	) -> Vec<Cmd<Self::CustomMsg>> {
		match message {
			Msg::Term(Event::Key(key_event)) => {
				match key_event.code {
					KeyCode::Backspace => {
						self.query.pop();
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

		return vec![];
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

fn main() {
	let program = Program::new();
	let mut model = PrefixMatcher::new();
	program.run(&mut model);
}
