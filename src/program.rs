use crossterm::{
	cursor::{
		Hide as CursorHide, MoveTo, MoveToNextLine, RestorePosition,
		SavePosition, Show as CursorShow,
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

use crate::interface::{Cmd, Msg, Subroutine, TermCommand, TermCommandImpl};

pub struct Program {}

impl Program {
	pub fn new() -> Self {
		Self {}
	}

	pub fn run<M, T>(&self, model: &mut M)
	where
		M: crate::Model<CustomMsg = T>,
		T: Send + 'static,
	{
		let mut stdout = stdout();
		let (sender, reciever) = mpsc::channel::<Msg<T>>();

		// Setup the TUI view
		stdout.execute(EnterAlternateScreen);
		enable_raw_mode().unwrap();
		stdout.execute(Clear(ClearType::All));
		stdout.execute(CursorHide);

		let mut threads = vec![];
		threads.push(run_subroutine(
			crossterm_subroutine(),
			sender.clone(),
		));

		let commands = model.init();
		for command in commands {
			// XXX: code duplication
			match command {
				Cmd::Term(tc) => exec_tc(&mut stdout, tc),
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

		'event: loop {
			let view = model.view();
			draw(&mut stdout, view.as_ref());
			drop(view);

			let Ok(message) = reciever.recv() else {
				break;
			};

			let commands = model.update(message);
			for command in commands {
				match command {
					Cmd::Term(tc) => {
						exec_tc(&mut stdout, tc)
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

		// Restore the terminal view
		stdout.execute(CursorShow);
		disable_raw_mode().unwrap();
		stdout.execute(LeaveAlternateScreen);
	}
}

// XXX: perhaps this should queue commands instead
fn exec_tc(stdout: &mut Stdout, tc: TermCommand) {
	let tc: TermCommandImpl = tc.into();
	stdout.execute(tc);
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

fn draw(stdout: &mut Stdout, view: &str) {
	let height = terminal_size().unwrap().1;

	stdout.queue(SavePosition);
	stdout.queue(Clear(ClearType::All));
	stdout.queue(MoveTo(0, 0));

	for (row, line) in view.lines().enumerate() {
		if row >= height.into() {
			break;
		}

		stdout.queue(Print(line));
		stdout.queue(MoveToNextLine(1));
	}

	stdout.queue(RestorePosition);
	stdout.flush();
}
