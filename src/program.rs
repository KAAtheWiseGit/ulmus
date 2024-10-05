use crossterm::{
	cursor::{MoveTo, MoveToNextLine},
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

use crate::interface::{Cmd, Msg};

pub struct Program<M, T>
where
	M: crate::Model<CustomMsg = T>,
	T: Send + 'static,
{
	model: M,
	reciever: mpsc::Receiver<Msg<T>>,
	sender: mpsc::Sender<Msg<T>>,
}

impl<M, T> Program<M, T>
where
	M: crate::Model<CustomMsg = T>,
	T: Send + 'static,
{
	pub fn new(model: M) -> Self {
		let (sender, reciever) = mpsc::channel::<Msg<T>>();

		Self {
			model,
			reciever,
			sender,
		}
	}

	pub fn run(&mut self) {
		let mut stdout = stdout();

		// Setup the TUI view
		stdout.execute(EnterAlternateScreen);
		enable_raw_mode().unwrap();
		stdout.execute(Clear(ClearType::All));

		let mut threads = vec![];
		threads.push(spawn_crossterm(self.sender.clone()));

		loop {
			let Ok(message) = self.reciever.recv() else {
				break;
			};

			match self.model.update(message) {
				Cmd::Term => {
					// TODO execute the crossterm command
				}
				Cmd::Quit => {
					break;
				}
				Cmd::Subroutine(subroutine) => {
					let sender = self.sender.clone();
					let handle = thread::spawn(move || {
						subroutine(sender);
					});
					threads.push(handle);
				}
			}

			let view = self.model.view();
			draw(&mut stdout, view.as_ref());
		}

		for handle in threads {
			handle.join();
		}

		// Restore the terminal view
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
			sender.send(Msg::Term(event));
		}
	})
}

fn draw(stdout: &mut Stdout, view: &str) {
	let height = terminal_size().unwrap().1;
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
