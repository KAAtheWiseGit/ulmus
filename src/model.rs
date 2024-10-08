use crate::{Command, Reactive};

/// The `Model` trait describes the behaviour of your TUI.
pub trait Model: Reactive {
	/// Returns [commands][`Cmd`] which will be ran on startup, before the
	/// first render.
	fn init(&self) -> Vec<Command>;

	fn view(&self) -> impl crossterm::Command;
}
