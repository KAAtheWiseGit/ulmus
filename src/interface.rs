use crossterm::{event::Event as CrosstermEvent, Command as CrosstermCommand};

use std::sync::mpsc;

/// A task launched in a separate thread.  Using the passed sender it can send
/// model's [custom messages][Model#associatedtype.CustomMsg] back to the main
/// thread, passing them to [`update`][Model#tymethod.update].
pub type Subroutine<T> = Box<dyn FnOnce(mpsc::Sender<Msg<T>>) + Send>;

/// A message from the framework to the user model.
pub enum Msg<T: Send + 'static> {
	/// Terminal event, received via [crossterm's
	/// `read`][crossterm::event::read].  It can be configured via
	/// [`Program`] options or commands on
	/// [`init`][Model#tymethod.init][^note].
	///
	///
	/// [^note]: Technically, it can be sent with any `Cmd`, even inside
	///   [`update`][Model#tymethod.update].  This can be used to
	///   dynamically enable and disable mouse or bracketed paste.
	Term(CrosstermEvent),
	/// A custom message, sent by one of the launched
	/// [subroutine][`Subroutine`].
	Custom(T),
}

/// Commands are returned by model in [`init`][Model#tymethod.init] and
/// [`update`][Model#tymethod.update] and can be used to change control the
/// event loop.
pub enum Cmd<T: Send + 'static> {
	/// Execute an arbitrary [crossterm command][crossterm::Command].
	/// Because the latter isn't object-safe, commands must be converted
	/// into [`TermCommand`] first.
	///
	/// The command execution will be queued and flushed with the next
	/// render.  Note that the commands are queued before the redraw, so
	/// they can't be used for updating the view contents.
	Term(TermCommand),
	/// Immediately shuts down the program.
	Quit,
	/// Launches a subroutine.  Note that this command can be sent at any
	/// time, even in [`update`][Model#tymethod.update] and that the
	/// subroutine can start sending messages immediately.
	Subroutine(Subroutine<T>),
}

/// The `Model` trait describes the behaviour of your TUI.
pub trait Model {
	/// A custom message, which can be returned by user-launched
	/// [subroutines][`Subroutine`].  If there are several possible
	/// messages, `CustomMsg` should probably be an enum wrapping all of
	/// them.
	///
	/// As it will be sent across thread boundaries, it has to be `Send` and
	/// `'static`.
	type CustomMsg: Sized + Send + 'static;

	/// Returns [commands][`Cmd`] which will be ran on startup, before the
	/// first render.
	fn init(&self) -> Vec<Cmd<Self::CustomMsg>>;

	/// Updates the model in response to a [message][`Msg`].  This is the
	/// only time in the model's lifecycle when it can be mutated.
	fn update(
		&mut self,
		message: Msg<Self::CustomMsg>,
	) -> Vec<Cmd<Self::CustomMsg>>;

	/// Returns a string, which will be rendered to the terminal, line by
	/// line[^lines].
	///
	/// It is the implementers responsibility to:
	///
	/// - Style the view.
	/// - Ensure each line is no longer than the width of the terminal.
	///
	/// But Ulmus will gracefully handle excessive number of lines and cut
	/// them off at the bottom of the terminal.
	///
	///
	/// [^lines]: Splitting is done using [`lines`][str#method.lines].
	fn view(&self) -> impl AsRef<str>;
}

/// A conversion type, required because crossterm's [`Command`][cmd] is not
/// object-safe.  It's only used to send crossterm commands via [`Cmd::Term`].
///
/// [cmd]: crossterm::Command
pub struct TermCommand(String);

/// A hacky workaround because `TermCommand` can't implement `From<T:
/// CrosstermCommand>` and `CrosstermCommand` at  the same time, as it causes it
/// to conflict with the `From<T> for T` implementation from the standard
/// library.
pub struct TermCommandImpl(String);

impl<T: CrosstermCommand> From<T> for TermCommand {
	fn from(value: T) -> Self {
		let mut buffer = String::new();
		if value.write_ansi(&mut buffer).is_err() {
			unreachable!("`String` is an infallible writer");
		}
		Self(buffer)
	}
}

impl Into<TermCommandImpl> for TermCommand {
	fn into(self) -> TermCommandImpl {
		TermCommandImpl(self.0)
	}
}

impl CrosstermCommand for TermCommandImpl {
	fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
		f.write_str(&self.0)
	}
}
