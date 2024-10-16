use crossterm::{
	cursor,
	event::{self, Event, MouseEventKind},
	queue,
	style::Print,
	terminal::{self, ClearType},
	ExecutableCommand,
};

use std::{
	io::{stdout, Result, Write},
	sync::mpsc,
	thread,
};

use crate::{Area, Command, Message, Model, Subroutine};

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
	/// Don't hide the cursor.  If enabled, it becomes the [model's][Model]
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
	/// [`Command::Quit`] command.
	pub fn run<M: Model>(&self, model: &mut M) -> Result<()> {
		let mut stdout = stdout().lock();
		let (sender, reciever) = mpsc::channel::<Message>();
		// TODO resizing
		let area = self.get_area()?;

		set_panic_hook(*self);
		self.init_term(&mut stdout)?;

		run_subroutine(crossterm_subroutine(), sender.clone());

		let mut commands = model.init();
		'event: loop {
			// A hack to move commands into the loop
			let iter = commands;
			for command in iter {
				match command {
					Command::SetCursor(x, y) => {
						stdout.execute(
							cursor::MoveTo(x, y),
						)?;
					}
					Command::Quit => break 'event,
					Command::Subroutine(subroutine) => {
						run_subroutine(
							subroutine,
							sender.clone(),
						);
					}
				}
			}

			let widget = model.view();
			let command = widget.render(area);
			self.draw(&mut stdout, area, command)?;
			stdout.flush()?;

			let Ok(message) = reciever.recv() else {
				break;
			};
			if let Some(Event::Mouse(event)) =
				message.as_ref::<Event>()
			{
				// If the event is a click outside of the TUI
				// area, ignore it.
				//
				// XXX: should mouse up and drag events be
				// ignored too?
				if matches!(event.kind, MouseEventKind::Down(_))
					&& !area.contains(*event)
				{
					commands = vec![];
					continue;
				}

				let msg = widget.process_mouse(*event, area);
				commands = model.update(msg);
				continue;
			};

			commands = model.update(message);
		}

		self.deinit_term(&mut stdout)?;
		drop(stdout);

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
		} else {
			term.execute(terminal::Clear(
				ClearType::FromCursorDown,
			))?;
		}
		if self.enable_mouse {
			term.execute(event::DisableMouseCapture)?;
		}
		if self.enable_paste {
			term.execute(event::DisableBracketedPaste)?;
		}
		Ok(())
	}

	fn get_area(&self) -> Result<Area> {
		let size = terminal::size()?;
		let cursor = if self.inline {
			cursor::position()?
		} else {
			(0, 0)
		};

		Ok(Area {
			x: cursor.0,
			y: cursor.1,
			width: size.0 - cursor.0,
			height: size.1 - cursor.1,
		})
	}

	fn draw(
		&self,
		stdout: &mut impl Write,
		area: Area,
		command: String,
	) -> Result<()> {
		queue!(
			stdout,
			cursor::SavePosition,
			cursor::MoveTo(area.x, area.y),
			terminal::Clear(ClearType::FromCursorDown),
			cursor::RestorePosition,
			Print(command),
		)?;
		Ok(())
	}
}

fn run_subroutine(
	subroutine: Subroutine,
	sender: mpsc::Sender<Message>,
) -> thread::JoinHandle<()> {
	thread::spawn(move || subroutine(sender))
}

/// A subroutine which reads crossterm events.
///
/// # Safety
///
/// Only this subroutine is allowed to call crossterm's `read` or `poll`.
fn crossterm_subroutine() -> Subroutine {
	Box::new(move |sender| {
		while let Ok(event) = event::read() {
			if sender.send(Message::new(event)).is_err() {
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
