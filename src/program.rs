use crossterm::{
	terminal::{
		disable_raw_mode, enable_raw_mode, Clear, ClearType,
		EnterAlternateScreen, LeaveAlternateScreen,
	},
	ExecutableCommand,
};

use std::{io::stdout, sync::mpsc};

use crate::interface::{Cmd, Msg};

pub struct Program<M, T>
where
	M: crate::Model<CustomMsg = T>,
	T: Sized,
{
	model: M,
	reciever: mpsc::Receiver<Msg<T>>,
	sender: mpsc::Sender<Msg<T>>,
}

impl<M, T> Program<M, T>
where
	M: crate::Model<CustomMsg = T>,
	T: Sized,
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
					// TODO
				}
			}

			let _ = self.model.view();
			// TODO draw the output
		}

		// Restore the terminal view
		disable_raw_mode().unwrap();
		stdout.execute(LeaveAlternateScreen);
	}
}
