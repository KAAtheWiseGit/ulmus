use crate::{Command, Reactive, View};

/// The `Model` trait describes the behaviour of your TUI.
pub trait Model: Reactive + View {
	/// Returns [commands][`Cmd`] which will be ran on startup, before the
	/// first render.
	fn init(&self) -> Vec<Command>;
}
