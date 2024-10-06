use ulmus::{Cmd, Model, Msg, Program};

use crossterm::event::{Event, KeyCode};

struct InlinePicker {
	items: Vec<&'static str>,
	picks: Vec<bool>,
	focus: usize,
}

impl InlinePicker {
	fn new() -> Self {
		Self {
			items: vec!["Apples", "Lemons", "Watermelons"],
			picks: [false].repeat(3),
			focus: 0,
		}
	}
}

impl Model for InlinePicker {
	type CustomMsg = ();

	fn init(&self) -> Vec<Cmd<Self::CustomMsg>> {
		vec![]
	}

	fn update(
		&mut self,
		message: Msg<Self::CustomMsg>,
	) -> Vec<Cmd<Self::CustomMsg>> {
		if let Msg::Term(Event::Key(key_event)) = message {
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
		};

		vec![]
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

		out
	}
}

fn main() -> std::io::Result<()> {
	let program = Program::default().inline();

	let mut model = InlinePicker::new();

	program.run(&mut model)?;

	Ok(())
}
