use crossterm::event::{Event, KeyCode, MouseEventKind};
use ulmus::{
	widget::{Flexbox, Size, Text, Widget},
	Command, Message, Model, Program,
};

struct Counter {
	count: i64,
}

impl Counter {
	fn new(count: i64) -> Counter {
		Counter { count }
	}
}

#[derive(Clone, Copy)]
enum Action {
	Increment,
	Decrement,
}

fn button(title: &str, action: Action) -> Box<Text> {
	Text::new_with(title.to_owned(), move |message| {
		if matches!(message.kind, MouseEventKind::Down(_)) {
			Message::new(action)
		} else {
			Message::empty()
		}
	})
}

impl Model for Counter {
	fn init(&self) -> Vec<Command> {
		vec![]
	}

	fn update(&mut self, message: Message) -> Vec<Command> {
		if let Some(event) = message.as_ref::<Event>() {
			match event {
				Event::Key(key_event) => match key_event.code {
					KeyCode::Char('+') => {
						self.count += 1;
					}
					KeyCode::Char('-') => {
						self.count -= 1;
					}
					KeyCode::Esc => {
						return vec![Command::Quit];
					}
					_ => {}
				},
				_ => {}
			}
		}

		vec![]
	}

	fn view(&self) -> Box<dyn Widget> {
		Flexbox::vertical(
			vec![
				Text::new(self.count.to_string()),
				Flexbox::horizontal(
					vec![
						button(
							"Increment",
							Action::Increment,
						),
						button(
							"Decrement",
							Action::Decrement,
						),
					],
					vec![Size::Auto, Size::Auto],
				),
			],
			vec![Size::Auto, Size::Auto],
		)
	}
}

fn main() {
	let mut model = Counter::new(0);

	let program = Program::default().enable_mouse();

	program.run(&mut model).unwrap();

	println!("The counter value on exit was {}", model.count);
}
