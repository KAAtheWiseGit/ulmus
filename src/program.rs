use crossterm::{cursor, event, terminal, ExecutableCommand, QueueableCommand};

use std::{
	io::{stdout, Result, Write},
	sync::mpsc,
	thread,
};

use crate::reactive::TermCommandImpl;
use crate::{Cmd, Model, Msg, Subroutine, TermCommand};

/// A program which runs the [user model][Model].
#[derive(Clone, Copy)]
pub struct Program {
	show_cursor: bool,
	inline: bool,
	enable_mouse: bool,
	enable_paste: bool,
}

impl Default for Program {
	/// The default program:
	///
	/// - Hides the cursor.
	/// - Uses alternate screen.
	/// - Doesn't enable mouse or bracketed paste events.
	fn default() -> Self {
		Self {
			show_cursor: false,
			inline: false,
			enable_mouse: false,
			enable_paste: false,
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

	/// Enables [bracketed paste mode][bpm].
	///
	/// [bpm]: https://en.wikipedia.org/wiki/Bracketed-paste
	pub fn enable_paste(mut self) -> Self {
		self.enable_paste = true;
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
		#[allow(unused)]
		let top_row = if self.inline {
			cursor::position()?.1
		} else {
			0
		};

		set_panic_hook(*self);
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

			let cmd = model.view();
			stdout.queue(cmd)?;
			stdout.flush()?;

			let Ok(message) = reciever.recv() else {
				break;
			};
			commands = model.update(message);
		}

		self.deinit_term(&mut stdout)?;

		Ok(())
	}

	fn init_term(&self, term: &mut impl Write) -> Result<()> {
		if !self.inline {
			term.execute(terminal::EnterAlternateScreen)?;
		}
		terminal::enable_raw_mode()?;
		if !self.show_cursor {
			term.execute(cursor::Hide)?;
		}
		if self.enable_mouse {
			term.execute(event::EnableMouseCapture)?;
		}
		if self.enable_paste {
			term.execute(event::EnableBracketedPaste)?;
		}
		Ok(())
	}

	fn deinit_term(&self, term: &mut impl Write) -> Result<()> {
		if !self.show_cursor {
			term.execute(cursor::Show)?;
		}
		terminal::disable_raw_mode()?;
		if !self.inline {
			term.execute(terminal::LeaveAlternateScreen)?;
		}
		if self.enable_mouse {
			term.execute(event::DisableMouseCapture)?;
		}
		if self.enable_paste {
			term.execute(event::DisableBracketedPaste)?;
		}
		Ok(())
	}
}

fn queue_tc(term: &mut impl Write, tc: TermCommand) -> Result<()> {
	let tc: TermCommandImpl = tc.into();
	term.queue(tc)?;
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

fn set_panic_hook(program: Program) {
	let old_hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(move |info| {
		if program.deinit_term(&mut stdout()).is_err() {
			eprintln!("Sorry, failed to restore terminal.  It'll probably be all jumbled up now.")
		};
		old_hook(info);
	}))
}
