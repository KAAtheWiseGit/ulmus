use ulmus::{widget::Text, Command, Message, Model, Program, Reactive};

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
	fn init(&self) -> Vec<Command> {
		vec![]
	}

	fn view(&self) -> impl crossterm::Command {
		let mut out = String::new();

		for i in 0..self.items.len() {
			let cursor = if self.focus == i { ">" } else { " " };
			let picked = if self.picks[i] { "x" } else { " " };

			out += &format!(
				" {cursor} [{picked}] {}\n",
				self.items[i]
			);
		}

		Text::from(out)
	}
}

impl Reactive for InlinePicker {
	fn update(&mut self, message: Message) -> Vec<Command> {
		if let Some(Event::Key(key_event)) = message.as_ref::<Event>() {
			match key_event.code {
				KeyCode::Esc => {
					return vec![Command::Quit];
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
}

fn main() -> std::io::Result<()> {
	let program = Program::default().inline();

	let mut model = InlinePicker::new();

	program.run(&mut model)?;

	Ok(())
}
