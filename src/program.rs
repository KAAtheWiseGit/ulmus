use crossterm::{
	cursor::{
		self, Hide as CursorHide, MoveTo, MoveToNextLine,
		RestorePosition, SavePosition, Show as CursorShow,
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
	io::{stdout, Result, StdoutLock, Write},
	sync::mpsc,
	thread,
};

use crate::interface::{Cmd, Msg, Subroutine, TermCommand, TermCommandImpl};

pub struct Program {
	show_cursor: bool,
	inline: bool,
}

impl Default for Program {
	fn default() -> Self {
		Self {
			show_cursor: false,
			inline: false,
		}
	}
}

impl Program {
	pub fn show_cursor(mut self) -> Self {
		self.show_cursor = true;
		self
	}

	pub fn inline(mut self) -> Self {
		self.inline = true;
		self
	}

	pub fn run<M, T>(&self, model: &mut M) -> Result<()>
	where
		M: crate::Model<CustomMsg = T>,
		T: Send + 'static,
	{
		let mut stdout = stdout().lock();
		let (sender, reciever) = mpsc::channel::<Msg<T>>();
		let top_row = if self.inline {
			cursor::position()?.1
		} else {
			0
		};

		self.init_term(&mut stdout)?;

		let mut threads = vec![];
		threads.push(run_subroutine(
			crossterm_subroutine(),
			sender.clone(),
		));

		let commands = model.init();
		for command in commands {
			// XXX: code duplication
			match command {
				Cmd::Term(tc) => queue_tc(&mut stdout, tc)?,
				Cmd::Quit => {
					// Quittin on `init` is weird and
					// returning would skip cleanup, so it's
					// ignored.
				}
				Cmd::Subroutine(subroutine) => {
					run_subroutine(
						subroutine,
						sender.clone(),
					);
				}
			}
		}
		// `init` will be flushed by the first draw

		'event: loop {
			let view = model.view();
			draw(&mut stdout, view.as_ref(), top_row)?;
			drop(view);

			let Ok(message) = reciever.recv() else {
				break;
			};

			let commands = model.update(message);
			for command in commands {
				match command {
					Cmd::Term(tc) => {
						queue_tc(&mut stdout, tc)?;
					}
					Cmd::Quit => break 'event,
					Cmd::Subroutine(subroutine) => {
						run_subroutine(
							subroutine,
							sender.clone(),
						);
					}
				}
			}
		}

		self.deinit_term(&mut stdout)?;

		Ok(())
	}

	fn init_term(&self, stdout: &mut StdoutLock) -> Result<()> {
		if !self.inline {
			stdout.execute(EnterAlternateScreen)?;
		}
		enable_raw_mode()?;
		if !self.show_cursor {
			stdout.execute(CursorHide)?;
		}
		Ok(())
	}

	fn deinit_term(&self, stdout: &mut StdoutLock) -> Result<()> {
		if !self.show_cursor {
			stdout.execute(CursorShow)?;
		}
		disable_raw_mode()?;
		if !self.inline {
			stdout.execute(LeaveAlternateScreen)?;
		}
		Ok(())
	}
}

// XXX: perhaps this should queue commands instead
fn queue_tc(stdout: &mut StdoutLock, tc: TermCommand) -> Result<()> {
	let tc: TermCommandImpl = tc.into();
	stdout.queue(tc)?;
	Ok(())
}

fn run_subroutine<T>(
	subroutine: Subroutine<T>,
	sender: mpsc::Sender<Msg<T>>,
) -> thread::JoinHandle<()>
where
	T: Send + 'static,
{
	thread::spawn(move || subroutine(sender))
}

/// A subroutine which reads crossterm events.
///
/// # Safety
///
/// Only this subroutine is allowed to call crossterm's `read` or `poll`.
fn crossterm_subroutine<T>() -> Subroutine<T>
where
	T: Send + 'static,
{
	Box::new(move |sender| {
		while let Ok(event) = crossterm_read() {
			if sender.send(Msg::Term(event)).is_err() {
				return;
			}
		}
	})
}

fn draw(stdout: &mut StdoutLock, view: &str, top_row: u16) -> Result<()> {
	let height = terminal_size().unwrap().1 - top_row;

	stdout.queue(SavePosition)?;
	stdout.queue(MoveTo(0, top_row))?;

	// Overwrite the view instead of clearing it to avoid flickering.  We do
	// need to clear the bottom and the rest of the line, as they might've
	// not been overwritten.
	//
	// https://www.textualize.io/blog/7-things-ive-learned-building-a-modern-tui-framework/

	for (row, line) in view.lines().enumerate() {
		if row >= height.into() {
			break;
		}

		stdout.queue(Print(line))?;
		stdout.queue(Clear(ClearType::UntilNewLine))?;
		stdout.queue(MoveToNextLine(1))?;
	}

	stdout.queue(Clear(ClearType::FromCursorDown))?;
	stdout.queue(RestorePosition)?;

	stdout.flush()?;

	Ok(())
}
