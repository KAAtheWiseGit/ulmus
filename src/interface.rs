use crossterm::{event::Event as CrosstermEvent, Command as CrosstermCommand};

use std::sync::mpsc;

/// The `Model` trait describes the behaviour of your TUI.
pub trait Model {
	/// Returns [commands][`Cmd`] which will be ran on startup, before the
	/// first render.
	fn init(&self) -> Vec<Cmd<Self::CustomMsg>>;

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
