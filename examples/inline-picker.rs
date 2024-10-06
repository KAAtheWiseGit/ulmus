use ulmus::{Cmd, Model, Msg, Program};

use crossterm::{
	cursor,
	event::{Event, KeyCode},
};

struct InlinePicker {
	items: Vec<&'static str>,
	picks: Vec<bool>,
	focus: usize,
}

impl InlinePicker {
	fn new() -> Self {
		return Self {
			items: vec!["Apples", "Lemons", "Watermelons"],
			picks: vec![false].repeat(3),
			focus: 0,
		};
	}
}

impl Model for InlinePicker {
	type CustomMsg = ();

	fn init(&self) -> Vec<Cmd<Self::CustomMsg>> {
		vec![
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
				match key_event.code {
					KeyCode::Esc => {
						return vec![Cmd::Quit];
					}
					KeyCode::Down => {
						if self.focus < 2 {
							self.focus += 1;
						}
					}
					KeyCode::Up => {
						if self.focus > 0 {
							self.focus -= 1;
						}
					}
					KeyCode::Enter | KeyCode::Char(' ') => {
						// flip
						self.picks[self.focus] ^= true;
					}
					_ => {}
				}
			}
			_ => {}
		}

		return vec![];
	}

	fn view(&self) -> impl AsRef<str> {
		let mut out = String::new();

		for i in 0..self.items.len() {
			let cursor = if self.focus == i { ">" } else { " " };
			let picked = if self.picks[i] { "x" } else { " " };

			out += &format!(
				" {cursor} [{picked}] {}\n",
				self.items[i]
			);
		}

		return out;
	}
}

fn main() -> std::io::Result<()> {
	let program = Program::default().inline();

	let mut model = InlinePicker::new();

	program.run(&mut model)?;

	Ok(())
}
