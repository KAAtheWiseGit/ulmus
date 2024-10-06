use crossterm::{
	cursor::{
		Hide as CursorHide, MoveTo, MoveToNextLine, Show as CursorShow,
	},
	event::read as crossterm_read,
	style::Print,
	terminal::{
		disable_raw_mode, enable_raw_mode, size as terminal_size,
		Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
	},
	ExecutableCommand, QueueableCommand,
};

use std::{
	io::{stdout, Stdout, Write},
	sync::mpsc,
	thread,
};

use crate::interface::{Cmd, Msg, TermCommandImpl};

pub struct Program<M, T>
where
	M: crate::Model<CustomMsg = T>,
	T: Send + 'static,
{
	model: M,
}

impl<M, T> Program<M, T>
where
	M: crate::Model<CustomMsg = T>,
	T: Send + 'static,
{
	pub fn new(model: M) -> Self {
		Self { model }
	}

	pub fn run(&mut self) {
		let mut stdout = stdout();
		let (sender, reciever) = mpsc::channel::<Msg<T>>();

		// Setup the TUI view
		stdout.execute(EnterAlternateScreen);
		enable_raw_mode().unwrap();
		stdout.execute(Clear(ClearType::All));
		stdout.execute(CursorHide);

		let mut threads = vec![];
		threads.push(spawn_crossterm(sender.clone()));

		'event: loop {
			let view = self.model.view();
			draw(&mut stdout, view.as_ref());
			drop(view);

			let Ok(message) = reciever.recv() else {
				break;
			};

			let commands = self.model.update(message);
			for command in commands {
				#[cfg_attr(rustfmt, rustfmt_skip)]
				match command {
				Cmd::Term(term_command) => {
					let term_command: TermCommandImpl =
						term_command.into();
					stdout.execute(term_command);
				}
				Cmd::Quit => {
					break 'event;
				}
				Cmd::Subroutine(subroutine) => {
					let sender = sender.clone();
					let handle =
						thread::spawn(move || {
							subroutine(sender);
						});
					threads.push(handle);
				}
				}
			}
		}

		drop(reciever);

		// Restore the terminal view
		stdout.execute(CursorShow);
		disable_raw_mode().unwrap();
		stdout.execute(LeaveAlternateScreen);
	}
}

fn spawn_crossterm<T>(sender: mpsc::Sender<Msg<T>>) -> thread::JoinHandle<()>
where
	T: Send + 'static,
{
	thread::spawn(move || {
		while let Ok(event) = crossterm_read() {
			if sender.send(Msg::Term(event)).is_err() {
				return;
			}
		}
	})
}

fn draw(stdout: &mut Stdout, view: &str) {
	let height = terminal_size().unwrap().1;

	stdout.queue(Clear(ClearType::All));
	stdout.queue(MoveTo(0, 0));

	for (row, line) in view.lines().enumerate() {
		if row >= height.into() {
			break;
		}

		stdout.queue(Print(line));
		stdout.queue(MoveToNextLine(1));
	}

	stdout.flush();
}
