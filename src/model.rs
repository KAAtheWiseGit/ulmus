use std::{any::Any, sync::mpsc};

use crate::widget::Widget;

pub type Subroutine = Box<dyn FnOnce(mpsc::Sender<Message>) + Send>;

pub struct Message {
	value: Box<dyn Any + Send>,
}

impl Message {
	pub fn new<T: 'static + Send>(value: T) -> Self {
		Message {
			value: Box::new(value),
		}
	}

	pub fn empty() -> Self {
		Self::new(())
	}

	pub fn as_ref<T: 'static + Send>(&self) -> Option<&T> {
		self.value.downcast_ref()
	}

	pub fn downcast<T: 'static + Send>(self) -> Result<Box<T>, Self> {
		self.value.downcast::<T>().map_err(|value| Self { value })
	}

	pub fn is<T: 'static + Send>(&self) -> bool {
		self.value.is::<T>()
	}
}

/// Commands are returned by model in [`init`][Model#tymethod.init] and
/// [`update`][Model#tymethod.update] and can be used to change control the
/// event loop.
pub enum Command {
	SetCursor(u16, u16),
	/// Immediately shuts down the program.
	Quit,
	/// Launches a subroutine.  Note that this command can be sent at any
	/// time, even in [`update`][Model#tymethod.update] and that the
	/// subroutine can start sending messages immediately.
	Subroutine(Subroutine),
}

/// The `Model` trait describes the behaviour of your TUI.
pub trait Model {
	/// Returns [commands][`Cmd`] which will be ran on startup, before the
	/// first render.
	fn init(&self) -> Vec<Command>;

	fn update(&mut self, message: Message) -> Vec<Command>;

	fn view(&self) -> Box<dyn Widget>;
}
