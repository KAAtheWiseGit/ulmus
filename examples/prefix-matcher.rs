use ulmus::{Cmd, Model, Msg, Program};

use crossterm::event::{Event, KeyCode};
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

	fn update(
		&mut self,
		message: Msg<Self::CustomMsg>,
	) -> Cmd<Self::CustomMsg> {
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
						return Cmd::Quit;
					}
					_ => {}
				}
			}
			_ => {}
		}

		return Cmd::Term;
	}

	fn view(&self) -> impl AsRef<str> {
		let matched_paths: Vec<String> = self
			.paths
			.clone()
			.into_iter()
			.filter(|p| p.starts_with(&self.query))
			.take(10)
			.collect();
		let matched_paths = matched_paths.join("\n");

		return format!("> {}\n{}", &self.query, matched_paths);
	}
}

fn main() {
	let mut program = Program::new(PrefixMatcher::new());
	program.run();
}
