use std::{any::Any, sync::mpsc};

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
	Subroutine(Subroutine),
}

pub trait Reactive {
	fn update(&mut self, message: Message) -> Vec<Command>;
}
