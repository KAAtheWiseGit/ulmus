use crossterm::{
	cursor, event,
	style::Print,
	terminal::{self, ClearType},
	ExecutableCommand, QueueableCommand,
};

use std::{
	io::{stdout, Result, StdoutLock, Write},
	sync::mpsc,
	thread,
};

use crate::interface::{
	Cmd, Model, Msg, Subroutine, TermCommand, TermCommandImpl,
};

/// A program which runs the [user model][Model].
pub struct Program {
	show_cursor: bool,
	inline: bool,
	enable_mouse: bool,
}

impl Default for Program {
	/// The default program:
	///
	/// - Hides the cursor.
	/// - Uses alternate screen.
	/// - Doesn't enable mouse or bracketed paste.
	fn default() -> Self {
		Self {
			show_cursor: false,
			inline: false,
			enable_mouse: false,
		}
	}
}

impl Program {
	/// Don't hide the cursor.  If enabled, it becomes the
	/// [model's][Model]
	/// responsibility to set the cursor position.
	pub fn show_cursor(mut self) -> Self {
		self.show_cursor = true;
		self
	}

	/// Runs the TUI inline, from the row it was called on to the bottom of
	/// the terminal.  After the program finishes running only the TUI view
	/// will be cleared.
	pub fn inline(mut self) -> Self {
		self.inline = true;
		self
	}

	/// Enables receiving mouse events in [`update`][Model#tymethod.update].
	pub fn enable_mouse(mut self) -> Self {
		self.enable_mouse = true;
		self
	}

	/// Runs the model.  This function will block until the model returns a
	/// [`Cmd::Quit`] command.
	pub fn run<M, T>(&self, model: &mut M) -> Result<()>
	where
		M: Model<CustomMsg = T>,
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

		run_subroutine(crossterm_subroutine(), sender.clone());

		let mut commands = model.init();

		'event: loop {
			// A hack to move commands into the loop
			let iter = commands;
			for command in iter {
				match command {
					Cmd::Term(tc) => {
						queue_tc(&mut stdout, tc)?
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

			let view = model.view();
			draw(&mut stdout, view.as_ref(), top_row)?;
			drop(view);

			let Ok(message) = reciever.recv() else {
				break;
			};
			commands = model.update(message);
		}

		self.deinit_term(&mut stdout)?;

		Ok(())
	}

	fn init_term(&self, stdout: &mut StdoutLock) -> Result<()> {
		if !self.inline {
			stdout.execute(terminal::EnterAlternateScreen)?;
		}
		terminal::enable_raw_mode()?;
		if !self.show_cursor {
			stdout.execute(cursor::Hide)?;
		}
		if self.enable_mouse {
			stdout.execute(event::EnableMouseCapture)?;
		}
		Ok(())
	}

	fn deinit_term(&self, stdout: &mut StdoutLock) -> Result<()> {
		if !self.show_cursor {
			stdout.execute(cursor::Show)?;
		}
		terminal::disable_raw_mode()?;
		if !self.inline {
			stdout.execute(terminal::LeaveAlternateScreen)?;
		}
		if self.enable_mouse {
			stdout.execute(event::DisableMouseCapture)?;
		}
		Ok(())
	}
}

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
		while let Ok(event) = event::read() {
			if sender.send(Msg::Term(event)).is_err() {
				return;
			}
		}
	})
}

fn draw(stdout: &mut StdoutLock, view: &str, top_row: u16) -> Result<()> {
	let height = terminal::size().unwrap().1 - top_row;

	stdout.queue(cursor::SavePosition)?;
	stdout.queue(cursor::MoveTo(0, top_row))?;

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
		stdout.queue(terminal::Clear(
			terminal::ClearType::UntilNewLine,
		))?;
		stdout.queue(cursor::MoveToNextLine(1))?;
	}

	stdout.queue(terminal::Clear(ClearType::FromCursorDown))?;
	stdout.queue(cursor::RestorePosition)?;

	stdout.flush()?;

	Ok(())
}
